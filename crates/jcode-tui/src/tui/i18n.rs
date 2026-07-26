//! Chinese (Simplified) i18n support for the TUI.
//!
//! Provides language-aware UI string resolution so the same surface (model picker,
//! help text, status bar) renders in Chinese or English based on the user's
//! `display.language` config setting.

use crate::config::config;

/// Supported UI languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    En,
    Zh,
}

/// Resolve the current language from the persisted config.
pub fn current_language() -> Language {
    match config().display.language.as_deref() {
        Some("zh") => Language::Zh,
        _ => Language::En,
    }
}

/// Return `zh` when the current language is Chinese, `en` otherwise.
/// Convenience for inline ternaries.
pub fn if_zh(zh: &'static str, en: &'static str) -> &'static str {
    if current_language() == Language::Zh { zh } else { en }
}

/// Translate a PickerKind schema label.
pub fn picker_primary_label() -> &'static str {
    if_zh("模型", "MODEL")
}
pub fn picker_secondary_label() -> &'static str {
    if_zh("提供商", "PROVIDER")
}
pub fn picker_tertiary_label() -> &'static str {
    if_zh("方式", "METHOD")
}
pub fn picker_preview_submit_hint() -> &'static str {
    if_zh("  ↵ 选择", "  ↵ open")
}
pub fn picker_active_submit_hint() -> &'static str {
    if_zh("  ↑↓ ←→ ↵ 退出", "  ↑↓ ←→ ↵ Esc")
}

/// Localized label for a reasoning effort level.
/// Returns the label in the current UI language (Chinese or English).
pub fn effort_label(effort: &str) -> &'static str {
    match current_language() {
        Language::Zh => match effort {
            "none" => "无",
            "low" => "低",
            "medium" | "med" => "中",
            "high" => "高",
            "xhigh" => "极高",
            "max" => "最大",
            _ => "?",
        },
        Language::En => match effort {
            "none" => "none",
            "low" => "low",
            "medium" | "med" => "med",
            "high" => "high",
            "xhigh" => "xhigh",
            "max" => "max",
            _ => "?",
        },
    }
}

/// Chinese labels for reasoning effort levels (legacy, always Chinese).
pub fn effort_label_zh(effort: &str) -> &'static str {
    match effort {
        "none" => "无",
        "low" => "低",
        "medium" | "med" => "中",
        "high" => "高",
        "xhigh" => "极高",
        "max" => "最大",
        _ => "?",
    }
}

/// Model picker top hint.
pub fn model_picker_hint() -> &'static str {
    if_zh(
        " 快捷键: Ctrl+O 设置默认 · Ctrl+N 收藏 · Shift+Tab 切换到下一个收藏模型 · ←→ 切换思考等级",
        " keys: Ctrl+O set default · Ctrl+N favorite · Shift+Tab switch active model to next favorite · ←→ cycle effort",
    )
}