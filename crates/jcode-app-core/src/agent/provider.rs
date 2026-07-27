use super::*;

fn normalize_reasoning_preference(effort: &str) -> anyhow::Result<String> {
    let normalized = effort.trim().to_ascii_lowercase();
    if let Some(canonical) = jcode_provider_core::canonical_reasoning_effort(&normalized) {
        Ok(canonical.to_string())
    } else if crate::prompt::is_swarm_effort(&normalized) {
        Ok(normalized)
    } else {
        anyhow::bail!(
            "Unsupported reasoning effort '{}'; expected none|minimal|low|medium|high|xhigh|max|swarm|swarm-deep",
            effort
        )
    }
}

impl Agent {
    pub fn set_premium_mode(&self, mode: crate::provider::copilot::PremiumMode) {
        self.provider.set_premium_mode(mode);
    }

    pub fn premium_mode(&self) -> crate::provider::copilot::PremiumMode {
        self.provider.premium_mode()
    }

    pub fn provider_fork(&self) -> Arc<dyn Provider> {
        self.provider.fork()
    }

    pub fn provider_handle(&self) -> Arc<dyn Provider> {
        Arc::clone(&self.provider)
    }

    pub fn available_models(&self) -> Vec<&'static str> {
        self.provider.available_models()
    }

    pub fn available_models_for_switching(&self) -> Vec<String> {
        self.provider.available_models_for_switching()
    }

    pub fn available_models_display(&self) -> Vec<String> {
        self.provider.available_models_display()
    }

    pub fn model_routes(&self) -> Vec<crate::provider::ModelRoute> {
        self.provider.model_routes()
    }

    pub fn model_catalog_snapshot(&self) -> jcode_provider_core::ModelCatalogSnapshot {
        jcode_provider_core::ModelCatalogSnapshot::new(
            Some(self.provider_name()),
            Some(self.provider_model()),
            self.available_models_display(),
            self.model_routes(),
        )
    }

    pub fn registry(&self) -> Registry {
        self.registry.clone()
    }

    pub async fn compaction_mode(&self) -> crate::config::CompactionMode {
        self.registry.compaction().read().await.mode()
    }

    pub async fn set_compaction_mode(&self, mode: crate::config::CompactionMode) -> Result<()> {
        let compaction = self.registry.compaction();
        let mut manager = compaction.write().await;
        manager.set_mode(mode);
        Ok(())
    }

    pub fn provider_messages(&mut self) -> Vec<Message> {
        self.session.messages_for_provider()
    }

    pub fn set_model(&mut self, model: &str) -> Result<()> {
        self.set_model_from_provider_state_event(
            model,
            crate::provider::ProviderModelSelectionSource::User,
        )
    }

    pub fn set_route_selection(
        &mut self,
        selection: &crate::provider::RouteSelection,
    ) -> Result<()> {
        self.set_route_selection_from_provider_state_event(
            selection,
            crate::provider::ProviderModelSelectionSource::User,
        )
    }

    pub(crate) fn set_route_selection_from_auth(
        &mut self,
        selection: &crate::provider::RouteSelection,
    ) -> Result<()> {
        self.set_route_selection_from_provider_state_event(
            selection,
            crate::provider::ProviderModelSelectionSource::Auth,
        )
    }

    fn set_route_selection_from_provider_state_event(
        &mut self,
        selection: &crate::provider::RouteSelection,
        source: crate::provider::ProviderModelSelectionSource,
    ) -> Result<()> {
        self.provider.set_route_selection(selection)?;
        let resolved_model = self.provider.model();
        self.session.provider_key = Some(selection.runtime_key.stable_id());
        self.session.route_api_method = Some(selection.api_method.clone());
        self.session.model = Some(resolved_model.clone());
        self.reapply_reasoning_effort_preference();
        let event = crate::provider::ProviderStateEvent::selected_model(source, resolved_model);
        self.provider_runtime_state.apply(event);
        self.persist_session_best_effort("route selection");
        self.log_env_snapshot("set_route_selection");
        Ok(())
    }

    pub(crate) fn set_model_from_auth(&mut self, model: &str) -> Result<()> {
        self.set_model_from_provider_state_event(
            model,
            crate::provider::ProviderModelSelectionSource::Auth,
        )
    }

    fn set_model_from_provider_state_event(
        &mut self,
        model: &str,
        source: crate::provider::ProviderModelSelectionSource,
    ) -> Result<()> {
        crate::provider::set_model_with_auth_refresh(self.provider.as_ref(), model)?;
        let resolved_model = self.provider.model();
        self.session.provider_key =
            crate::provider::MultiProvider::session_provider_key_after_model_switch(
                model,
                self.provider.name(),
                self.session.provider_key.as_deref(),
            );
        self.session.model = Some(resolved_model.clone());
        self.reapply_reasoning_effort_preference();
        let event = crate::provider::ProviderStateEvent::selected_model(source, resolved_model);
        self.provider_runtime_state.apply(event);
        self.persist_session_best_effort("model selection");
        self.log_env_snapshot("set_model");
        Ok(())
    }

    pub(crate) fn provider_model_selection_generation(&self) -> u64 {
        self.provider_runtime_state.selection_generation()
    }

    pub(crate) fn user_selected_provider_model_after(&self, generation: u64) -> bool {
        self.provider_runtime_state.user_selected_after(generation)
    }

    pub fn restore_reasoning_effort_from_session(&mut self) {
        if self.session.reasoning_effort.is_some() {
            self.reapply_reasoning_effort_preference();
        } else {
            self.session.reasoning_effort = self.provider.reasoning_effort();
        }
        // Mirror the effort into the deadlock-free side-table so server handlers
        // (e.g. the swarm seed handler) can learn this session's effort without
        // taking the agent lock.
        let effective = self.provider.reasoning_effort();
        let recorded = self
            .session
            .reasoning_effort
            .as_deref()
            .filter(|effort| crate::prompt::is_swarm_effort(effort))
            .or(effective.as_deref());
        crate::session_effort::record_session_effort(&self.session.id, recorded);
    }

    pub fn set_reasoning_effort(&mut self, effort: &str) -> Result<Option<String>> {
        let requested = normalize_reasoning_preference(effort)?;

        // `Session::reasoning_effort` is the durable user preference. The
        // provider owns the effective value after model-specific resolution.
        // Keeping these separate lets `max` temporarily resolve to `high` and
        // automatically return to `max` when the user switches to a stronger
        // model later.
        let advertised = self.provider.available_efforts();
        if let Err(error) = self.provider.set_reasoning_effort(&requested) {
            if !advertised.is_empty() {
                return Err(error);
            }
            crate::logging::info(&format!(
                "Reasoning effort preference '{}' is durable but unapplied for model '{}': {}",
                requested,
                self.provider.model(),
                error
            ));
        }
        self.session.reasoning_effort = Some(requested.clone());
        let current = self.provider.reasoning_effort();
        // Keep the side-table in sync (see `restore_reasoning_effort_from_session`).
        let recorded = crate::prompt::is_swarm_effort(&requested)
            .then_some(requested.as_str())
            .or(current.as_deref());
        crate::session_effort::record_session_effort(&self.session.id, recorded);
        self.log_env_snapshot("set_reasoning_effort");
        self.session.save()?;
        Ok(current)
    }

    fn reapply_reasoning_effort_preference(&self) {
        let Some(preference) = self.session.reasoning_effort.as_deref() else {
            return;
        };
        let advertised = self.provider.available_efforts();
        if let Err(error) = self.provider.set_reasoning_effort(preference) {
            if !advertised.is_empty() {
                crate::logging::warn(&format!(
                    "Could not apply reasoning effort preference '{}' to model '{}': {}",
                    preference,
                    self.provider.model(),
                    error
                ));
            }
        }
        let effective = self.provider.reasoning_effort();
        let recorded = crate::prompt::is_swarm_effort(preference)
            .then_some(preference)
            .or(effective.as_deref());
        crate::session_effort::record_session_effort(&self.session.id, recorded);
    }

    pub fn reasoning_effort_preference(&self) -> Option<String> {
        self.session.reasoning_effort.clone()
    }

    pub fn subagent_model(&self) -> Option<String> {
        self.session.subagent_model.clone()
    }

    pub fn set_subagent_model(&mut self, model: Option<String>) -> Result<()> {
        self.session.subagent_model = model;
        self.log_env_snapshot("set_subagent_model");
        self.session.save()?;
        Ok(())
    }

    pub fn session_provider_key(&self) -> Option<String> {
        self.session.provider_key.clone()
    }

    /// API method/runtime route used to select the active model (e.g.
    /// "openai-api", "claude-oauth", "openai-compatible:nvidia-nim"). Spawned
    /// swarm agents inherit this so they reconstruct the coordinator's exact
    /// auth route instead of falling back to the config default.
    pub fn session_route_api_method(&self) -> Option<String> {
        self.session.route_api_method.clone()
    }

    /// The credential the active provider will use for the next request, when
    /// the provider distinguishes OAuth (subscription) from API key (cost).
    /// Resolved authoritatively here so remote clients can render billing/usage
    /// without re-deriving it from the provider name.
    pub fn active_resolved_credential(&self) -> Option<jcode_provider_core::ResolvedCredential> {
        self.provider.active_resolved_credential()
    }

    pub fn set_session_provider_key(&mut self, provider_key: Option<String>) {
        self.session.provider_key = provider_key;
    }

    pub fn rename_session_title(&mut self, title: Option<String>) -> Result<String> {
        self.session.rename_title(title);
        self.log_env_snapshot("rename_session");
        self.session.save()?;
        Ok(self.session.display_title_or_name().to_string())
    }

    pub fn autoreview_enabled(&self) -> Option<bool> {
        self.session.autoreview_enabled
    }

    pub fn set_autoreview_enabled(&mut self, enabled: bool) -> Result<()> {
        self.session.autoreview_enabled = Some(enabled);
        self.log_env_snapshot("set_autoreview_enabled");
        self.session.save()?;
        Ok(())
    }

    pub fn autojudge_enabled(&self) -> Option<bool> {
        self.session.autojudge_enabled
    }

    pub fn set_autojudge_enabled(&mut self, enabled: bool) -> Result<()> {
        self.session.autojudge_enabled = Some(enabled);
        self.log_env_snapshot("set_autojudge_enabled");
        self.session.save()?;
        Ok(())
    }

    /// Set the working directory for this session
    pub fn set_working_dir(&mut self, dir: &str) {
        if self.session.working_dir.as_deref() == Some(dir) {
            return;
        }
        self.session.working_dir = Some(dir.to_string());
        self.session.refresh_initial_session_context_message();
        self.log_env_snapshot("working_dir");
    }

    /// Get the working directory for this session
    pub fn working_dir(&self) -> Option<&str> {
        self.session.working_dir.as_deref()
    }

    /// Get the stored messages (for transcript export)
    pub fn messages(&self) -> &[StoredMessage] {
        &self.session.messages
    }
}

#[cfg(test)]
mod reasoning_preference_tests {
    use super::normalize_reasoning_preference;

    #[test]
    fn aliases_are_canonicalized_before_session_persistence() {
        assert_eq!(normalize_reasoning_preference(" MIN ").unwrap(), "minimal");
        assert_eq!(normalize_reasoning_preference("off").unwrap(), "none");
        assert_eq!(normalize_reasoning_preference("maximum").unwrap(), "max");
        assert_eq!(
            normalize_reasoning_preference("swarm-deep").unwrap(),
            "swarm-deep"
        );
        assert!(normalize_reasoning_preference("turbo").is_err());
    }
}
