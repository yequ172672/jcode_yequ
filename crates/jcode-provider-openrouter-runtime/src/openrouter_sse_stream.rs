use super::*;
use jcode_provider_openrouter::stream::OpenRouterStream;

fn local_endpoint_troubleshooting_hint(api_base: &str, model: &str) -> &'static str {
    let lower = api_base.to_ascii_lowercase();
    if lower.contains("localhost:11434") || lower.contains("127.0.0.1:11434") {
        return "Ollama hint: make sure `ollama serve` is running, the model is installed with `ollama pull <model>`, and run jcode with an installed model, for example `jcode --provider ollama --model llama3.2 run 'hello'`.";
    }

    if lower.contains("localhost:1234") || lower.contains("127.0.0.1:1234") {
        return "LM Studio hint: start the Local Server in LM Studio, load a chat model, and run jcode with the exact model id shown by LM Studio's /v1/models endpoint.";
    }

    if lower.contains("localhost") || lower.contains("127.0.0.1") || lower.contains("[::1]") {
        return "Local endpoint hint: make sure the server is running, the base URL includes /v1, the selected model is loaded, and the server supports streaming POST /chat/completions.";
    }

    let _ = model;
    "Hint: check network connectivity, DNS/TLS, that the base URL includes the API version (usually /v1), and that the model exists on the provider."
}

// ============================================================================
// SSE Stream Parser
// ============================================================================

#[expect(
    clippy::too_many_arguments,
    reason = "stream helpers thread transport, auth, request, event channel, and pin state explicitly"
)]
pub(super) async fn run_stream_with_retries(
    client: Client,
    api_base: String,
    auth: ProviderAuth,
    send_openrouter_headers: bool,
    mut request: Value,
    tx: mpsc::Sender<Result<StreamEvent>>,
    provider_pin: Arc<Mutex<Option<ProviderPin>>>,
    model: String,
    allow_generated_reasoning_fallback: bool,
    reasoning_fallback_ladder: &'static [&'static str],
    effective_reasoning_effort: Option<Arc<RwLock<Option<String>>>>,
) {
    let mut next_retry_delay = None;
    let mut transport_attempt = 1u32;
    let mut request_attempt = 0u32;
    let mut effort_fallbacks = 0usize;
    let mut delay_before_retry = false;

    loop {
        request_attempt += 1;
        if delay_before_retry {
            let delay = jcode_provider_core::retry_after::retry_delay(
                transport_attempt.saturating_sub(1),
                RETRY_BASE_DELAY_MS,
                next_retry_delay.take(),
            );
            tokio::time::sleep(delay).await;
            jcode_base::logging::info(&format!(
                "Retrying API request using {} (attempt {}/{})",
                auth.label(),
                transport_attempt,
                MAX_RETRIES
            ));
        }
        delay_before_retry = false;

        jcode_base::logging::info(&format!(
            "API stream request attempt {} over HTTPS transport (model: {}, endpoint: {}, auth: {})",
            request_attempt,
            model,
            api_base,
            auth.label()
        ));

        // Track whether this attempt streams replay-visible output so a
        // mid-stream transport fault can roll the partial output back on the
        // consumer before the retry replays the response from the top.
        let (attempt_tx, attempt_guard) =
            jcode_provider_core::attempt_tracker::track_attempt_output(tx.clone());

        // Retries use a fresh unpooled client: the fault that broke attempt N
        // (e.g. TLS BadRecordMac from a corrupting middlebox) may also have
        // poisoned other idle pooled connections opened through the same path,
        // so reusing the shared pool can fail identically. A fresh client
        // guarantees a brand-new TCP+TLS connection.
        let attempt_client = if request_attempt == 1 {
            client.clone()
        } else {
            jcode_provider_core::fresh_transport_client()
        };

        match stream_response(
            attempt_client,
            api_base.clone(),
            auth.clone(),
            send_openrouter_headers,
            request.clone(),
            attempt_tx,
            Arc::clone(&provider_pin),
            model.clone(),
        )
        .await
        {
            Ok(()) => {
                let _ = attempt_guard.finish().await;
                if let Some(state) = effective_reasoning_effort.as_ref() {
                    let effective = request_reasoning_effort(&request).map(str::to_string);
                    *state.write().await = effective;
                }
                return;
            }
            Err(e) => {
                let saw_output = attempt_guard.finish().await;
                // Full anyhow chain ({:#}) so a `.context(...)`-wrapped transport
                // cause (e.g. TLS BadRecordMac) is visible to the classifier.
                let error_str = format!("{e:#}").to_lowercase();
                if allow_generated_reasoning_fallback
                    && !saw_output
                    && effort_fallbacks < reasoning_fallback_ladder.len()
                    && let Some(fallback) = fallback_rejected_reasoning_request(
                        &mut request,
                        &error_str,
                        reasoning_fallback_ladder,
                    )
                {
                    effort_fallbacks += 1;
                    let _ = tx
                        .send(Ok(StreamEvent::StatusDetail {
                            detail: fallback.status_detail(&model),
                        }))
                        .await;
                    continue;
                }
                if is_retryable_error(&error_str) && transport_attempt < MAX_RETRIES {
                    transport_attempt += 1;
                    if saw_output {
                        // Partial output already reached the consumer; tell it
                        // to discard the partial attempt so the retried
                        // response replays cleanly instead of duplicating.
                        jcode_base::logging::warn(&format!(
                            "Transient API error after partial output; rolling back partial attempt and retrying: {}",
                            e
                        ));
                        let _ = tx
                            .send(Ok(StreamEvent::RetryRollback {
                                attempt: transport_attempt,
                                max: MAX_RETRIES,
                            }))
                            .await;
                    } else {
                        jcode_base::logging::info(&format!(
                            "Transient API error, will retry: {}",
                            e
                        ));
                    }
                    next_retry_delay = jcode_provider_core::retry_after::retry_after_from_error(&e);
                    delay_before_retry = true;
                    continue;
                }

                let _ = tx.send(Err(e)).await;
                return;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ReasoningFallback {
    Downgraded { from: String, to: String },
    Removed { from: String },
}

impl ReasoningFallback {
    fn status_detail(&self, model: &str) -> String {
        match self {
            Self::Downgraded { from, to } => {
                format!("Reasoning effort fallback for {model}: {from} → {to}")
            }
            Self::Removed { from } => format!(
                "Reasoning effort '{from}' is not accepted by {model}; using provider default"
            ),
        }
    }
}

fn request_reasoning_effort(request: &Value) -> Option<&str> {
    request
        .get("reasoning_effort")
        .and_then(Value::as_str)
        .or_else(|| {
            request
                .get("reasoning")
                .and_then(|value| value.get("effort"))
                .and_then(Value::as_str)
        })
}

fn fallback_rejected_reasoning_request(
    request: &mut Value,
    error: &str,
    supported_efforts: &[&str],
) -> Option<ReasoningFallback> {
    let status = parsed_http_status(error)?;
    if !matches!(status, 400 | 422)
        || !(error.contains("reasoning_effort") || error.contains("reasoning"))
    {
        return None;
    }

    let field_is_rejected = [
        "unknown field",
        "unknown parameter",
        "unrecognized",
        "unexpected field",
        "extra inputs",
        "not permitted",
        "unsupported parameter",
    ]
    .iter()
    .any(|marker| error.contains(marker));
    let value_is_rejected = field_is_rejected
        || [
            "invalid value",
            "unsupported value",
            "not supported",
            "must be one of",
            "allowed values",
        ]
        .iter()
        .any(|marker| error.contains(marker));
    if !value_is_rejected {
        return None;
    }

    let current = request_reasoning_effort(request)?.to_string();
    if !field_is_rejected
        && let Some(parsed) = jcode_provider_core::parse_reasoning_effort(&current)
        && let Some(next) = supported_efforts.iter().rev().copied().find(|candidate| {
            jcode_provider_core::parse_reasoning_effort(candidate)
                .is_some_and(|candidate| candidate < parsed && candidate.as_str() != "none")
        })
    {
        if request.get("reasoning_effort").is_some() {
            request["reasoning_effort"] = serde_json::json!(next);
        } else if request.get("reasoning").is_some() {
            request["reasoning"]["effort"] = serde_json::json!(next);
        }
        return Some(ReasoningFallback::Downgraded {
            from: current,
            to: next.to_string(),
        });
    }

    if let Some(object) = request.as_object_mut() {
        object.remove("reasoning_effort");
        object.remove("reasoning");
    }
    Some(ReasoningFallback::Removed { from: current })
}

#[expect(
    clippy::too_many_arguments,
    reason = "stream helpers thread transport, auth, request, event channel, and pin state explicitly"
)]
async fn stream_response(
    client: Client,
    api_base: String,
    auth: ProviderAuth,
    send_openrouter_headers: bool,
    request: Value,
    tx: mpsc::Sender<Result<StreamEvent>>,
    provider_pin: Arc<Mutex<Option<ProviderPin>>>,
    model: String,
) -> Result<()> {
    use jcode_message_types::ConnectionPhase;
    let _ = tx
        .send(Ok(StreamEvent::ConnectionPhase {
            phase: ConnectionPhase::SendingRequest,
        }))
        .await;
    let connect_start = std::time::Instant::now();

    let url = format!("{}/chat/completions", api_base);
    let mut req = apply_kimi_coding_agent_headers(
        auth.apply(
            client
                .post(&url)
                .header("Content-Type", "application/json")
                .header("Accept-Encoding", "identity"),
        )
        .await?,
        &api_base,
        Some(&model),
    );

    if send_openrouter_headers {
        req = req
            .header("HTTP-Referer", "https://github.com/jcode")
            .header("X-Title", "jcode");
    }

    let response = req
        .json(&request)
        .send()
        .await
        .with_context(|| {
            let hint = local_endpoint_troubleshooting_hint(&api_base, &model);
            format!(
                "Failed to send OpenAI-compatible chat request\n  endpoint: {}\n  model: {}\n  auth: {}\n{}",
                url,
                model,
                auth.label(),
                hint
            )
        })?;

    let connect_ms = connect_start.elapsed().as_millis();
    jcode_base::logging::info(&format!(
        "HTTP connection established in {}ms (status={})",
        connect_ms,
        response.status()
    ));

    if !response.status().is_success() {
        let status = response.status();
        let retry_after = jcode_provider_core::retry_after::retry_after(response.headers());
        let body = jcode_base::util::http_error_body(response, "HTTP error").await;
        let hint = local_endpoint_troubleshooting_hint(&api_base, &model);
        return Err(jcode_provider_core::retry_after::error_with_retry_after(
            format!(
                "OpenAI-compatible chat request failed\n  endpoint: {}\n  model: {}\n  auth: {}\n  status: {}\n  response: {}\n{}",
                url,
                model,
                auth.label(),
                status,
                body,
                hint
            ),
            retry_after,
        ));
    }

    let _ = tx
        .send(Ok(StreamEvent::ConnectionPhase {
            phase: ConnectionPhase::WaitingForResponse,
        }))
        .await;

    let mut stream = OpenRouterStream::new(response.bytes_stream(), model.clone(), provider_pin);

    // Idle timeout between streamed chunks. Configurable so slow reasoning
    // models (e.g. DeepSeek) that think silently for minutes before emitting
    // tokens don't trip a premature timeout (issue #196). Resolved from
    // `[provider] stream_idle_timeout_secs` / `JCODE_STREAM_IDLE_TIMEOUT_SECS`,
    // defaulting to 180s. Shared with the native provider paths (issue #434).
    let sse_chunk_timeout = jcode_base::provider::stream_idle_timeout();
    let idle_timeout_secs = sse_chunk_timeout.as_secs();

    loop {
        let event = match tokio::time::timeout(sse_chunk_timeout, stream.next()).await {
            Ok(Some(Ok(event))) => event,
            Ok(Some(Err(e))) => anyhow::bail!(
                "OpenAI-compatible stream error\n  endpoint: {}\n  model: {}\n  auth: {}\n  error: {}",
                url,
                model,
                auth.label(),
                e
            ),
            Ok(None) => break, // stream ended normally
            Err(_) => {
                jcode_base::logging::warn(&format!(
                    "OpenRouter SSE stream timed out (no data for {}s)",
                    idle_timeout_secs
                ));
                anyhow::bail!(
                    "OpenAI-compatible stream timeout\n  endpoint: {}\n  model: {}\n  auth: {}\n  timeout: no data received for {} seconds\n{}",
                    url,
                    model,
                    auth.label(),
                    idle_timeout_secs,
                    local_endpoint_troubleshooting_hint(&api_base, &model)
                );
            }
        };
        if tx.send(Ok(event)).await.is_err() {
            return Ok(());
        }
    }

    Ok(())
}

/// Extract the HTTP status code reported in a formatted provider error string.
///
/// Error strings produced in this module embed the status as `status: <code>`
/// (e.g. `status: 402 Payment Required`). The input may be lowercased before
/// it reaches here, so matching is case-insensitive.
fn parsed_http_status(error_str: &str) -> Option<u16> {
    let lower = error_str.to_ascii_lowercase();
    let idx = lower.find("status:")?;
    let rest = lower[idx + "status:".len()..].trim_start();
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.len() == 3 {
        digits.parse().ok()
    } else {
        None
    }
}

fn is_retryable_error(error_str: &str) -> bool {
    // Explicit non-retryable HTTP statuses take precedence over the loose
    // substring heuristics below. These are deterministic client-side failures
    // (auth, billing, malformed request) where retrying is futile and just
    // burns time/credits. 429 (rate limit) is classified explicitly so it does
    // not depend on provider-specific body wording.
    match parsed_http_status(error_str) {
        Some(400 | 401 | 402 | 403 | 404 | 405 | 406 | 422) => return false,
        Some(429) => return true,
        _ => {}
    }

    jcode_provider_core::is_transient_transport_error(error_str)
        || error_str.contains("stream error")
        || error_str.contains("eof")
        || error_str.contains("5")
            && (error_str.contains("50")
                || error_str.contains("502")
                || error_str.contains("503")
                || error_str.contains("504")
                || error_str.contains("internal server error"))
        || error_str.contains("overloaded")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_endpoint_hint_mentions_ollama_actions() {
        let hint = local_endpoint_troubleshooting_hint("http://localhost:11434/v1", "llama3.2");
        assert!(hint.contains("ollama serve"));
        assert!(hint.contains("ollama pull"));
        assert!(hint.contains("--provider ollama"));
    }

    #[test]
    fn local_endpoint_hint_mentions_lm_studio_server() {
        let hint = local_endpoint_troubleshooting_hint("http://127.0.0.1:1234/v1", "local-model");
        assert!(hint.contains("LM Studio"));
        assert!(hint.contains("Local Server"));
        assert!(hint.contains("/v1/models"));
    }

    #[test]
    fn parsed_http_status_extracts_code() {
        assert_eq!(
            parsed_http_status("status: 402 payment required"),
            Some(402)
        );
        assert_eq!(parsed_http_status("  status:404 not found"), Some(404));
        assert_eq!(parsed_http_status("no status here"), None);
        // Embedded numbers elsewhere must not be misread as a status.
        assert_eq!(parsed_http_status("you requested 65536 tokens"), None);
    }

    #[test]
    fn payment_required_is_not_retryable() {
        let err = "openai-compatible chat request failed\n  endpoint: \
            https://openrouter.ai/api/v1/chat/completions\n  model: openai/gpt-5.4\n  \
            auth: openrouter_api_key\n  status: 402 payment required\n  response: \
            {\"error\":{\"message\":\"this request requires more credits, or fewer \
            max_tokens. you requested up to 65536 tokens, but can only afford 34424\"}}";
        assert!(!is_retryable_error(err));
    }

    #[test]
    fn client_errors_are_not_retryable() {
        for status in [400u16, 401, 402, 403, 404, 405, 406, 422] {
            let err = format!("chat request failed\n  status: {status} client error");
            assert!(
                !is_retryable_error(&err),
                "status {status} should not be retryable"
            );
        }
    }

    #[test]
    fn server_errors_remain_retryable() {
        assert!(is_retryable_error(
            "chat request failed\n  status: 503 service unavailable"
        ));
        assert!(is_retryable_error(
            "chat request failed\n  status: 500 internal server error"
        ));
        // Provider overload messages should still be retried.
        assert!(is_retryable_error("overloaded"));
    }

    #[test]
    fn http_429_is_retryable_without_rate_limit_words_in_body() {
        assert!(is_retryable_error(
            "chat request failed\n  status: 429 unknown\n  response: {}"
        ));
    }

    #[test]
    fn rejected_reasoning_value_steps_down_without_touching_other_fields() {
        let mut request = serde_json::json!({
            "model": "gpt-test",
            "reasoning_effort": "max",
            "messages": [{"role": "user", "content": "hi"}],
        });

        let fallback = fallback_rejected_reasoning_request(
            &mut request,
            "status: 400 bad request: reasoning_effort has an unsupported value",
            jcode_provider_core::OPENAI_SELECTABLE_EFFORTS,
        );

        assert_eq!(
            fallback,
            Some(ReasoningFallback::Downgraded {
                from: "max".to_string(),
                to: "xhigh".to_string(),
            })
        );
        assert_eq!(request["reasoning_effort"], "xhigh");
        assert_eq!(request["model"], "gpt-test");
        assert!(request.get("messages").is_some());
    }

    #[test]
    fn rejected_reasoning_field_is_removed_for_provider_default() {
        let mut request = serde_json::json!({
            "model": "custom-model",
            "reasoning_effort": "high",
        });

        let fallback = fallback_rejected_reasoning_request(
            &mut request,
            "status: 422 unprocessable entity: unknown parameter reasoning_effort",
            jcode_provider_core::OPENAI_SELECTABLE_EFFORTS,
        );

        assert_eq!(
            fallback,
            Some(ReasoningFallback::Removed {
                from: "high".to_string(),
            })
        );
        assert!(request.get("reasoning_effort").is_none());
        assert_eq!(request["model"], "custom-model");
    }

    #[test]
    fn unrelated_client_errors_do_not_trigger_reasoning_fallback() {
        let mut request = serde_json::json!({
            "model": "gpt-test",
            "reasoning_effort": "high",
        });

        assert_eq!(
            fallback_rejected_reasoning_request(
                &mut request,
                "status: 400 bad request: messages are required",
                jcode_provider_core::OPENAI_SELECTABLE_EFFORTS,
            ),
            None
        );
        assert_eq!(request["reasoning_effort"], "high");
    }

    #[test]
    fn deepseek_fallback_uses_only_deepseek_effort_levels() {
        let mut request = serde_json::json!({
            "model": "deepseek-test",
            "reasoning_effort": "low",
        });

        let fallback = fallback_rejected_reasoning_request(
            &mut request,
            "status: 400 bad request: reasoning_effort has an unsupported value",
            jcode_provider_core::DEEPSEEK_SELECTABLE_EFFORTS,
        );

        assert_eq!(
            fallback,
            Some(ReasoningFallback::Removed {
                from: "low".to_string(),
            })
        );
        assert!(request.get("reasoning_effort").is_none());
    }
}
