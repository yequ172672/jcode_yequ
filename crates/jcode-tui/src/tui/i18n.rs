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
// -- Help overlay functions --

pub fn help_section_commands() -> &'static str { if_zh("  命令", "  Commands") }
pub fn help_section_keys() -> &'static str { if_zh("  快捷键", "  Keys") }
pub fn help_section_skills() -> &'static str { if_zh("  技能", "  Skills") }
pub fn help_section_all_skills() -> &'static str { if_zh("  所有技能", "  All Skills") }
pub fn help_entry_help() -> &'static str { if_zh("显示此帮助覆盖层", "Show this help overlay") }
pub fn help_entry_help_command() -> &'static str { if_zh("显示一个命令的详细信息", "Show details for one command") }
pub fn help_entry_model() -> &'static str { if_zh("列出或切换模型", "List or switch models") }
pub fn help_entry_model_switch() -> &'static str { if_zh("切换到指定模型", "Switch to a different model") }
pub fn help_entry_provider_test() -> &'static str { if_zh("显示当前提供商/模型的实测证据", "Show live-test evidence for the current provider/model") }
pub fn help_entry_agents() -> &'static str { if_zh("配置代理角色的模型", "Configure models for agent roles") }
pub fn help_entry_swarm_prompt() -> &'static str { if_zh("打开当前 swarm 路由提示", "Open the active swarm routing prompt in your editor") }
pub fn help_entry_effort() -> &'static str { if_zh("设置思考等级", "Set effort") }
pub fn help_entry_fast() -> &'static str { if_zh("切换快速模式", "Toggle fast mode") }
pub fn help_entry_transport() -> &'static str { if_zh("设置连接方式", "Set connection transport") }
pub fn help_entry_alignment() -> &'static str { if_zh("显示或持久化文本对齐偏好", "Show or persist text alignment preference") }
pub fn help_entry_compact_notifications() -> &'static str { if_zh("折叠 swarm/文件活动通知为单行", "Collapse swarm/file-activity notifications to one line") }
pub fn help_entry_show_agentgrep() -> &'static str { if_zh("在聊天中内联渲染完整的 agentgrep 搜索结果", "Render full agentgrep search output inline in chat") }
pub fn help_entry_config() -> &'static str { if_zh("显示当前配置", "Show active configuration") }
pub fn help_entry_config_init() -> &'static str { if_zh("创建默认配置文件", "Create default config file") }
pub fn help_entry_config_edit() -> &'static str { if_zh("在 $EDITOR 中打开配置", "Open config in $EDITOR") }
pub fn help_entry_dictate() -> &'static str { if_zh("运行配置的外部听写", "Run configured external dictation") }
pub fn help_entry_git() -> &'static str { if_zh("显示仓库分支和工作树状态", "Show branch and working tree status for the repo") }
pub fn help_entry_context() -> &'static str { if_zh("显示完整的会话上下文快照", "Show the full session context snapshot") }
pub fn help_entry_skills() -> &'static str { if_zh("显示已加载的技能和推荐", "Show loaded skills and jcode-endorsed recommendations") }
pub fn help_entry_info() -> &'static str { if_zh("显示会话信息和令牌用量", "Show session info and token usage") }
pub fn help_entry_keys() -> &'static str { if_zh("显示与终端/OS 的按键绑定冲突", "Show keybinding conflicts with your terminal/OS") }
pub fn help_entry_usage() -> &'static str { if_zh("显示已连接提供商的用量限制", "Show connected provider usage limits") }
pub fn help_entry_support() -> &'static str { if_zh("发送支持邮件（预填诊断信息）", "Email support with diagnostics prefilled") }
pub fn help_entry_version() -> &'static str { if_zh("显示版本和构建详情", "Show version and build details") }
pub fn help_entry_changelog() -> &'static str { if_zh("显示此构建的近期变更", "Show recent changes in this build") }
pub fn help_entry_btw() -> &'static str { if_zh("对之前的话题问一个跟进问题", "Ask a follow-up about an earlier topic") }
pub fn help_entry_clear() -> &'static str { if_zh("清空对话并从头开始", "Clear conversation and start fresh") }
pub fn help_entry_export() -> &'static str { if_zh("将会话导出为纯文本格式", "Export session as plain text") }
pub fn help_entry_copy_last_url() -> &'static str { if_zh("将最后聊到的 URL 复制到4剪贴板", "Copy the last chat URL to clipboard") }
pub fn help_entry_improve() -> &'static str { if_zh("启动自动改进循环", "Start an automatic improvement loop") }
pub fn help_entry_refactor() -> &'static str { if_zh("启动自动重构循环", "Start an automatic refactor loop") }
pub fn help_entry_catch_up() -> &'static str { if_zh("跳转至已完成的会话并打开摘要", "Jump to finished sessions and open a Catch Up brief") }
pub fn help_entry_edit() -> &'static str { if_zh("在 $EDITOR 中打开信息卡", "Open info cards in $EDITOR") }
pub fn help_entry_notifications() -> &'static str { if_zh("管理实时会话：查看哪些在运行和就绪", "Manage live sessions: see which are working vs ready") }
pub fn help_entry_windmill() -> &'static str { if_zh("启用和管理 Niri 风格的会话工作区", "Enable and manage the Niri-style session workspace") }
pub fn help_entry_observe() -> &'static str { if_zh("让代理阅读聊天并在之后提问", "Let the agent catch up on chat and ask questions later") }
pub fn help_entry_workspace() -> &'static str { if_zh("切换到另一个工作区和会话", "Switch to another workspace and session") }
pub fn help_entry_workspace_new() -> &'static str { if_zh("创建一个新的空工作区", "Create a new empty workspace") }
pub fn help_entry_workspace_list() -> &'static str { if_zh("列出所有工作区中的活动会话", "List active sessions in all workspaces") }
pub fn help_entry_workspace_rename() -> &'static str { if_zh("重命名当前工作区", "Rename the current workspace") }
pub fn help_entry_workspace_remove() -> &'static str { if_zh("删除当前工作区及其会话", "Delete the current workspace and its sessions") }
pub fn help_entry_resume() -> &'static str { if_zh("浏览和恢复之前的会话", "Browse and resume previous sessions") }
pub fn help_entry_save() -> &'static str { if_zh("将会话加书签以供 /resume 使用", "Bookmark session for /resume") }
pub fn help_entry_share() -> &'static str { if_zh("导出并编码当前会话为共享文本", "Export and encode the current session as share text") }
pub fn help_entry_onboarding() -> &'static str { if_zh("重新显示设置欢迎屏幕", "Re-show the setup welcome screen") }
pub fn help_entry_memory() -> &'static str { if_zh("切换记忆功能", "Toggle memory features") }
pub fn help_entry_forget() -> &'static str { if_zh("从当前项目记忆中移除项目", "Remove an item from current project memory") }
pub fn help_entry_telemetry() -> &'static str { if_zh("查看或更改遥测偏好", "View or change telemetry preferences") }
pub fn help_entry_swarm() -> &'static str { if_zh("切换 swarm 功能", "Toggle swarm features") }
pub fn help_entry_login() -> &'static str { if_zh("交互式或直接登录", "Interactive or direct login") }
pub fn help_entry_logout() -> &'static str { if_zh("登出提供商", "Log out from a provider") }
pub fn help_entry_account() -> &'static str { if_zh("管理已保存的账户", "Manage saved accounts") }
pub fn help_entry_subagent_model() -> &'static str { if_zh("设置子代理模型覆盖", "Set sub-agent model override") }
pub fn help_entry_language() -> &'static str { if_zh("切换语言", "Switch language") }
pub fn help_entry_reload() -> &'static str { if_zh("如有新版本则重载到新二进制", "Reload to newer binary if available") }
pub fn help_entry_continue_all() -> &'static str { if_zh("继续所有会自恢复的中断的实时会话", "Continue every interrupted live session that would auto-resume") }
pub fn help_entry_quit() -> &'static str { if_zh("退出 jcode", "Exit jcode") }
pub fn help_entry_client_reload() -> &'static str { if_zh("强制重载客户端二进制", "Force reload client binary") }
pub fn help_entry_server_reload() -> &'static str { if_zh("强制重载服务器二进制", "Force reload server binary") }
pub fn help_entry_get() -> &'static str { if_zh("从会话恢复提示（高级）", "Get resume prompt from a session (advanced)") }
pub fn help_entry_yank() -> &'static str { if_zh("将会话内容拉取到此会话的提示中", "Pull session content into this session's prompts") }
pub fn help_skill_activate() -> &'static str { if_zh("激活技能", "Activate skill") }

// -- Onboarding --
pub fn onboarding_esc_skip() -> &'static str { if_zh("Esc 跳过引导（稍后用 /login 登录）。", "Esc to skip onboarding (log in later with /login).") }
pub fn onboarding_yes() -> &'static str { if_zh("是", "Yes") }
pub fn onboarding_no() -> &'static str { if_zh("否", "No") }
pub fn onboarding_continue() -> &'static str { if_zh("继续", "Continue") }
pub fn onboarding_import_less() -> &'static str { if_zh("导具更少", "Import less") }
pub fn onboarding_telemetry_settings() -> &'static str { if_zh("遥测设置", "Telemetry settings") }
pub fn onboarding_telemetry_title() -> &'static str { if_zh("遥测设置", "Telemetry settings") }
pub fn onboarding_send_everything() -> &'static str { if_zh("发送所有内容（包括提示词）", "Send everything, including prompts") }
pub fn onboarding_helps_most() -> &'static str { if_zh("最能帮助 jcode 改进", "Helps jcode the most") }
pub fn onboarding_no_content() -> &'static str { if_zh("不发送提示词或转录", "No prompts or transcripts") }
pub fn onboarding_usage_stats() -> &'static str { if_zh("仅用量统计和崩溃报告", "Usage stats and crash reports only") }
pub fn onboarding_send_nothing() -> &'static str { if_zh("不发送任何内容", "Send nothing") }
pub fn onboarding_no_crash_fix() -> &'static str { if_zh("我们将无法看到崩溃并修复它们", "We stop seeing crashes and can't fix them") }
pub fn onboarding_telemetry_env_disabled() -> &'static str { if_zh("您的环境已禁用遥测（JCODE_NO_TELEMETRY）。", "Your environment already disables telemetry (JCODE_NO_TELEMETRY).") }
pub fn onboarding_esc_back() -> &'static str { if_zh("Esc 返回。稍后可用 /telemetry 更改。", "Esc goes back. Change this later with /telemetry.") }
pub fn onboarding_choose_provider() -> &'static str { if_zh("按 Enter 选择提供商（OpenAI、Anthropic 等）。", "Press Enter to choose a provider (OpenAI, Anthropic, and more).") }
pub fn onboarding_help_fix(agent: &str) -> String {
    match current_language() {
        Language::Zh => format!("按 H 让 {} 帮助修复此问题。", agent),
        Language::En => format!("Press H to have {} help fix this for you.", agent),
    }
}
pub fn onboarding_pick_login() -> &'static str { if_zh("按 Enter 选择登录对象（OpenAI、Anthropic 等）。", "Press Enter to pick who to log in with (OpenAI, Anthropic, and more).") }
pub fn onboarding_press_number(count: usize) -> String {
    match current_language() {
        Language::Zh => format!("按 1-{} 或输入任意内容开始", count),
        Language::En => format!("Press 1-{} or type anything to start", count),
    }
}

