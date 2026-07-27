//! Placeholder-route helpers for the remote model catalog (issue: poisoned
//! "remote-catalog"/"current" rows pinning models to useless routes).
//!
//! "remote-catalog"/"current" are placeholder methods from names-only catalog
//! downgrades, not real provider routes. A model covered only by placeholders
//! still needs its actual routes (OpenAI OAuth/API key, OpenRouter, ...)
//! synthesized, and placeholder routes must never be persisted to the catalog
//! cache: they describe a catalog that is still refreshing, and persisting
//! them would resurrect useless rows on the next cold start.

/// Whether `method` is a placeholder api-method rather than a real route.
pub(super) fn is_placeholder_route_method(method: &str) -> bool {
    matches!(
        crate::provider::ModelRouteApiMethod::parse(method),
        crate::provider::ModelRouteApiMethod::RemoteCatalog
            | crate::provider::ModelRouteApiMethod::Current
    )
}

/// Whether every method in `methods` is a placeholder.
pub(super) fn methods_are_placeholder_only<I>(methods: I) -> bool
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    methods
        .into_iter()
        .all(|method| is_placeholder_route_method(method.as_ref()))
}

/// Whether a picker route can carry the session's reasoning-effort preference.
///
/// The preference is intentionally available for every concrete runtime. The
/// active provider resolves it to its supported wire value (or keeps it as an
/// unapplied preference when the protocol has no control field). Placeholder
/// catalog rows are the only exception because they are not selectable runtime
/// identities and cannot safely persist a route-specific preference.
pub(super) fn route_supports_reasoning_effort(api_method: &str) -> bool {
    use crate::provider::ModelRouteApiMethod as Method;
    !matches!(
        Method::parse(api_method),
        Method::RemoteCatalog | Method::Current
    )
}
