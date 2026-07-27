use super::{
    accent_color, ai_color, ai_text, asap_color, clear_area, dim_color, get_grouped_changelog,
    header_icon_color, header_name_color, header_session_color, pending_color, queued_color,
    record_chat_overlay_copy_snapshot, rgb, tool_color, user_bg, user_color, user_text,
};
use crate::tui::TuiState;
use crate::tui::info_widget::WidgetPlacement;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use super::selection_highlight::highlight_line_selection;

pub(super) fn draw_changelog_overlay(
    frame: &mut Frame,
    area: Rect,
    scroll: usize,
    app: &dyn TuiState,
) {
    clear_area(frame, area);

    let groups = get_grouped_changelog();
    let mut lines: Vec<Line<'static>> = Vec::new();

    if groups.is_empty() {
        lines.push(Line::from(Span::styled(
            "No changelog entries available.",
            Style::default().fg(dim_color()),
        )));
    } else {
        for group in &groups {
            let heading = match &group.released_at {
                Some(released_at) => format!("  {} · {}", group.version, released_at),
                None => format!("  {}", group.version),
            };
            lines.push(Line::from(Span::styled(
                heading,
                Style::default()
                    .fg(rgb(200, 200, 220))
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));
            for entry in &group.entries {
                lines.push(Line::from(vec![
                    Span::styled("    • ", Style::default().fg(dim_color())),
                    Span::styled(entry.clone(), Style::default().fg(rgb(170, 170, 185))),
                ]));
            }
            lines.push(Line::from(""));
        }
    }

    let total_lines = lines.len();
    let visible_height = area.height.saturating_sub(2) as usize;
    let max_scroll = total_lines.saturating_sub(visible_height);
    let scroll = scroll.min(max_scroll);

    let scroll_info = if total_lines > visible_height {
        let pct = if max_scroll > 0 {
            (scroll * 100) / max_scroll
        } else {
            100
        };
        format!(" {}% ", pct)
    } else {
        String::new()
    };

    let title = format!(" Changelog {} ", scroll_info);
    let block = Block::default()
        .title(Span::styled(
            title,
            Style::default()
                .fg(rgb(200, 200, 220))
                .add_modifier(Modifier::BOLD),
        ))
        .title_bottom(Line::from(Span::styled(
            " Esc to close · drag to select, release to copy · wheel/j/k scroll ",
            Style::default().fg(dim_color()),
        )))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(dim_color()));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width == 0 || inner.height == 0 {
        return;
    }

    let visible_end = scroll
        .saturating_add(inner.height as usize)
        .min(total_lines);

    // Register the rendered lines so the shared copy-selection machinery can map
    // mouse drags to text and highlight + copy the selection, exactly like the
    // chat viewport. Without this, mouse capture would block native terminal
    // selection and there would be no way to copy from the overlay.
    record_chat_overlay_copy_snapshot(&lines, scroll, visible_end, inner);

    let mut visible_lines: Vec<Line<'static>> =
        lines.get(scroll..visible_end).unwrap_or(&[]).to_vec();

    if let Some(range) = app.copy_selection_range().filter(|range| {
        range.start.pane == crate::tui::CopySelectionPane::Chat
            && range.end.pane == crate::tui::CopySelectionPane::Chat
    }) {
        let (start, end) = if (range.start.abs_line, range.start.column)
            <= (range.end.abs_line, range.end.column)
        {
            (range.start, range.end)
        } else {
            (range.end, range.start)
        };
        for abs_idx in start.abs_line.max(scroll)..=end.abs_line.min(visible_end.saturating_sub(1))
        {
            let rel_idx = abs_idx.saturating_sub(scroll);
            if let Some(line) = visible_lines.get_mut(rel_idx) {
                let start_col = if abs_idx == start.abs_line {
                    start.column
                } else {
                    0
                };
                let end_col = if abs_idx == end.abs_line {
                    end.column
                } else {
                    line.width()
                };
                *line = highlight_line_selection(line, start_col, end_col);
            }
        }
    }

    frame.render_widget(Paragraph::new(visible_lines), inner);
}

pub(super) fn draw_help_overlay(frame: &mut Frame, area: Rect, scroll: usize, app: &dyn TuiState) {
    clear_area(frame, area);

    let section_style = Style::default()
        .fg(accent_color())
        .add_modifier(Modifier::BOLD);
    let cmd_style = Style::default().fg(rgb(230, 230, 240));
    let desc_style = Style::default().fg(rgb(150, 150, 165));
    let key_style = Style::default().fg(rgb(200, 180, 120));
    let sep_style = Style::default().fg(rgb(50, 50, 55));

    let mut lines: Vec<Line<'static>> = Vec::new();

    let separator = || -> Line<'static> {
        Line::from(Span::styled(
            "  ─────────────────────────────────────────────────",
            sep_style,
        ))
    };

    let help_entry = |cmd: &str, desc: &str| -> Line<'static> {
        Line::from(vec![
            Span::styled("    ", Style::default()),
            Span::styled(cmd.to_string(), cmd_style),
            Span::styled("  ", Style::default()),
            Span::styled(desc.to_string(), desc_style),
        ])
    };

    let key_entry = |key: &str, desc: &str| -> Line<'static> {
        Line::from(vec![
            Span::styled("    ", Style::default()),
            Span::styled(format!("{:<22}", key), key_style),
            Span::styled(desc.to_string(), desc_style),
        ])
    };

    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(crate::tui::i18n::help_section_commands(), section_style)));
    lines.push(Line::from(""));
    lines.push(help_entry("/help", crate::tui::i18n::help_entry_help()));
    lines.push(help_entry(
        "/help <command>",
        crate::tui::i18n::help_entry_help_command(),
    ));
    lines.push(help_entry("/model", crate::tui::i18n::help_entry_model()));
    lines.push(help_entry("/model <name>", crate::tui::i18n::help_entry_model_switch()));
    lines.push(help_entry(
        "/provider-test-coverage",
        crate::tui::i18n::help_entry_provider_test(),
    ));
    lines.push(help_entry("/agents", crate::tui::i18n::help_entry_agents()));
    lines.push(help_entry(
        "/swarm-prompt",
        crate::tui::i18n::help_entry_swarm_prompt(),
    ));
    lines.push(help_entry(
        "/effort <level>",
        crate::tui::i18n::help_entry_effort(),
    ));
    lines.push(help_entry(
        "/fast [on|off|status|default ...]",
        crate::tui::i18n::help_entry_fast(),
    ));
    lines.push(help_entry(
        "/transport <mode>",
        crate::tui::i18n::help_entry_transport(),
    ));
    lines.push(help_entry(
        "/alignment [status|centered|left]",
        crate::tui::i18n::help_entry_alignment(),
    ));
    lines.push(help_entry(
        "/compact-notifications [status|on|off]",
        crate::tui::i18n::help_entry_compact_notifications(),
    ));
    lines.push(help_entry(
        "/show-agentgrep-output [status|on|off]",
        crate::tui::i18n::help_entry_show_agentgrep(),
    ));
    lines.push(help_entry("/config", crate::tui::i18n::help_entry_config()));
    lines.push(help_entry("/config init", crate::tui::i18n::help_entry_config_init()));
    lines.push(help_entry("/config edit", crate::tui::i18n::help_entry_config_edit()));
    lines.push(help_entry("/dictate", crate::tui::i18n::help_entry_dictate()));
    lines.push(help_entry(
        "/git [status]",
        crate::tui::i18n::help_entry_git(),
    ));
    lines.push(help_entry(
        "/context",
        crate::tui::i18n::help_entry_context(),
    ));
    lines.push(help_entry(
        "/skills",
        crate::tui::i18n::help_entry_skills(),
    ));
    lines.push(help_entry("/info", crate::tui::i18n::help_entry_info()));
    lines.push(help_entry(
        "/keys",
        crate::tui::i18n::help_entry_keys(),
    ));
    lines.push(help_entry("/usage", crate::tui::i18n::help_entry_usage()));
    lines.push(help_entry(
        "/support",
        crate::tui::i18n::help_entry_support(),
    ));
    lines.push(help_entry("/version", crate::tui::i18n::help_entry_version()));
    lines.push(help_entry(
        "/changelog",
        crate::tui::i18n::help_entry_changelog(),
    ));

    lines.push(Line::from(""));
    lines.push(separator());
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(crate::tui::i18n::help_section_session(), section_style)));
    lines.push(Line::from(""));
    lines.push(help_entry("/clear", crate::tui::i18n::help_entry_clear()));
    lines.push(help_entry(
        "/compact",
        "Summarize old messages to free context",
    ));
    lines.push(help_entry(
        "/rewind",
        "Show numbered history, /rewind N to rewind",
    ));
    lines.push(help_entry(
        "/fix",
        "Attempt recovery when model cannot continue",
    ));
    lines.push(help_entry(
        "/poke",
        "Poke model to resume with incomplete todos (on/off/status)",
    ));
    lines.push(help_entry(
        "/plan [goal]",
        "Draft a plan-only proposal as a plan card (no edits)",
    ));
    lines.push(help_entry(
        "/improve",
        "Autonomously improve the repo until returns diminish",
    ));
    lines.push(help_entry(
        "/improve resume",
        "Resume the last saved improve loop/plan",
    ));
    lines.push(help_entry(
        "/refactor",
        "Run a safe refactor loop with independent review",
    ));
    lines.push(help_entry(
        "/refactor resume",
        "Resume the last saved refactor loop/plan",
    ));
    lines.push(help_entry(
        "/splitview [on|off|status]",
        "Mirror the current chat in the side panel",
    ));
    lines.push(help_entry(
        "/fork [prompt]",
        "Fork session into a new window (alias: /split)",
    ));
    lines.push(help_entry(
        "/transfer",
        "Open a fresh session with only compacted context + copied todos",
    ));
    lines.push(help_entry(
        "/workspace [status|on|off|add]",
        crate::tui::i18n::help_entry_windmill(),
    ));
    lines.push(help_entry(
        "/catchup [next|list]",
        crate::tui::i18n::help_entry_catch_up(),
    ));
    lines.push(help_entry(
        "/back",
        "Return to the previous Catch Up source session",
    ));
    lines.push(help_entry("/resume", crate::tui::i18n::help_entry_resume()));
    lines.push(help_entry(
        "/active",
        crate::tui::i18n::help_entry_notifications(),
    ));
    lines.push(help_entry(
        "/catchup [next]",
        "Jump into finished sessions with a side-panel brief",
    ));
    lines.push(help_entry(
        "/back",
        "Return to the previous Catch Up session",
    ));
    lines.push(help_entry("/save [label]", crate::tui::i18n::help_entry_save()));
    lines.push(help_entry(
        "/rename <name>|--clear",
        "Set or clear current session name",
    ));
    lines.push(help_entry(
        "/unsave",
        "Remove bookmark from current session",
    ));

    lines.push(Line::from(""));
    lines.push(separator());
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(crate::tui::i18n::help_section_memory_swarm(), section_style)));
    lines.push(Line::from(""));
    lines.push(help_entry("/memory [on|off]", crate::tui::i18n::help_entry_memory()));
    lines.push(help_entry(
        "/test [claim]",
        "Run layered verification and produce proof",
    ));
    lines.push(help_entry(
        "/initiatives",
        "Open initiatives overview / resume an initiative",
    ));
    lines.push(help_entry("/swarm [on|off]", crate::tui::i18n::help_entry_swarm()));

    lines.push(Line::from(""));
    lines.push(separator());
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(crate::tui::i18n::help_section_auth_accounts(), section_style)));
    lines.push(Line::from(""));
    lines.push(help_entry("/auth", crate::tui::i18n::help_entry_auth()));
    lines.push(help_entry(
        "/login [provider]",
        crate::tui::i18n::help_entry_login(),
    ));
    lines.push(help_entry(
        "/account",
        "Open combined Claude/OpenAI account picker",
    ));
    lines.push(help_entry(
        "/subscription",
        "Inspect jcode subscription scaffold",
    ));
    lines.push(help_entry(
        "/subscribe",
        "Why and how to subscribe to jcode",
    ));

    lines.push(Line::from(""));
    lines.push(separator());
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(crate::tui::i18n::help_section_system(), section_style)));
    lines.push(Line::from(""));
    lines.push(help_entry("/reload", crate::tui::i18n::help_entry_reload()));
    lines.push(help_entry(
        "/restart",
        "Restart with current binary (no build)",
    ));
    lines.push(help_entry(
        "/rebuild",
        "Full update (git pull + build + tests)",
    ));
    if app.is_remote_mode() {
        lines.push(help_entry("/client-reload", crate::tui::i18n::help_entry_client_reload()));
        lines.push(help_entry("/server-reload", crate::tui::i18n::help_entry_server_reload()));
        lines.push(help_entry(
            "/continue",
            crate::tui::i18n::help_entry_continue_all(),
        ));
    }
    lines.push(help_entry(
        "/debug-visual",
        "Enable visual debugging for TUI issues",
    ));
    lines.push(help_entry("/quit", crate::tui::i18n::help_entry_quit()));

    let skills = app.available_skills();
    if !skills.is_empty() {
        lines.push(Line::from(""));
        lines.push(separator());
        lines.push(Line::from(""));

        lines.push(Line::from(Span::styled(crate::tui::i18n::help_section_skills(), section_style)));
        lines.push(Line::from(""));
        for skill in &skills {
            lines.push(help_entry(&format!("/{}", skill), crate::tui::i18n::help_skill_activate()));
        }
    }

    lines.push(Line::from(""));
    lines.push(separator());
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(crate::tui::i18n::help_section_navigation(), section_style)));
    lines.push(Line::from(""));
    lines.push(key_entry("PageUp / PageDown", crate::tui::i18n::help_key_scroll_history()));
    lines.push(key_entry("Up / Down", crate::tui::i18n::help_key_scroll_input()));
    lines.push(key_entry(
        "Ctrl+J / Ctrl+K",
        "Jump to next / previous user prompt (also Ctrl+] / Ctrl+[)",
    ));
    lines.push(key_entry(
        "Ctrl+Shift+J / Ctrl+Shift+K",
        "Scroll history down / up one line",
    ));
    lines.push(key_entry(
        "Cmd/Super+K / J",
        "Jump to previous / next user prompt (macOS, if forwarded)",
    ));
    lines.push(key_entry("Ctrl+1..4", crate::tui::i18n::help_key_resize_panel_25()));
    lines.push(key_entry(
        "Ctrl+5..9",
        "Jump by recency (5 = 5th most recent)",
    ));

    lines.push(Line::from(""));
    lines.push(separator());
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(
        "  Diagrams & Diffs",
        section_style,
    )));
    lines.push(Line::from(""));
    lines.push(key_entry(
        crate::tui::keybind::side_panel_toggle_key_label(),
        "Toggle side panel (or diagram pane if empty)",
    ));
    lines.push(key_entry("Alt+T", crate::tui::i18n::help_key_toggle_diagram()));
    lines.push(key_entry(
        "Alt+Shift+I",
        "Show/hide inline images (persists)",
    ));
    lines.push(key_entry("Ctrl+H / Ctrl+L", crate::tui::i18n::help_key_focus_chat()));
    lines.push(key_entry(
        "Ctrl+Left / Right",
        "Cycle diagrams (when diagram focused)",
    ));
    lines.push(key_entry("h/j/k/l / arrows", crate::tui::i18n::help_key_pan_diagram()));
    lines.push(key_entry("[ / ]", crate::tui::i18n::help_key_zoom_diagram()));
    lines.push(key_entry("+ / -", crate::tui::i18n::help_key_resize_diagram()));
    lines.push(key_entry(
        "Alt+G / /diff",
        "Cycle diff mode (Off/Inline/Pinned/File)",
    ));
    lines.push(key_entry("Shift+Tab", crate::tui::i18n::help_key_cycle_favorites()));
    lines.push(key_entry("Ctrl+O", crate::tui::i18n::help_key_set_default()));
    lines.push(key_entry(
        "Ctrl+N",
        "Toggle favorite model (in /model picker)",
    ));

    lines.push(Line::from(""));
    lines.push(separator());
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(crate::tui::i18n::help_section_input_editing(), section_style)));
    lines.push(Line::from(""));
    lines.push(key_entry(
        "Ctrl+C / Ctrl+D",
        "Quit (press twice to confirm)",
    ));
    lines.push(key_entry("Ctrl+X", crate::tui::i18n::help_key_cut_line()));
    lines.push(key_entry(
        "Ctrl+A",
        "Copy visible chat viewport plus nearby context",
    ));
    lines.push(key_entry("Ctrl+U", crate::tui::i18n::help_key_clear_input()));
    lines.push(key_entry("Ctrl+K", crate::tui::i18n::help_key_delete_to_end()));
    lines.push(key_entry(
        "Alt+Backspace / Alt+Delete",
        crate::tui::i18n::help_key_delete_word(),
    ));
    lines.push(key_entry(
        "Cmd/Super+Backspace / Delete",
        crate::tui::i18n::help_key_delete_word(),
    ));
    if cfg!(target_os = "macos") {
        // On macOS, Cmd+Left/Right default to effort cycling; Home/End and
        // Cmd+A/E still jump to the start/end of the input.
        lines.push(key_entry("Home / End", crate::tui::i18n::help_key_home_end()));
    } else {
        lines.push(key_entry(
            "Cmd/Super+Left / Right",
            crate::tui::i18n::help_key_home_end(),
        ));
    }
    lines.push(key_entry("Cmd/Super+Z", crate::tui::i18n::help_key_undo_edit()));
    lines.push(key_entry("Cmd/Super+X / V", crate::tui::i18n::help_key_cut_paste()));
    lines.push(key_entry("Ctrl+S", crate::tui::i18n::help_key_stash_pop()));
    lines.push(key_entry("Ctrl+Backspace", crate::tui::i18n::help_key_delete_word()));
    lines.push(key_entry("Ctrl+B / Ctrl+F", crate::tui::i18n::help_key_move_word_lr()));
    lines.push(key_entry("Ctrl+Left / Right", crate::tui::i18n::help_key_move_word_lr()));
    lines.push(key_entry(
        "Shift+Enter / Alt+Enter",
        "Insert newline in input",
    ));
    lines.push(key_entry(
        "Ctrl+Enter / Cmd+Enter",
        "Use opposite send mode while processing",
    ));
    lines.push(key_entry("Ctrl+Up", crate::tui::i18n::help_key_retrieve_pending()));
    lines.push(key_entry("Ctrl+Tab / Ctrl+T", crate::tui::i18n::help_key_toggle_queue()));
    lines.push(key_entry("Ctrl+R", crate::tui::i18n::help_key_recover_tools()));
    lines.push(key_entry(
        "Ctrl+V / Alt+V",
        "Paste clipboard (text or image)",
    ));
    lines.push(key_entry(
        "Alt+A",
        "Quick-copy visible chat viewport plus nearby context",
    ));
    lines.push(key_entry("Alt+Y", crate::tui::i18n::help_key_toggle_chat_select()));
    lines.push(key_entry("Alt+S", crate::tui::i18n::help_key_toggle_scroll_lock()));
    lines.push(key_entry("Ctrl+P", crate::tui::i18n::help_key_toggle_auto_poke()));
    lines.push(key_entry("Alt+X", crate::tui::i18n::help_key_toggle_todo_card()));
    lines.push(key_entry(
        &crate::tui::keybind::effort_switch_keys_label(),
        "Cycle effort (reasoning + swarm)",
    ));
    if cfg!(target_os = "macos") {
        lines.push(key_entry(
            "Alt+Left / Right",
            "Move by word in input (also Alt+B / Alt+F)",
        ));
    }
    if let Some(label) = app.dictation_key_label() {
        lines.push(key_entry(&label, "Run configured dictation"));
    }
    if let Some(label) = crate::tui::keybind::load_open_resume_key().label {
        lines.push(key_entry(&label, "Open the /resume session picker"));
    }
    if let Some(label) = crate::tui::keybind::load_new_terminal_key().label {
        lines.push(key_entry(
            &label,
            "Spawn new jcode session in a new terminal",
        ));
    }

    lines.push(Line::from(""));

    let total_lines = lines.len();
    let visible_height = area.height.saturating_sub(2) as usize;
    let max_scroll = total_lines.saturating_sub(visible_height);
    let scroll = scroll.min(max_scroll);

    let scroll_info = if total_lines > visible_height {
        let pct = if max_scroll > 0 {
            (scroll * 100) / max_scroll
        } else {
            100
        };
        format!(" {}% ", pct)
    } else {
        String::new()
    };

    let title = format!(" Help {} ", scroll_info);
    let block = Block::default()
        .title(Span::styled(
            title,
            Style::default()
                .fg(rgb(200, 200, 220))
                .add_modifier(Modifier::BOLD),
        ))
        .title_bottom(Line::from(Span::styled(
            " Esc to close · mouse wheel/j/k scroll · Space/PageUp page · /help <cmd> for details ",
            Style::default().fg(dim_color()),
        )))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(dim_color()));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .scroll((scroll as u16, 0));

    frame.render_widget(paragraph, area);
}

pub(super) fn draw_model_status_overlay(
    frame: &mut Frame,
    area: Rect,
    scroll: usize,
    content: &str,
) {
    clear_area(frame, area);

    let title_style = Style::default()
        .fg(accent_color())
        .add_modifier(Modifier::BOLD);
    let text_style = Style::default().fg(rgb(210, 210, 220));
    let dim_style = Style::default().fg(dim_color());

    let mut lines: Vec<Line<'static>> = Vec::new();
    lines.push(Line::from(Span::styled("  Model Status", title_style)));
    lines.push(Line::from(Span::styled(
        "  Live verification evidence for provider/model behavior in jcode",
        dim_style,
    )));
    lines.push(Line::from(""));

    for raw in content.lines() {
        if let Some(title) = raw.strip_prefix("# ") {
            lines.push(Line::from(Span::styled(format!("  {title}"), title_style)));
        } else if let Some(title) = raw.strip_prefix("## ") {
            lines.push(Line::from(Span::styled(format!("  {title}"), title_style)));
        } else if raw.trim().is_empty() {
            lines.push(Line::from(""));
        } else {
            lines.push(Line::from(Span::styled(
                format!("  {raw}"),
                model_status_line_style(raw, text_style),
            )));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  ↑/↓ scroll, PgUp/PgDn page, c copy report, q/Esc close",
        dim_style,
    )));

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" /provider-test-coverage "),
        )
        .scroll((scroll.min(u16::MAX as usize) as u16, 0));
    frame.render_widget(paragraph, area);
}

fn model_status_line_style(raw: &str, default: Style) -> Style {
    // Reuse the same semantic classification the CLI uses so the TUI overlay
    // and `jcode provider-test-coverage` stay color-consistent.
    use crate::live_tests::CoverageLineStyle;
    match crate::live_tests::classify_provider_test_coverage_line(raw) {
        CoverageLineStyle::Title => Style::default()
            .fg(accent_color())
            .add_modifier(Modifier::BOLD),
        CoverageLineStyle::Pass => Style::default().fg(rgb(120, 220, 150)),
        CoverageLineStyle::Fail => Style::default().fg(rgb(240, 110, 110)),
        CoverageLineStyle::Warn => Style::default().fg(rgb(235, 190, 105)),
        CoverageLineStyle::Dim => Style::default().fg(dim_color()),
        CoverageLineStyle::Plain => default,
    }
}

pub(super) fn draw_debug_overlay(
    frame: &mut Frame,
    placements: &[WidgetPlacement],
    chunks: &[Rect],
) {
    if chunks.len() < 5 {
        return;
    }
    render_overlay_box(frame, chunks[0], "messages", Color::Red);
    render_overlay_box(frame, chunks[1], "queued", Color::Yellow);
    render_overlay_box(frame, chunks[2], "status", Color::Cyan);
    render_overlay_box(frame, chunks[3], "picker", Color::Magenta);
    render_overlay_box(frame, chunks[4], "input", Color::Green);
    if chunks.len() > 5 && chunks[5].height > 0 {
        render_overlay_box(frame, chunks[5], "donut", Color::Blue);
    }

    for placement in placements {
        let title = format!("widget:{}", placement.kind.as_str());
        render_overlay_box(frame, placement.rect, &title, Color::Magenta);
    }
}

fn render_overlay_box(frame: &mut Frame, area: Rect, title: &str, color: Color) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .title(Span::styled(title.to_string(), Style::default().fg(color)));
    frame.render_widget(block, area);
}

pub(super) fn debug_palette_json() -> Option<serde_json::Value> {
    Some(serde_json::json!({
        "user_color": color_to_rgb(user_color()),
        "ai_color": color_to_rgb(ai_color()),
        "tool_color": color_to_rgb(tool_color()),
        "dim_color": color_to_rgb(dim_color()),
        "accent_color": color_to_rgb(accent_color()),
        "queued_color": color_to_rgb(queued_color()),
        "asap_color": color_to_rgb(asap_color()),
        "pending_color": color_to_rgb(pending_color()),
        "user_text": color_to_rgb(user_text()),
        "user_bg": color_to_rgb(user_bg()),
        "ai_text": color_to_rgb(ai_text()),
        "header_icon_color": color_to_rgb(header_icon_color()),
        "header_name_color": color_to_rgb(header_name_color()),
        "header_session_color": color_to_rgb(header_session_color()),
    }))
}

fn color_to_rgb(color: Color) -> Option<[u8; 3]> {
    match color {
        Color::Rgb(r, g, b) => Some([r, g, b]),
        Color::Indexed(n) if n >= 16 => {
            let (r, g, b) = crate::tui::color_support::indexed_to_rgb(n);
            Some([r, g, b])
        }
        _ => None,
    }
}
