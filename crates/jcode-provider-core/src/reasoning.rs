//! Shared reasoning-effort ladders.
//!
//! Keep these in provider-core so provider runtimes and UI clients expose the
//! same ordered values. The canonical wire-level ladder is intentionally kept
//! separate from Jcode UI sentinels such as `swarm` and `swarm-deep`.

use serde::{Deserialize, Serialize};

/// Canonical provider wire-level reasoning effort values in ascending strength.
pub const WIRE_REASONING_EFFORTS: &[&str] =
    &["none", "minimal", "low", "medium", "high", "xhigh", "max"];

/// A canonical provider wire-level reasoning effort.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    None,
    Minimal,
    Low,
    Medium,
    High,
    #[serde(rename = "xhigh")]
    XHigh,
    Max,
}

impl ReasoningEffort {
    /// Canonical wire spelling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Minimal => "minimal",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::XHigh => "xhigh",
            Self::Max => "max",
        }
    }

    const fn rank(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Minimal => 1,
            Self::Low => 2,
            Self::Medium => 3,
            Self::High => 4,
            Self::XHigh => 5,
            Self::Max => 6,
        }
    }

    const fn is_none(self) -> bool {
        matches!(self, Self::None)
    }
}

/// Parse a provider/UI spelling into the canonical wire-level effort.
///
/// Accepted aliases are deliberately small and deterministic. Jcode UI sentinels
/// such as `swarm` and `swarm-deep` are not wire-level efforts and return `None`.
pub fn parse_reasoning_effort(value: &str) -> Option<ReasoningEffort> {
    match value.trim().to_ascii_lowercase().as_str() {
        "none" | "off" | "disabled" => Some(ReasoningEffort::None),
        "minimal" | "min" => Some(ReasoningEffort::Minimal),
        "low" => Some(ReasoningEffort::Low),
        "medium" | "med" => Some(ReasoningEffort::Medium),
        "high" => Some(ReasoningEffort::High),
        "xhigh" | "x-high" | "extra-high" | "extra_high" => Some(ReasoningEffort::XHigh),
        "max" | "maximum" => Some(ReasoningEffort::Max),
        _ => None,
    }
}

/// A provider's advertised wire-level reasoning capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningCapability {
    /// Canonical supported wire-level efforts, sorted weakest to strongest and
    /// deduplicated by [`ReasoningCapability::new`]. Empty means no support was
    /// advertised and must not be treated as an implicit full ladder.
    pub efforts: Vec<ReasoningEffort>,
}

impl ReasoningCapability {
    pub fn new<I>(efforts: I) -> Self
    where
        I: IntoIterator<Item = ReasoningEffort>,
    {
        let mut efforts: Vec<_> = efforts.into_iter().collect();
        efforts.sort();
        efforts.dedup();
        Self { efforts }
    }

    /// Build a capability from provider-advertised strings.
    ///
    /// Unknown values are ignored. If no known values remain, the capability is
    /// empty and resolution reports `Unapplied` rather than inventing support.
    pub fn from_values<I, S>(values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Self::new(
            values
                .into_iter()
                .filter_map(|v| parse_reasoning_effort(v.as_ref())),
        )
    }

    pub fn is_empty(&self) -> bool {
        self.efforts.is_empty()
    }
}

/// Why a requested effort was not applied exactly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningResolutionReason {
    Exact,
    DowngradedToStrongestSupportedAtOrBelowRequest,
    UpgradedToLowestSupportedNonNone,
    NoneUnsupportedUsingLowestSupportedNonNone,
    UnknownRequestedEffort,
    CapabilityUnavailable,
}

/// Deterministic resolution of a requested wire-level effort against capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningResolution {
    Applied {
        requested: ReasoningEffort,
        resolved: ReasoningEffort,
        reason: ReasoningResolutionReason,
    },
    Unapplied {
        requested: Option<String>,
        reason: ReasoningResolutionReason,
    },
}

/// Resolve a requested effort against known provider capability.
///
/// Rules:
/// - exact supported match wins;
/// - non-`none` requests choose the strongest supported non-`none` effort at or
///   below the request;
/// - if no non-`none` effort exists at or below the request, choose the lowest
///   supported non-`none` effort;
/// - `none` stays `none` if supported;
/// - unsupported `none` falls back to the lowest supported non-`none` effort;
/// - unknown requests and empty/unknown capabilities are unapplied.
pub fn resolve_reasoning_effort(
    requested: impl AsRef<str>,
    capability: &ReasoningCapability,
) -> ReasoningResolution {
    let requested_raw = requested.as_ref().trim();
    let Some(requested) = parse_reasoning_effort(requested_raw) else {
        return ReasoningResolution::Unapplied {
            requested: if requested_raw.is_empty() {
                None
            } else {
                Some(requested_raw.to_string())
            },
            reason: ReasoningResolutionReason::UnknownRequestedEffort,
        };
    };

    if capability.is_empty() {
        return ReasoningResolution::Unapplied {
            requested: Some(requested.as_str().to_string()),
            reason: ReasoningResolutionReason::CapabilityUnavailable,
        };
    }

    if capability.efforts.contains(&requested) {
        return ReasoningResolution::Applied {
            requested,
            resolved: requested,
            reason: ReasoningResolutionReason::Exact,
        };
    }

    let lowest_non_none = capability
        .efforts
        .iter()
        .copied()
        .find(|effort| !effort.is_none());

    if requested.is_none() {
        return match lowest_non_none {
            Some(resolved) => ReasoningResolution::Applied {
                requested,
                resolved,
                reason: ReasoningResolutionReason::NoneUnsupportedUsingLowestSupportedNonNone,
            },
            None => ReasoningResolution::Unapplied {
                requested: Some(requested.as_str().to_string()),
                reason: ReasoningResolutionReason::CapabilityUnavailable,
            },
        };
    }

    if let Some(resolved) = capability
        .efforts
        .iter()
        .copied()
        .rfind(|effort| !effort.is_none() && effort.rank() <= requested.rank())
    {
        return ReasoningResolution::Applied {
            requested,
            resolved,
            reason: ReasoningResolutionReason::DowngradedToStrongestSupportedAtOrBelowRequest,
        };
    }

    match lowest_non_none {
        Some(resolved) => ReasoningResolution::Applied {
            requested,
            resolved,
            reason: ReasoningResolutionReason::UpgradedToLowestSupportedNonNone,
        },
        None => ReasoningResolution::Unapplied {
            requested: Some(requested.as_str().to_string()),
            reason: ReasoningResolutionReason::CapabilityUnavailable,
        },
    }
}

/// OpenAI Responses API effort levels, followed by Jcode's swarm modes.
pub const OPENAI_SELECTABLE_EFFORTS: &[&str] = &[
    "none",
    "minimal",
    "low",
    "medium",
    "high",
    "xhigh",
    "max",
    "swarm",
    "swarm-deep",
];

/// OpenRouter's unified reasoning effort levels.
///
/// OpenRouter currently treats `max` as an alias for `xhigh`, so it is not a
/// separate rung in this ladder.
pub const OPENROUTER_SELECTABLE_EFFORTS: &[&str] = &[
    "none",
    "minimal",
    "low",
    "medium",
    "high",
    "xhigh",
    "swarm",
    "swarm-deep",
];

/// Direct DeepSeek effort levels, followed by Jcode's swarm modes.
pub const DEEPSEEK_SELECTABLE_EFFORTS: &[&str] = &[
    "none",
    "low",
    "medium",
    "high",
    "max",
    "swarm",
    "swarm-deep",
];

/// Convert a provider-advertised OpenAI/OpenRouter effort into the canonical
/// static value used by the provider trait.
pub fn canonical_reasoning_effort(value: &str) -> Option<&'static str> {
    parse_reasoning_effort(value).map(ReasoningEffort::as_str)
}

/// Infer the selectable effort ladder when only provider/model identity is
/// available, such as in a remote TUI session.
///
/// This is a user-preference ladder, not a claim that every value is accepted
/// verbatim by the provider. The active runtime resolves the preference against
/// its concrete model capability before serializing a request. Keeping the
/// picker universal lets a preference survive route/model switches and enables
/// automatic downgrade instead of hiding the control behind provider-name
/// heuristics.
pub fn inferred_reasoning_efforts(
    provider_name: Option<&str>,
    model_name: Option<&str>,
) -> Vec<&'static str> {
    let has_identity = provider_name.is_some_and(|value| !value.trim().is_empty())
        || model_name.is_some_and(|value| !value.trim().is_empty());
    if has_identity {
        OPENAI_SELECTABLE_EFFORTS.to_vec()
    } else {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_ladders_preserve_distinct_max_semantics() {
        assert_eq!(
            inferred_reasoning_efforts(Some("openai"), Some("gpt-5.4")),
            OPENAI_SELECTABLE_EFFORTS
        );
        assert!(OPENAI_SELECTABLE_EFFORTS.contains(&"max"));
        assert!(OPENAI_SELECTABLE_EFFORTS.contains(&"minimal"));
        assert!(OPENROUTER_SELECTABLE_EFFORTS.contains(&"minimal"));
        assert!(!OPENROUTER_SELECTABLE_EFFORTS.contains(&"max"));
        assert!(DEEPSEEK_SELECTABLE_EFFORTS.contains(&"max"));
        assert_eq!(
            inferred_reasoning_efforts(Some("openai-compatible:custom"), Some("gpt-5.6")),
            OPENAI_SELECTABLE_EFFORTS,
            "direct OpenAI-compatible runtimes use the OpenAI reasoning_effort vocabulary"
        );
    }

    #[test]
    fn inferred_ladder_is_universal_across_provider_identities() {
        assert_eq!(
            inferred_reasoning_efforts(Some("anthropic"), Some("claude-sonnet-4-6")),
            OPENAI_SELECTABLE_EFFORTS
        );
        assert_eq!(
            inferred_reasoning_efforts(Some("opencode-go"), Some("provider/model")),
            OPENAI_SELECTABLE_EFFORTS
        );
        assert_eq!(
            inferred_reasoning_efforts(Some("custom-provider"), Some("custom-model")),
            OPENAI_SELECTABLE_EFFORTS
        );
        assert!(inferred_reasoning_efforts(None, None).is_empty());
    }

    #[test]
    fn wire_ladder_is_canonical_and_separate_from_swarm_modes() {
        assert_eq!(
            WIRE_REASONING_EFFORTS,
            &["none", "minimal", "low", "medium", "high", "xhigh", "max"]
        );
        assert!(!WIRE_REASONING_EFFORTS.contains(&"swarm"));
        assert!(!WIRE_REASONING_EFFORTS.contains(&"swarm-deep"));
    }

    #[test]
    fn parses_aliases_to_canonical_wire_values() {
        assert_eq!(
            parse_reasoning_effort(" x-high "),
            Some(ReasoningEffort::XHigh)
        );
        assert_eq!(
            parse_reasoning_effort("extra_high"),
            Some(ReasoningEffort::XHigh)
        );
        assert_eq!(
            parse_reasoning_effort("maximum"),
            Some(ReasoningEffort::Max)
        );
        assert_eq!(canonical_reasoning_effort("MIN"), Some("minimal"));
        assert_eq!(canonical_reasoning_effort("swarm"), None);
        assert_eq!(canonical_reasoning_effort(""), None);
    }

    #[test]
    fn capability_from_values_dedupes_sorts_and_ignores_unknowns() {
        assert_eq!(
            ReasoningCapability::from_values(["high", "bogus", "low", "high"]),
            ReasoningCapability::new([ReasoningEffort::Low, ReasoningEffort::High])
        );
        assert!(ReasoningCapability::from_values(["bogus", "swarm"]).is_empty());
    }

    #[test]
    fn resolver_uses_exact_match_first() {
        let caps = ReasoningCapability::from_values(["none", "low", "high"]);
        assert_eq!(
            resolve_reasoning_effort("low", &caps),
            ReasoningResolution::Applied {
                requested: ReasoningEffort::Low,
                resolved: ReasoningEffort::Low,
                reason: ReasoningResolutionReason::Exact,
            }
        );
    }

    #[test]
    fn resolver_downgrades_to_strongest_supported_non_none_at_or_below_request() {
        let caps = ReasoningCapability::from_values(["none", "low", "high"]);
        assert_eq!(
            resolve_reasoning_effort("xhigh", &caps),
            ReasoningResolution::Applied {
                requested: ReasoningEffort::XHigh,
                resolved: ReasoningEffort::High,
                reason: ReasoningResolutionReason::DowngradedToStrongestSupportedAtOrBelowRequest,
            }
        );
    }

    #[test]
    fn resolver_upgrades_non_none_to_lowest_supported_non_none_when_needed() {
        let caps = ReasoningCapability::from_values(["none", "medium", "max"]);
        assert_eq!(
            resolve_reasoning_effort("minimal", &caps),
            ReasoningResolution::Applied {
                requested: ReasoningEffort::Minimal,
                resolved: ReasoningEffort::Medium,
                reason: ReasoningResolutionReason::UpgradedToLowestSupportedNonNone,
            }
        );
    }

    #[test]
    fn resolver_preserves_none_when_supported() {
        let caps = ReasoningCapability::from_values(["none", "medium"]);
        assert_eq!(
            resolve_reasoning_effort("none", &caps),
            ReasoningResolution::Applied {
                requested: ReasoningEffort::None,
                resolved: ReasoningEffort::None,
                reason: ReasoningResolutionReason::Exact,
            }
        );
    }

    #[test]
    fn resolver_falls_back_from_none_to_lowest_supported_non_none_with_reason() {
        let caps = ReasoningCapability::from_values(["medium", "max"]);
        assert_eq!(
            resolve_reasoning_effort("none", &caps),
            ReasoningResolution::Applied {
                requested: ReasoningEffort::None,
                resolved: ReasoningEffort::Medium,
                reason: ReasoningResolutionReason::NoneUnsupportedUsingLowestSupportedNonNone,
            }
        );
    }

    #[test]
    fn resolver_reports_unapplied_for_unknown_or_empty_capabilities() {
        assert_eq!(
            resolve_reasoning_effort(
                "high",
                &ReasoningCapability::from_values(["swarm", "bogus"])
            ),
            ReasoningResolution::Unapplied {
                requested: Some("high".to_string()),
                reason: ReasoningResolutionReason::CapabilityUnavailable,
            }
        );
        assert_eq!(
            resolve_reasoning_effort("", &ReasoningCapability::from_values(["low"])),
            ReasoningResolution::Unapplied {
                requested: None,
                reason: ReasoningResolutionReason::UnknownRequestedEffort,
            }
        );
        assert_eq!(
            resolve_reasoning_effort("turbo", &ReasoningCapability::from_values(["low"])),
            ReasoningResolution::Unapplied {
                requested: Some("turbo".to_string()),
                reason: ReasoningResolutionReason::UnknownRequestedEffort,
            }
        );
    }
}
