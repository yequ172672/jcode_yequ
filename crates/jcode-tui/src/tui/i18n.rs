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
    if current_language() == Language::Zh {
        zh
    } else {
        en
    }
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
            "minimal" => "最小",
            "low" => "低",
            "medium" | "med" => "中",
            "high" => "高",
            "xhigh" => "极高",
            "max" => "最大",
            _ => "?",
        },
        Language::En => match effort {
            "none" => "none",
            "minimal" => "minimal",
            "low" => "low",
            "medium" | "med" => "med",
            "high" => "high",
            "xhigh" => "xhigh",
            "max" => "max",
            _ => "?",
        },
    }
}

/// Model picker top hint.
pub fn model_picker_hint() -> &'static str {
    if_zh(
        " 快捷键: Ctrl+O 设置默认 · Ctrl+N 收藏 · Shift+Tab 切换到下一个收藏模型 · ←→ 切换思考等级",
        " keys: Ctrl+O set default · Ctrl+N favorite · Shift+Tab switch active model to next favorite · ←→ cycle effort",
    )
}

// ── Account─────────────────────────

pub fn account_primary_label() -> &'static str {
    if_zh("账户", "ACCOUNT")
}
pub fn account_secondary_label() -> &'static str {
    if_zh("状态", "STATE")
}
pub fn account_preview_submit_hint() -> &'static str {
    if_zh("  ↵ 选择", "  ↵ select")
}
pub fn account_active_submit_hint() -> &'static str {
    if_zh("  ↑↓/jk ↵ 退出", "  ↑↓/jk ↵ Esc")
}

/// Account picker compact state labels.
pub fn account_state_active() -> &'static str {
    if_zh("当前", "active")
}
pub fn account_state_saved() -> &'static str {
    if_zh("已保存", "saved")
}
pub fn account_state_add() -> &'static str {
    if_zh("添加", "add")
}
pub fn account_state_replace() -> &'static str {
    if_zh("替换", "replace")
}
pub fn account_state_manage() -> &'static str {
    if_zh("管理", "manage")
}

// ── Login picker ────────────────────────────────────────────────────────────

pub fn login_primary_label() -> &'static str {
    if_zh("项目", "ITEM")
}
pub fn login_secondary_label() -> &'static str {
    if_zh("提供商", "PROVIDER")
}
pub fn login_tertiary_label() -> &'static str {
    if_zh("操作", "ACTION")
}

// ── Usage picker ────────────────────────────────────────────────────────────

pub fn usage_primary_label() -> &'static str {
    if_zh("项目", "ITEM")
}
pub fn usage_secondary_label() -> &'static str {
    if_zh("状态", "STATUS")
}
pub fn usage_tertiary_label() -> &'static str {
    if_zh("窗口", "WINDOW")
}

// ── Agent target picker ─────────────────────────────────────────────────────

pub fn agent_target_primary_label() -> &'static str {
    if_zh("目标", "TARGET")
}
pub fn agent_target_secondary_label() -> &'static str {
    if_zh("模型", "MODEL")
}
pub fn agent_target_tertiary_label() -> &'static str {
    if_zh("配置", "CONFIG")
}

// ── Common UI hints ─────────────────────────────────────────────────────────

pub fn no_matches_label() -> &'static str {
    if_zh("   无匹配", "   no matches")
}
pub fn default_shortcut_hint() -> &'static str {
    if_zh("  Ctrl-O=设为默认", "  Ctrl-O=set default")
}
pub fn entry_default_label() -> &'static str {
    if_zh(" 默认", " default")
}
pub fn entry_new_label() -> &'static str {
    if_zh(" 新", " new")
}
pub fn entry_old_label() -> &'static str {
    if_zh(" 旧", " old")
}

// ── Input hints ─────────────────────────────────────────────────────────────

pub fn shell_mode_local_hint() -> &'static str {
    if_zh("  shell 模式 · Enter 在本地执行", "  shell mode · Enter runs locally")
}
pub fn shell_mode_remote_hint() -> &'static str {
    if_zh("  shell 模式 · Enter 在服务器执行", "  shell mode · Enter runs on server")
}
pub fn new_session_hint() -> &'static str {
    if_zh("  ↗ 下一条提示将开启新会话", "  ↗ Next prompt opens a new session")
}
pub fn send_now_hint() -> &'static str {
    if_zh("  Ctrl/Cmd+Enter 立即发送", "  Ctrl/Cmd+Enter to send now")
}
pub fn queue_hint() -> &'static str {
    if_zh("  Ctrl/Cmd+Enter 排队", "  Ctrl/Cmd+Enter to queue")
}

// ── Session picker status labels ────────────────────────────────────────────
// Individual status labels are API surfaces; some may be unused in current code
// but kept for completeness with the format variants below.
#[allow(dead_code)]

pub fn session_status_active() -> &'static str {
    if_zh("活跃", "active")
}
#[allow(dead_code)]
pub fn session_status_closed() -> &'static str {
    if_zh("已关闭", "closed")
}
#[allow(dead_code)]
pub fn session_status_crashed() -> &'static str {
    if_zh("已崩溃", "crashed")
}
#[allow(dead_code)]
pub fn session_status_reloaded() -> &'static str {
    if_zh("已重载", "reloaded")
}
#[allow(dead_code)]
pub fn session_status_compacted() -> &'static str {
    if_zh("已压缩", "compacted")
}
#[allow(dead_code)]
pub fn session_status_rate_limited() -> &'static str {
    if_zh("频率受限", "rate-limited")
}
#[allow(dead_code)]
pub fn session_status_errored() -> &'static str {
    if_zh("错误", "errored")
}
pub fn session_status_live_claude() -> &'static str {
    if_zh("Claude 在线", "live Claude")
}
pub fn session_status_working() -> &'static str {
    if_zh("工作中", "working")
}
pub fn session_status_ready() -> &'static str {
    if_zh("就绪", "ready")
}

// ── Session status format helpers (status + duration) ───────────────────────

pub fn session_status_working_format(duration: String) -> String {
    match current_language() {
        Language::Zh => format!("工作中 {}", duration),
        Language::En => format!("working {}", duration),
    }
}
pub fn session_status_closed_format(time_ago: &str) -> String {
    match current_language() {
        Language::Zh => format!("已关闭 {}", time_ago),
        Language::En => format!("closed {}", time_ago),
    }
}
pub fn session_status_crashed_format(time_ago: &str) -> String {
    match current_language() {
        Language::Zh => format!("已崩溃 {}", time_ago),
        Language::En => format!("crashed {}", time_ago),
    }
}
pub fn session_status_reloaded_format(time_ago: &str) -> String {
    match current_language() {
        Language::Zh => format!("已重载 {}", time_ago),
        Language::En => format!("reloaded {}", time_ago),
    }
}
pub fn session_status_compacted_format(time_ago: &str) -> String {
    match current_language() {
        Language::Zh => format!("已压缩 {}", time_ago),
        Language::En => format!("compacted {}", time_ago),
    }
}
pub fn session_status_rate_limited_format(time_ago: &str) -> String {
    match current_language() {
        Language::Zh => format!("频率受限 {}", time_ago),
        Language::En => format!("rate-limited {}", time_ago),
    }
}
pub fn session_status_errored_format(time_ago: &str) -> String {
    match current_language() {
        Language::Zh => format!("错误 {}", time_ago),
        Language::En => format!("errored {}", time_ago),
    }
}

// ── Info widget ─────────────────────────────────────────────────────────────

pub fn session_count_label(count: usize) -> String {
    match current_language() {
        Language::Zh => format!("{} 会话", count),
        Language::En => {
            if count == 1 {
                format!("{} session", count)
            } else {
                format!("{} sessions", count)
            }
        }
    }
}

// ── Common system notices ───────────────────────────────────────────────────

pub fn interrupted_label() -> &'static str {
    if_zh("已中断", "Interrupted")
}
pub fn login_cancelled_label() -> &'static str {
    if_zh("登录已取消。", "Login cancelled.")
}
pub fn no_rewind_label() -> &'static str {
    if_zh("没有可撤销的回退。", "No rewind to undo.")
}
pub fn workspace_mode_off() -> &'static str {
    if_zh("工作区模式：关", "Workspace mode: off")
}

pub fn workspace_mode_on() -> &'static str {
    if_zh("工作区模式：开", "Workspace mode: on")
}
pub fn workspace_current_workspace() -> &'static str {
    if_zh("当前工作区", "Current workspace")
}
pub fn workspace_visible_rows() -> &'static str {
    if_zh("可见行数", "Visible rows")
}
pub fn workspace_populated_workspaces() -> &'static str {
    if_zh("已使用工作区数", "Populated workspaces")
}
pub fn workspace_mapped_sessions() -> &'static str {
    if_zh("已映射会话数", "Mapped sessions")
}
pub fn scheduled_tasks_active_label() -> &'static str {
    if_zh("已调度任务活跃", "Scheduled tasks active")
}
pub fn scheduled_tasks_label(count: usize) -> String {
    match current_language() {
        Language::Zh => format!("{} 个已调度任务", count),
        Language::En => {
            if count == 1 {
                "1 scheduled task".to_string()
            } else {
                format!("{} scheduled tasks", count)
            }
        }
    }
}
pub fn tasks_queued_label(count: usize) -> String {
    match current_language() {
        Language::Zh => format!("{} 个排队任务", count),
        Language::En => {
            if count == 1 {
                "1 task queued".to_string()
            } else {
                format!("{} tasks queued", count)
            }
        }
    }
}
pub fn not_running_label() -> &'static str {
    if_zh("未运行", "Not running")
}
pub fn memory_label(count: usize) -> &'static str {
    match current_language() {
        Language::Zh => "记忆",
        Language::En => if count == 1 { "memory" } else { "memories" },
    }
}
pub fn remote_starting_server() -> &'static str {
    if_zh("正在启动服务器…", "starting server…")
}
pub fn remote_connecting() -> &'static str {
    if_zh("正在连接服务器…", "connecting to server…")
}
pub fn remote_loading_session() -> &'static str {
    if_zh("正在加载会话…", "loading session…")
}
pub fn remote_waiting_for_reload() -> &'static str {
    if_zh("正在等待重载…", "waiting for reload…")
}
pub fn remote_reconnecting(attempt: u32) -> String {
    match current_language() {
        Language::Zh => format!("正在重连（第 {} 次）…", attempt),
        Language::En => format!("reconnecting ({attempt})…"),
    }
}
pub fn improve_active_loop() -> &'static str {
    if_zh("活跃改进循环", "active improvement loop")
}
pub fn improve_plan_only() -> &'static str {
    if_zh("仅改进计划", "improvement plan-only")
}
pub fn refactor_active_loop() -> &'static str {
    if_zh("活跃重构循环", "active refactor loop")
}
pub fn refactor_plan_only() -> &'static str {
    if_zh("仅重构计划", "refactor plan-only")
}
pub fn subagent_model_usage(current_summary: &str) -> String {
    match current_language() {
        Language::Zh => format!(
            "当前会话的子代理模型：{}\n\n使用 /subagent-model <名称> 固定模型，或 /subagent-model inherit 使用当前模型。",
            current_summary
        ),
        Language::En => format!(
            "Subagent model for this session: {}\n\nUse /subagent-model <name> to pin a model, or /subagent-model inherit to use the current model.",
            current_summary
        ),
    }
}
pub fn subagent_model_inherit_notice() -> &'static str {
    if_zh("子代理模型：继承", "Subagent model: inherit")
}
pub fn language_switched_zh() -> &'static str {
    "语言已切换为中文。"
}
pub fn language_switched_en() -> &'static str {
    "Language switched to English."
}
pub fn language_status_zh() -> &'static str {
    "语言：中文"
}
pub fn language_status_en() -> &'static str {
    "Language: English"
}
pub fn language_usage() -> &'static str {
    if_zh(
        "用法: /language [en|zh]\n  /language    - 打开语言选择器\n  /language en - 切换到英文\n  /language zh - 切换到中文",
        "Usage: /language [en|zh]\n  /language    - open the language picker\n  /language en - switch to English\n  /language zh - 切换到中文",
    )
}
pub fn observe_mode_disabled() -> &'static str {
    if_zh("观察模式已禁用。", "Observe mode disabled.")
}
pub fn btw_usage() -> &'static str {
    if_zh("用法: /btw <问题>", "Usage: /btw <question>")
}
