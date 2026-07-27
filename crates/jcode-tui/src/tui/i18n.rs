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

pub fn swarm_gallery_label() -> &'static str { if_zh("swarm", "swarm") }
pub fn swarm_gallery_header(count: usize, active: usize) -> String {
    match current_language() {
        Language::Zh => format!("· {} agent{} · {} 活跃", count, if count == 1 { "" } else { "s" }, active),
        Language::En => format!("· {} agent{} · {} active", count, if count == 1 { "" } else { "s" }, active),
    }
}
pub fn swarm_gallery_hints() -> &'static str { if_zh("alt+n 聊天  ·  alt+↑/↓ 选择  ·  alt+o 打开  ·  alt+shift+p 提示  ·  esc 聊天", "alt+n chat  ·  alt+↑/↓ select  ·  alt+o open  ·  alt+shift+p prompt  ·  esc chat") }
pub fn onboarding_press_number(count: usize) -> String {
    match current_language() {
        Language::Zh => format!("按 1-{} 或输入任意内容开始", count),
        Language::En => format!("Press 1-{} or type anything to start", count),
    }
}

// -- Help overlay: section headers --
pub fn help_section_session() -> &'static str { if_zh("  会话", "  Session") }
pub fn help_section_memory_swarm() -> &'static str { if_zh("  记忆 & Swarm", "  Memory & Swarm") }
pub fn help_section_auth_accounts() -> &'static str { if_zh("  认证 & 账户", "  Auth & Accounts") }
pub fn help_section_system() -> &'static str { if_zh("  系统", "  System") }
pub fn help_section_navigation() -> &'static str { if_zh("  导航", "  Navigation") }
pub fn help_section_input_editing() -> &'static str { if_zh("  输入 & 编辑", "  Input & Editing") }
pub fn help_section_model_status() -> &'static str { if_zh("  模型状态", "  Model Status") }

// -- Help overlay: key entry descriptions --
pub fn help_key_scroll_history() -> &'static str { if_zh("滚动历史", "Scroll history") }
pub fn help_key_scroll_input() -> &'static str { if_zh("滚动为空时）", "Scroll history (when input empty)") }
pub fn help_key_resize_panel_25() -> &'static str { if_zh("调整侧面板大小至 25/50/75/100%", "Resize side panel to 25/50/75/100%") }
pub fn help_key_toggle_diagram() -> &'static str { if_zh("切换图表位置（侧边/顶部）", "Toggle diagram position (side/top)") }
pub fn help_key_focus_chat() -> &'static str { if_zh("聚焦聊天 / 图表 / 差异", "Focus chat / diagram / diffs") }
pub fn help_key_pan_diagram() -> &'static str { if_zh("平移图表（聚焦时）", "Pan diagram (when focused)") }
pub fn help_key_zoom_diagram() -> &'static str { if_zh("缩放图表（聚焦时）", "Zoom diagram (when focused)") }
pub fn help_key_resize_diagram() -> &'static str { if_zh("调整图表面板大小", "Resize diagram pane") }
pub fn help_key_cycle_favorites() -> &'static str { if_zh("循环切换收藏模型", "Cycle favorited models") }
pub fn help_key_set_default() -> &'static str { if_zh("设置默认模型（在 /model 选择器中）", "Set default model (in /model picker)") }
pub fn help_key_cut_line() -> &'static str { if_zh("剪切整行输入到剪贴板", "Cut entire input line to clipboard") }
pub fn help_key_clear_input() -> &'static str { if_zh("清空输入行", "Clear input line") }
pub fn help_key_delete_to_end() -> &'static str { if_zh("删除到输入末尾", "Delete to end of input") }
pub fn help_key_home_end() -> &'static str { if_zh("移动到输入开头 / 末尾", "Move to start / end of input") }
pub fn help_key_undo_edit() -> &'static str { if_zh("撤销输入编辑", "Undo input edit") }
pub fn help_key_cut_paste() -> &'static str { if_zh("剪切输入 / 粘贴剪贴板", "Cut input / paste clipboard") }
pub fn help_key_stash_pop() -> &'static str { if_zh("暂存 / 弹出输入（保存以便后用）", "Stash / pop input (save for later)") }
pub fn help_key_delete_word() -> &'static str { if_zh("删除输入中前一个单词", "Delete previous word in input") }
pub fn help_key_move_word_lr() -> &'static str { if_zh("按词左右移动", "Move by word left / right") }
pub fn help_key_retrieve_pending() -> &'static str { if_zh("检索待发送消息以供编辑", "Retrieve pending message for editing") }
pub fn help_key_toggle_queue() -> &'static str { if_zh("切换队列模式", "Toggle queue mode") }
pub fn help_key_recover_tools() -> &'static str { if_zh("恢复缺失的工具输出", "Recover from missing tool outputs") }
pub fn help_key_toggle_chat_select() -> &'static str { if_zh("切换聊天选择/复制模式", "Toggle chat selection/copy mode") }
pub fn help_key_toggle_scroll_lock() -> &'static str { if_zh("切换输入滚动锁", "Toggle typing scroll lock") }
pub fn help_key_toggle_auto_poke() -> &'static str { if_zh("切换未完成待办的自动提醒", "Toggle auto-poke for incomplete todos") }
pub fn help_key_toggle_todo_card() -> &'static str { if_zh("显示/隐藏聊天中的待办卡片", "Show/dismiss todo list card in chat") }

// -- Remaining help entries --
pub fn help_entry_auth() -> &'static str { if_zh("显示认证状态", "Show authentication status") }

// -- Onboarding (remaining) --
pub fn onboarding_welcome_title() -> &'static str { if_zh("欢迎使用 jcode 引导", "Welcome to jcode onboarding") }
pub fn onboarding_keyboard_hint() -> &'static str { if_zh("使用键盘导航。", "Use your keyboard to navigate.") }
pub fn onboarding_importing() -> &'static str { if_zh("正在导入您的登录…", "Importing your logins…") }
pub fn onboarding_wait_moment() -> &'static str { if_zh("请稍候，只需片刻。", "Hang tight, this only takes a moment.") }
pub fn onboarding_import_failed() -> &'static str { if_zh("无法导入那些登录。", "We couldn't import those logins.") }
pub fn onboarding_no_problem() -> &'static str { if_zh("没问题 - 您可以直接登录。", "No problem - you can log in directly.") }
pub fn onboarding_first_login() -> &'static str { if_zh("请先登录以开始。", "First, log in to get started.") }
pub fn onboarding_found_logins(found: usize) -> String {
    match current_language() {
        Language::Zh => format!("找到 {} 个现有登录：", found),
        Language::En => format!("We found {} existing login{}:", found, if found == 1 { "" } else { "s" }),
    }
}
pub fn onboarding_import_label() -> &'static str { if_zh("Import：", "Import:") }
pub fn onboarding_login_provider(provider: &str) -> String {
    match current_language() {
        Language::Zh => format!("Login to {}?", provider),
        Language::En => format!("Log in to {}?", provider),
    }
}
pub fn onboarding_resume_continue(cli_label: &str) -> String {
    match current_language() {
        Language::Zh => format!("Continue in {}?", cli_label),
        Language::En => format!("Continue where you left off in {}?", cli_label),
    }
}
pub fn onboarding_auto_resume(seconds_left: u64, cli_label: &str) -> String {
    match current_language() {
        Language::Zh => format!("{sec}秒后自动打开{label}...", sec=seconds_left, label=cli_label),
        Language::En => format!("Opens the resume menu automatically in {}s...", seconds_left),
    }
}
// -- Status notices --
pub fn account_center_unavailable() -> &'static str { if_zh("账户", "Account center unavailable") }
pub fn account_center_choose_add_replace_target() -> &'static str { if_zh("账户", "Account center: choose add/replace target") }
pub fn account_center_choose_an_action() -> &'static str { if_zh("账户", "Account center: choose an action") }
pub fn account_picker_unavailable() -> &'static str { if_zh("账户", "Account picker unavailable") }
pub fn account_cancelled() -> &'static str { if_zh("Account:已取消", "Account: cancelled") }
pub fn active_sessions_loaded() -> &'static str { if_zh("Active sessions已加载", "Active sessions loaded") }
pub fn agent_model_save_failed() -> &'static str { if_zh("Agent model save失败", "Agent model save failed") }
pub fn already_in_this_session() -> &'static str { if_zh("已在此会话中", "Already in this session") }
pub fn architecture_review_queued() -> &'static str { if_zh("Architecture review已排队", "Architecture review queued") }
pub fn autojudge_launch_failed() -> &'static str { if_zh("Autojudge launch失败", "Autojudge launch failed") }
pub fn autojudge_queued() -> &'static str { if_zh("Autojudge已排队", "Autojudge queued") }
pub fn autojudge_off() -> &'static str { if_zh("Autojudge：关", "Autojudge: OFF") }
pub fn autojudge_on() -> &'static str { if_zh("Autojudge：开", "Autojudge: ON") }
pub fn autoreview_launch_failed() -> &'static str { if_zh("Autoreview launch失败", "Autoreview launch failed") }
pub fn autoreview_queued() -> &'static str { if_zh("Autoreview已排队", "Autoreview queued") }
pub fn autoreview_off() -> &'static str { if_zh("Autoreview：关", "Autoreview: OFF") }
pub fn autoreview_on() -> &'static str { if_zh("Autoreview：开", "Autoreview: ON") }
pub fn back_empty() -> &'static str { if_zh("", "Back: empty") }
pub fn bookmark_removed() -> &'static str { if_zh("Bookmark 已移除", "Bookmark removed") }
pub fn cache_stats() -> &'static str { if_zh("缓存统计", "Cache stats") }
pub fn catch_up_sessions_loaded() -> &'static str { if_zh("Catch Up sessions已加载", "Catch Up sessions loaded") }
pub fn catch_up_none_waiting() -> &'static str { if_zh("", "Catch Up: none waiting") }
pub fn changelog() -> &'static str { if_zh("更新日志", "Changelog") }
pub fn choose_a_suggested_review_or_start() -> &'static str { if_zh("选择", "Choose a suggested review or start a new session (↑↓, Enter)") }
pub fn claude_takeover_did_not_complete() -> &'static str { if_zh("Claude 接管", "Claude takeover did not complete") }
pub fn clearing_session_name() -> &'static str { if_zh("正在Clearing session name...", "Clearing session name...") }
pub fn compacting_context() -> &'static str { if_zh("正在压缩上下文", "Compacting context") }
pub fn compaction_failed() -> &'static str { if_zh("Compaction失败", "Compaction failed") }
pub fn context_compacted() -> &'static str { if_zh("", "Context compacted") }
pub fn continue_all_failed() -> &'static str { if_zh("Continue all失败", "Continue all failed") }
pub fn continuing_interrupted_sessions() -> &'static str { if_zh("正在Continuing interrupted sessions...", "Continuing interrupted sessions...") }
pub fn copied_selection() -> &'static str { if_zh("已复制选择内容", "Copied selection") }
pub fn copied_viewport_context() -> &'static str { if_zh("已复制视口上下文", "Copied viewport context") }
pub fn debug_gmail_draft_fixture_ready() -> &'static str { if_zh("", "Debug Gmail draft fixture ready") }
pub fn debug_expand_badge_fixture_ready() -> &'static str { if_zh("", "Debug expand badge fixture ready") }
pub fn diagram_image_not_found_on_disk() -> &'static str { if_zh("", "Diagram image not found on disk") }
pub fn diagram_not_cached() -> &'static str { if_zh("", "Diagram not cached") }
pub fn dictation_already_running() -> &'static str { if_zh("", "Dictation already running") }
pub fn dictation_failed() -> &'static str { if_zh("Dictation失败", "Dictation failed") }
pub fn dictation_not_configured() -> &'static str { if_zh("", "Dictation not configured") }
pub fn downloading_image() -> &'static str { if_zh("正在Downloading image...", "Downloading image...") }
pub fn effort_switch_failed() -> &'static str { if_zh("Effort switch失败", "Effort switch failed") }
pub fn emergency_compaction() -> &'static str { if_zh("紧急压缩", "Emergency compaction") }
pub fn failed_to_copy_input_line() -> &'static str { if_zh("", "Failed to copy input line") }
pub fn failed_to_copy_selection() -> &'static str { if_zh("", "Failed to copy selection") }
pub fn failed_to_copy_viewport_context() -> &'static str { if_zh("", "Failed to copy viewport context") }
pub fn failed_to_download_image() -> &'static str { if_zh("", "Failed to download image") }
pub fn fallback_resend_failed() -> &'static str { if_zh("Fallback resend失败", "Fallback resend failed") }
pub fn fallback_switch_failed() -> &'static str { if_zh("Fallback switch失败", "Fallback switch failed") }
pub fn feedback_recorded() -> &'static str { if_zh("反馈", "Feedback recorded") }
pub fn finish_current_work_before_catch_up() -> &'static str { if_zh("", "Finish current work before Catch Up") }
pub fn finish_current_work_before_going_back() -> &'static str { if_zh("", "Finish current work before going back") }
pub fn finish_current_work_before_moving_workspace() -> &'static str { if_zh("", "Finish current work before moving workspace focus") }
pub fn fix_applied() -> &'static str { if_zh("修复已应用", "Fix applied") }
pub fn focus_chat() -> &'static str { if_zh("焦点：聊天", "Focus: chat") }
pub fn focus_diagram_hjkl_pan_zoom_resize() -> &'static str { if_zh("", "Focus: diagram (hjkl pan, [/] zoom, +/- resize)") }
pub fn focus_side_pane_j_k_scroll() -> &'static str { if_zh("", "Focus: side pane (j/k scroll, Esc to return)") }
pub fn fork_failed() -> &'static str { if_zh("Fork失败", "Fork failed") }
pub fn forked_session_created() -> &'static str { if_zh("Forked session已创建", "Forked session created") }
pub fn git_status() -> &'static str { if_zh("Git 状态", "Git status") }
pub fn git_status_loading() -> &'static str { if_zh("正在Git status loading...", "Git status loading...") }
pub fn image_side_panel_off() -> &'static str { if_zh("Image side panel：关", "Image side panel: OFF") }
pub fn image_side_panel_on() -> &'static str { if_zh("Image side panel：开", "Image side panel: ON") }
pub fn initiatives() -> &'static str { if_zh("倡议", "Initiatives") }
pub fn input_cleared_ctrl_z_to_restore() -> &'static str { if_zh("", "Input cleared - Ctrl+Z to restore") }
pub fn interrupting_for_improve_resume() -> &'static str { if_zh("正在Interrupting for /improve resume...", "Interrupting for /improve resume...") }
pub fn interrupting_for_improve_stop() -> &'static str { if_zh("正在Interrupting for /improve stop...", "Interrupting for /improve stop...") }
pub fn interrupting_for_plan() -> &'static str { if_zh("正在Interrupting for /plan...", "Interrupting for /plan...") }
pub fn interrupting_for_refactor_resume() -> &'static str { if_zh("正在Interrupting for /refactor resume...", "Interrupting for /refactor resume...") }
pub fn interrupting_for_refactor_stop() -> &'static str { if_zh("正在Interrupting for /refactor stop...", "Interrupting for /refactor stop...") }
pub fn interrupting() -> &'static str { if_zh("正在Interrupting...", "Interrupting...") }
pub fn interrupting_auto_poke_off() -> &'static str { if_zh("正在中断", "Interrupting... Auto-poke OFF") }
pub fn interrupting_auto_poke_off_overnight_cancelled() -> &'static str { if_zh("Interrupting... Auto-poke OFF, overnight已取消", "Interrupting... Auto-poke OFF, overnight cancelled") }
pub fn interrupting_overnight_cancelled() -> &'static str { if_zh("Interrupting... Overnight已取消", "Interrupting... Overnight cancelled") }
pub fn jcode_account_management() -> &'static str { if_zh("", "Jcode account management") }
pub fn jcode_account_logging_out() -> &'static str { if_zh("", "Jcode account: logging out") }
pub fn jcode_account_requesting_browser_approval() -> &'static str { if_zh("", "Jcode account: requesting browser approval") }
pub fn judge_launch_failed() -> &'static str { if_zh("Judge launch失败", "Judge launch failed") }
pub fn judge_queued() -> &'static str { if_zh("Judge已排队", "Judge queued") }
pub fn loading_catch_up_sessions() -> &'static str { if_zh("正在加载Catch Up sessions...", "Loading Catch Up sessions...") }
pub fn loading_session() -> &'static str { if_zh("正在加载session...", "Loading session...") }
pub fn loading_session_re_requesting_history() -> &'static str { if_zh("", "Loading session… re-requesting history") }
pub fn local_shell_unavailable_in_remote_mode() -> &'static str { if_zh("Local shell 不可用 in remote mode", "Local shell unavailable in remote mode") }
pub fn login_api_base() -> &'static str { if_zh("正在Login: API base...", "Login: API base...") }
pub fn login_azure_api_key() -> &'static str { if_zh("正在Login: Azure API key...", "Login: Azure API key...") }
pub fn login_azure_auth_method() -> &'static str { if_zh("正在Login: Azure auth method...", "Login: Azure auth method...") }
pub fn login_azure_endpoint() -> &'static str { if_zh("正在Login: Azure endpoint...", "Login: Azure endpoint...") }
pub fn login_azure_model() -> &'static str { if_zh("正在Login: Azure model...", "Login: Azure model...") }
pub fn login_antigravity_waiting() -> &'static str { if_zh("正在Login: antigravity waiting...", "Login: antigravity waiting...") }
pub fn login_auto_import_failed() -> &'static str { if_zh("Login: auto import失败", "Login: auto import failed") }
pub fn login_choose_a_provider() -> &'static str { if_zh("登录：choose a provider", "Login: choose a provider") }
pub fn login_choose_sources_to_import() -> &'static str { if_zh("登录：choose sources to import", "Login: choose sources to import") }
pub fn login_copilot_device_flow() -> &'static str { if_zh("正在Login: copilot device flow...", "Login: copilot device flow...") }
pub fn login_exchanging() -> &'static str { if_zh("正在Login: exchanging...", "Login: exchanging...") }
pub fn login_failed() -> &'static str { if_zh("Login:失败", "Login: failed") }
pub fn login_importing_approved_sources() -> &'static str { if_zh("正在Login: importing approved sources...", "Login: importing approved sources...") }
pub fn login_importing_selected_logins() -> &'static str { if_zh("正在Login: importing selected logins...", "Login: importing selected logins...") }
pub fn login_no_external_imports_found() -> &'static str { if_zh("登录：no external imports found", "Login: no external imports found") }
pub fn login_opening_openai_sign_in_or() -> &'static str { if_zh("登录：opening OpenAI sign-in (or type /login for others)", "Login: opening OpenAI sign-in (or type /login for others)") }
pub fn login_paste_cursor_key() -> &'static str { if_zh("正在Login: paste cursor key...", "Login: paste cursor key...") }
pub fn login_waiting() -> &'static str { if_zh("正在Login: waiting...", "Login: waiting...") }
pub fn logout_failed() -> &'static str { if_zh("Logout失败", "Logout failed") }
pub fn logout_all_providers() -> &'static str { if_zh("登出: all providers", "Logout: all providers") }
pub fn logout_choose_a_provider() -> &'static str { if_zh("登出: choose a provider", "Logout: choose a provider") }
pub fn logout_completed_with_errors() -> &'static str { if_zh("登出: completed with errors", "Logout: completed with errors") }
pub fn mcp_all_connections_failed() -> &'static str { if_zh("MCP: all connections失败", "MCP: all connections failed") }
pub fn memory_off() -> &'static str { if_zh("Memory：关", "Memory: OFF") }
pub fn memory_on() -> &'static str { if_zh("Memory：开", "Memory: ON") }
pub fn merge_agent_failed_to_start() -> &'static str { if_zh("合并代理已启动", "Merge agent failed to start") }
pub fn merge_agent_launched() -> &'static str { if_zh("合并代理已启动", "Merge agent launched") }
pub fn migration_aborted() -> &'static str { if_zh("", "Migration aborted") }
pub fn model_favorites_unavailable_until_model_routes() -> &'static str { if_zh("Model favorites 不可用 until model routes finish loading", "Model favorites unavailable until model routes finish loading") }
pub fn model_list_refresh_failed() -> &'static str { if_zh("Model list refresh失败", "Model list refresh failed") }
pub fn model_list_update_failed() -> &'static str { if_zh("Model list update失败", "Model list update failed") }
pub fn model_list_updated() -> &'static str { if_zh("Model list已更新", "Model list updated") }
pub fn model_setup_will_retry_after_reconnect() -> &'static str { if_zh("", "Model setup will retry after reconnect") }
pub fn model_switch_failed() -> &'static str { if_zh("Model switch失败", "Model switch failed") }
pub fn model_switching_not_available() -> &'static str { if_zh("", "Model switching not available") }
pub fn model_unavailable() -> &'static str { if_zh("模型不可用", "Model unavailable") }
pub fn moving_tool_to_background() -> &'static str { if_zh("正在Moving tool to background...", "Moving tool to background...") }
pub fn next_prompt_new_session() -> &'static str { if_zh("", "Next prompt → new session") }
pub fn next_prompt_new_session_canceled() -> &'static str { if_zh("", "Next-prompt new session canceled") }
pub fn no_diagrams_to_open() -> &'static str { if_zh("没有diagrams to open", "No diagrams to open") }
pub fn no_favorited_models_yet_use_ctrl() -> &'static str { if_zh("没有favorited models yet. Use Ctrl+N to favorite one.", "No favorited models yet. Use Ctrl+N to favorite one.") }
pub fn no_image_in_clipboard() -> &'static str { if_zh("没有image in clipboard", "No image in clipboard") }
pub fn no_keybinding_conflicts_detected() -> &'static str { if_zh("没有keybinding conflicts detected", "No keybinding conflicts detected") }
pub fn no_logins_imported_press_enter_to() -> &'static str { if_zh("没有logins imported. Press Enter to choose a provider.", "No logins imported. Press Enter to choose a provider.") }
pub fn no_models_available() -> &'static str { if_zh("没有models available", "No models available") }
pub fn no_sessions_to_resume() -> &'static str { if_zh("没有sessions to resume", "No sessions to resume") }
pub fn no_supported_terminal_found_run_jcode() -> &'static str { if_zh("没有supported terminal found; run `jcode` manually", "No supported terminal found; run `jcode` manually") }
pub fn no_swarm_agent_selected() -> &'static str { if_zh("没有swarm agent selected", "No swarm agent selected") }
pub fn no_swarm_agents_to_open() -> &'static str { if_zh("没有swarm agents to open", "No swarm agents to open") }
pub fn no_terminal_available_for_merge_agent() -> &'static str { if_zh("没有terminal available for merge agent", "No terminal available for merge agent") }
pub fn no_text_or_image_in_clipboard() -> &'static str { if_zh("没有text or image in clipboard", "No text or image in clipboard") }
pub fn no_workspace_session_in_that_direction() -> &'static str { if_zh("没有workspace session in that direction", "No workspace session in that direction") }
pub fn nothing_to_undo() -> &'static str { if_zh("没有可撤销的操作", "Nothing to undo") }
pub fn nothing_visible_to_copy() -> &'static str { if_zh("没有可见内容可复制", "Nothing visible to copy") }
pub fn observe_off() -> &'static str { if_zh("Observe：关", "Observe: OFF") }
pub fn observe_on() -> &'static str { if_zh("Observe：开", "Observe: ON") }
pub fn onboarding_flow_already_active_can_t() -> &'static str { if_zh("", "Onboarding flow already active; can't start the simulator now") }
pub fn onboarding_preview_unavailable_while_busy() -> &'static str { if_zh("Onboarding preview 不可用 while busy", "Onboarding preview unavailable while busy") }
pub fn onboarding_preview_off() -> &'static str { if_zh("Onboarding preview：关", "Onboarding preview: off") }
pub fn onboarding_preview_on() -> &'static str { if_zh("Onboarding preview：开", "Onboarding preview: on") }
pub fn onboarding_simulator_off() -> &'static str { if_zh("Onboarding simulator：关", "Onboarding simulator: off") }
pub fn opened_swarm_prompt() -> &'static str { if_zh("已打开 swarm 提示", "Opened swarm prompt") }
pub fn overnight_auto_poke_complete() -> &'static str { if_zh("过夜日志", "Overnight auto-poke complete") }
pub fn overnight_auto_poke_finished() -> &'static str { if_zh("过夜日志", "Overnight auto-poke finished") }
pub fn overnight_auto_poke_stopped() -> &'static str { if_zh("过夜日志", "Overnight auto-poke stopped") }
pub fn overnight_cancel_requested() -> &'static str { if_zh("过夜日志", "Overnight cancel requested") }
pub fn overnight_log() -> &'static str { if_zh("过夜日志", "Overnight log") }
pub fn overnight_poke_stopped_non_retryable_error() -> &'static str { if_zh("过夜日志", "Overnight poke stopped: non-retryable error") }
pub fn overnight_queued_in_current_remote_session() -> &'static str { if_zh("过夜日志", "Overnight queued in current remote session") }
pub fn overnight_review_opened() -> &'static str { if_zh("过夜审查已打开", "Overnight review opened") }
pub fn overnight_started() -> &'static str { if_zh("过夜日志", "Overnight started") }
pub fn overnight_started_in_current_session() -> &'static str { if_zh("过夜日志", "Overnight started in current session") }
pub fn overnight_status() -> &'static str { if_zh("过夜日志", "Overnight status") }
pub fn overnight_stopped_errors() -> &'static str { if_zh("过夜日志", "Overnight stopped: errors") }
pub fn overnight_stopped_no_progress() -> &'static str { if_zh("过夜日志", "Overnight stopped: no progress") }
pub fn overnight_stopped_poke_budget() -> &'static str { if_zh("过夜日志", "Overnight stopped: poke budget") }
pub fn plan_proposal_received() -> &'static str { if_zh("", "Plan proposal received") }
pub fn poke_queued_after_current_turn() -> &'static str { if_zh("Poke 已排队 after current turn", "Poke queued after current turn") }
pub fn poke_stopped_non_retryable_error() -> &'static str { if_zh("", "Poke stopped: non-retryable error") }
pub fn poke_stopped_provider_guardrail() -> &'static str { if_zh("", "Poke stopped: provider guardrail") }
pub fn poke_off() -> &'static str { if_zh("Poke：关", "Poke: OFF") }
pub fn poke_on() -> &'static str { if_zh("Poke：开", "Poke: ON") }
pub fn premium_normal() -> &'static str { if_zh("Premium：正常", "Premium: normal") }
pub fn preparing_transfer() -> &'static str { if_zh("正在准备转移", "Preparing transfer") }
pub fn press_ctrl_c_again_to_quit() -> &'static str { if_zh("", "Press Ctrl+C again to quit") }
pub fn productivity_report_already_generating() -> &'static str { if_zh("", "Productivity report already generating…") }
pub fn productivity_report_failed() -> &'static str { if_zh("Productivity report失败", "Productivity report failed") }
pub fn productivity_scanning() -> &'static str { if_zh("", "Productivity → scanning") }
pub fn prompt_failed() -> &'static str { if_zh("Prompt失败", "Prompt failed") }
pub fn prompt_launch_failed() -> &'static str { if_zh("Prompt launch失败", "Prompt launch failed") }
pub fn prompt_launched_in_new_session() -> &'static str { if_zh("", "Prompt launched in new session") }
pub fn prompt_launching_in_new_session() -> &'static str { if_zh("", "Prompt launching in new session") }
pub fn prompt_restored_to_input_after_error() -> &'static str { if_zh("", "Prompt restored to input after error") }
pub fn prompt_session_created() -> &'static str { if_zh("Prompt session已创建", "Prompt session created") }
pub fn provider_switch_failed() -> &'static str { if_zh("Provider switch失败", "Provider switch failed") }
pub fn queued_test() -> &'static str { if_zh("", "Queued /test") }
pub fn queued_interleave_recovery_failed() -> &'static str { if_zh("Queued interleave recovery失败", "Queued interleave recovery failed") }
pub fn queued_prompt_failed() -> &'static str { if_zh("Queued prompt失败", "Queued prompt failed") }
pub fn rate_limited_queued_retry() -> &'static str { if_zh("Rate limited; 已排队 retry", "Rate limited; queued retry") }
pub fn rate_limited_queued_system_retry() -> &'static str { if_zh("Rate limited; 已排队 system retry", "Rate limited; queued system retry") }
pub fn reading_clipboard() -> &'static str { if_zh("正在Reading clipboard...", "Reading clipboard...") }
pub fn reasoning_effort_not_available_for_this() -> &'static str { if_zh("", "Reasoning effort not available for this provider") }
pub fn recovered_missing_tool_outputs() -> &'static str { if_zh("", "Recovered missing tool outputs") }
pub fn recovered_pending_prompts_after_reload() -> &'static str { if_zh("", "Recovered pending prompts after reload") }
pub fn recovered_queued_interleave_after_turn_finished() -> &'static str { if_zh("Recovered 已排队 interleave after turn finished", "Recovered queued interleave after turn finished") }
pub fn recovered_session() -> &'static str { if_zh("", "Recovered session") }
pub fn recovery_needed() -> &'static str { if_zh("需要恢复", "Recovery needed") }
pub fn refreshing_model_catalog() -> &'static str { if_zh("正在Refreshing model catalog...", "Refreshing model catalog...") }
pub fn refreshing_model_list() -> &'static str { if_zh("正在Refreshing model list...", "Refreshing model list...") }
pub fn reload_complete_prompt_preserved() -> &'static str { if_zh("", "Reload complete - prompt preserved") }
pub fn remote_protocol_error() -> &'static str { if_zh("", "Remote protocol error") }
pub fn renaming_session() -> &'static str { if_zh("正在Renaming session...", "Renaming session...") }
pub fn restored_queued_follow_up_after_reload() -> &'static str { if_zh("Restored 已排队 follow-up after reload", "Restored queued follow-up after reload") }
pub fn resuming_1_session() -> &'static str { if_zh("", "Resuming 1 session") }
pub fn review_launch_failed() -> &'static str { if_zh("Review launch失败", "Review launch failed") }
pub fn review_queued() -> &'static str { if_zh("Review已排队", "Review queued") }
pub fn running_test() -> &'static str { if_zh("", "Running /test") }
pub fn running_subagent() -> &'static str { if_zh("", "Running subagent") }
pub fn ssh_disconnected() -> &'static str { if_zh("", "SSH disconnected") }
pub fn ssh_setup_1_4_enter_target() -> &'static str { if_zh("", "SSH setup 1/4: enter target") }
pub fn ssh_setup_2_4_login_terminal() -> &'static str { if_zh("", "SSH setup 2/4: login terminal opened") }
pub fn ssh_setup_cancelled() -> &'static str { if_zh("SSH setup已取消", "SSH setup cancelled") }
pub fn selection_is_empty() -> &'static str { if_zh("选择内容为空", "Selection is empty") }
pub fn selection_ready_enter_y_c_to() -> &'static str { if_zh("选择内容为空", "Selection ready · Enter/Y/C to copy · Esc to cancel") }
pub fn self_dev() -> &'static str { if_zh("自开发", "Self-dev") }
pub fn self_dev_status() -> &'static str { if_zh("自开发", "Self-dev status") }
pub fn server_auto_reload_paused_possible_loop() -> &'static str { if_zh("", "Server auto-reload paused (possible loop)") }
pub fn server_still_busy_follow_up_stays() -> &'static str { if_zh("Server still busy; follow-up stays已排队", "Server still busy; follow-up stays queued") }
pub fn server_update_available() -> &'static str { if_zh("", "Server update available") }
pub fn server_update_available_auto_reload_failed() -> &'static str { if_zh("Server update available - auto reload失败", "Server update available - auto reload failed") }
pub fn server_update_available_manual_reload_recommended() -> &'static str { if_zh("", "Server update available - manual /reload recommended") }
pub fn session_cleared() -> &'static str { if_zh("Session已清除", "Session cleared") }
pub fn session_history_not_loading_try_restart() -> &'static str { if_zh("", "Session history not loading - try /restart") }
pub fn session_load_failed() -> &'static str { if_zh("Session load失败", "Session load failed") }
pub fn session_name_cleared() -> &'static str { if_zh("Session name已清除", "Session name cleared") }
pub fn session_renamed() -> &'static str { if_zh("", "Session renamed") }
pub fn session_saved() -> &'static str { if_zh("Session已保存", "Session saved") }
pub fn sessions_loaded() -> &'static str { if_zh("Sessions已加载", "Sessions loaded") }
pub fn shell_command_is_empty() -> &'static str { if_zh("", "Shell command is empty") }
pub fn side_panel_off() -> &'static str { if_zh("Side panel：关", "Side panel: OFF") }
pub fn split_view() -> &'static str { if_zh("分屏视图", "Split view") }
pub fn split_view_off() -> &'static str { if_zh("Split view：关", "Split view: OFF") }
pub fn split_view_on() -> &'static str { if_zh("Split view：开", "Split view: ON") }
pub fn starting_background_rebuild() -> &'static str { if_zh("正在Starting background rebuild...", "Starting background rebuild...") }
pub fn startup_prompt_failed() -> &'static str { if_zh("Startup prompt失败", "Startup prompt failed") }
pub fn startup_prompt_queued() -> &'static str { if_zh("Startup prompt已排队", "Startup prompt queued") }
pub fn stopped_model_endpoint_mismatch() -> &'static str { if_zh("", "Stopped: model/endpoint mismatch") }
pub fn stopped_repeated_auth_failures() -> &'static str { if_zh("", "Stopped: repeated auth failures") }
pub fn subagent_model_inherit() -> &'static str { if_zh("", "Subagent model: inherit") }
pub fn subscribe_login_jcode_to_start() -> &'static str { if_zh("", "Subscribe: /login jcode to start") }
pub fn swarm_page_alt_n_chat_alt() -> &'static str { if_zh("", "Swarm page: alt+n chat · alt+↑/↓ select · alt+o open · esc") }
pub fn swarm_view_closed() -> &'static str { if_zh("", "Swarm view closed") }
pub fn swarm_off() -> &'static str { if_zh("Swarm：关", "Swarm: OFF") }
pub fn swarm_on() -> &'static str { if_zh("Swarm：开", "Swarm: ON") }
pub fn swarm_alt_n_full_page_alt() -> &'static str { if_zh("", "Swarm: alt+n full page · alt+↑/↓ select · alt+o open · esc") }
pub fn this_command_requires_a_live_connection() -> &'static str { if_zh("", "This command requires a live connection") }
pub fn todos_card() -> &'static str { if_zh("待办卡片", "Todos card") }
pub fn todos_card_dismissed() -> &'static str { if_zh("待办卡片", "Todos card dismissed") }
pub fn todos_panel_off() -> &'static str { if_zh("Todos panel：关", "Todos panel: OFF") }
pub fn todos_panel_on() -> &'static str { if_zh("Todos panel：开", "Todos panel: ON") }
pub fn transcript_appended() -> &'static str { if_zh("转录为空", "Transcript appended") }
pub fn transcript_failed() -> &'static str { if_zh("Transcript失败", "Transcript failed") }
pub fn transcript_inserted() -> &'static str { if_zh("转录为空", "Transcript inserted") }
pub fn transcript_opened() -> &'static str { if_zh("转录为空", "Transcript opened") }
pub fn transcript_path() -> &'static str { if_zh("转录路径", "Transcript path") }
pub fn transcript_replaced_input() -> &'static str { if_zh("转录为空", "Transcript replaced input") }
pub fn transcript_was_empty() -> &'static str { if_zh("转录为空", "Transcript was empty") }
pub fn transfer_already_pending() -> &'static str { if_zh("", "Transfer already pending") }
pub fn transfer_failed() -> &'static str { if_zh("Transfer失败", "Transfer failed") }
pub fn transfer_launch_failed() -> &'static str { if_zh("Transfer launch失败", "Transfer launch failed") }
pub fn transfer_launched() -> &'static str { if_zh("转移已启动", "Transfer launched") }
pub fn transfer_open_failed() -> &'static str { if_zh("Transfer open失败", "Transfer open failed") }
pub fn transfer_queue_failed() -> &'static str { if_zh("Transfer queue失败", "Transfer queue failed") }
pub fn transfer_queued_after_current_turn() -> &'static str { if_zh("Transfer 已排队 after current turn", "Transfer queued after current turn") }
pub fn transfer_session_created() -> &'static str { if_zh("Transfer session已创建", "Transfer session created") }
pub fn try_a_suggestion_or_type_anything() -> &'static str { if_zh("", "Try a suggestion, or type anything to start") }
pub fn undoing_rewind() -> &'static str { if_zh("正在Undoing rewind...", "Undoing rewind...") }
pub fn updating_model_list() -> &'static str { if_zh("", "Updating model list…") }
pub fn updating_model_routes() -> &'static str { if_zh("", "Updating model routes…") }
pub fn usage_no_connected_providers() -> &'static str { if_zh("", "Usage → no connected providers") }
pub fn usage_refreshing() -> &'static str { if_zh("", "Usage → refreshing") }
pub fn usage_showing_cached_data_refreshing() -> &'static str { if_zh("", "Usage → showing cached data, refreshing") }
pub fn usage_updated() -> &'static str { if_zh("Usage →已更新", "Usage → updated") }
pub fn visual_debug_off() -> &'static str { if_zh("Visual debug：关", "Visual debug: OFF") }
pub fn visual_debug_on() -> &'static str { if_zh("Visual debug：开", "Visual debug: ON") }
pub fn waiting_for_reload_handoff() -> &'static str { if_zh("正在Waiting for reload handoff...", "Waiting for reload handoff...") }
pub fn workspace_add_queued() -> &'static str { if_zh("Workspace add已排队", "Workspace add queued") }
pub fn workspace_mode_disabled() -> &'static str { if_zh("工作区迁移", "Workspace mode disabled") }
pub fn workspace_mode_enabled() -> &'static str { if_zh("工作区迁移", "Workspace mode enabled") }
pub fn you_re_all_set_type_anything() -> &'static str { if_zh("", "You're all set, type anything to start") }
pub fn new_terminal_opened() -> &'static str { if_zh("", "↗ New terminal opened") }
pub fn input_restored() -> &'static str { if_zh("", "↶ Input restored") }
pub fn interactive_terminal_detected_command_will_timeout() -> &'static str { if_zh("", "⌨ Interactive terminal detected (command will timeout)") }
pub fn interleave_sent() -> &'static str { if_zh("", "⏭ Interleave sent") }
pub fn sending_now_interleave() -> &'static str { if_zh("", "⏭ Sending now (interleave)") }
pub fn cut_input_line() -> &'static str { if_zh("", "✂ Cut input line") }
pub fn dictation_running_press_again_to_stop() -> &'static str { if_zh("", "🎙 Dictation running - press again to stop") }
pub fn stopping_dictation() -> &'static str { if_zh("正在🎙 Stopping dictation...", "🎙 Stopping dictation...") }
pub fn input_restored_from_stash() -> &'static str { if_zh("", "📋 Input restored from stash") }
pub fn input_stashed() -> &'static str { if_zh("", "📋 Input stashed") }
pub fn swapped_input_with_stash() -> &'static str { if_zh("", "📋 Swapped input with stash") }
pub fn bookmark_set_press_again_to_return() -> &'static str { if_zh("", "📌 Bookmark set - press again to return") }

// -- Auth login/logout --
pub fn logout_anthropic_api_key() -> &'static str { if_zh("已登出 Anthropic API 密钥。", "Logged out of Anthropic API key.") }
pub fn logout_openai_api_key() -> &'static str { if_zh("已登出 OpenAI API 密钥。", "Logged out of OpenAI API key.") }
pub fn logout_openrouter_api_key() -> &'static str { if_zh("已登出 OpenRouter API 密钥。", "Logged out of OpenRouter API key.") }
pub fn logout_bedrock_api_key() -> &'static str { if_zh("已登出 Bedrock API 密钥。", "Logged out of Bedrock API key.") }
pub fn logout_cursor_api_key() -> &'static str { if_zh("已登出 Cursor API 密钥。", "Logged out of Cursor API key.") }
pub fn logout_gemini_api_key() -> &'static str { if_zh("已登出 Gemini API 密钥。", "Logged out of Gemini API key.") }
pub fn logout_copilot() -> &'static str { if_zh("已登出 Copilot。", "Logged out of Copilot.") }
pub fn logout_openai_accounts(count: usize) -> String {
    match current_language() {
        Language::Zh => format!("已登出 {} 个 OpenAI 账户。", count),
        Language::En => format!("Logged out of {} OpenAI account(s).", count),
    }
}
pub fn logout_anthropic_accounts(count: usize) -> String {
    match current_language() {
        Language::Zh => format!("已登出 {} 个 Anthropic 账户。", count),
        Language::En => format!("Logged out of {} Anthropic account(s).", count),
    }
}
pub fn returned_to_bookmark() -> &'static str { if_zh("", "📌 Returned to bookmark") }

// -- More auth functions --
pub fn logout_gemini() -> &'static str { if_zh("已登出 Gemini。", "Logged out of Gemini.") }
pub fn logout_api_key(provider: &str) -> String {
    match current_language() {
        Language::Zh => format!("已登出 {} API 密钥。", provider),
        Language::En => format!("Logged out of {} API key.", provider),
    }
}
pub fn logout_not_automated(display: &str, id: &str) -> String {
    match current_language() {
        Language::Zh => format!("{} 的登出尚未自动化。请从 /account {} 设置中移除其已保存的 API 密钥或外部 CLI 会话。", display, id),
        Language::En => format!("Logout for {} is not automated yet. Remove its saved API key or external CLI session from /account {} settings.", display, id),
    }
}
pub fn logged_out_of_summary(summary: &str) -> String {
    match current_language() {
        Language::Zh => format!("已登出：{}。", summary),
        Language::En => format!("Logged out of: {}.", summary),
    }
}
