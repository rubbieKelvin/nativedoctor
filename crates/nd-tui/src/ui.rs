//! Crossterm + ratatui render loop.

use std::io::{stdout, Stdout};

use crossterm::{
    event::{Event as CrosstermEvent, EventStream, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures_util::StreamExt;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::time::{interval, MissedTickBehavior};

use crate::app::{App, PaneFocus, RightTab};
use crate::runner::TuiMsg;

pub async fn run_terminal(mut rx: UnboundedReceiver<TuiMsg>) -> Result<(), String> {
    enable_raw_mode().map_err(|e| e.to_string())?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen).map_err(|e| e.to_string())?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(|e| e.to_string())?;

    let mut app = App::new();
    let mut reader = EventStream::new();
    let mut tick = interval(std::time::Duration::from_millis(120));
    tick.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let mut out: Result<(), String> = Ok(());

    loop {
        tokio::select! {
            biased;
            msg = rx.recv() => {
                match msg {
                    None => break,
                    Some(m) => app.apply(m),
                }
            }
            ev = reader.next() => {
                match ev {
                    Some(Ok(CrosstermEvent::Key(key))) => {
                        if key.kind == KeyEventKind::Press && app.handle_key(key) {
                            break;
                        }
                    }
                    Some(Ok(CrosstermEvent::Resize(_, _))) => {}
                    Some(Err(e)) => {
                        out = Err(e.to_string());
                        break;
                    }
                    None => {}
                    _ => {}
                }
            }
            _ = tick.tick() => {}
        }

        if let Err(e) = terminal.draw(|f| draw(f, &app)) {
            out = Err(e.to_string());
            break;
        }
    }

    restore_terminal(&mut terminal)?;
    out?;
    if app.should_fail_exit() {
        return Err(app
            .runner_error
            .clone()
            .unwrap_or_else(|| "run failed".into()));
    }
    Ok(())
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), String> {
    disable_raw_mode().map_err(|e| e.to_string())?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).map_err(|e| e.to_string())?;
    terminal.show_cursor().map_err(|e| e.to_string())?;
    Ok(())
}

fn draw(f: &mut Frame<'_>, app: &App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(4),
            Constraint::Length(2),
        ])
        .split(area);

    let hdr = format!(
        " session …{} │ {}",
        app.session_label,
        if app.current_path.is_empty() {
            "—"
        } else {
            app.current_path.as_str()
        }
    );
    let header = Paragraph::new(hdr).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" nativedoctor tui "),
    );
    f.render_widget(header, chunks[0]);

    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(38), Constraint::Percentage(62)])
        .split(chunks[1]);

    let timeline_h = main[0].height.saturating_sub(2) as usize;
    let items: Vec<ListItem> = app
        .timeline
        .iter()
        .skip(app.timeline_scroll)
        .take(timeline_h.max(1))
        .map(|l| ListItem::new(l.as_str()))
        .collect();
    let tl_title = if app.focus == PaneFocus::Timeline {
        " timeline (focused) "
    } else {
        " timeline "
    };
    let timeline = List::new(items).block(Block::default().borders(Borders::ALL).title(tl_title));
    f.render_widget(timeline, main[0]);

    if app.stream_ratio.is_some() {
        let rs = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(3)])
            .split(main[1]);
        draw_right_pane(f, app, rs[0]);
        let g = Gauge::default()
            .ratio(app.stream_ratio.unwrap_or(0.0).clamp(0.0, 1.0))
            .label(app.stream_label.clone())
            .block(Block::default().borders(Borders::ALL).title(" download "));
        f.render_widget(g, rs[1]);
    } else {
        draw_right_pane(f, app, main[1]);
    }

    let help = format!(
        "{} │ q quit │ tab focus │ 1-4 tabs │ j/k scroll │ h hex body │ {}",
        app.footer_status,
        if app.checkpoint_note.is_empty() {
            ""
        } else {
            "checkpoint active │ "
        }
    );
    let footer = Paragraph::new(help).block(Block::default().borders(Borders::TOP));
    f.render_widget(footer, chunks[2]);
}

fn draw_right_pane(f: &mut Frame<'_>, app: &App, area: Rect) {
    let focus_hint = if app.focus == PaneFocus::Right {
        " (focused)"
    } else {
        ""
    };
    let title = match app.right_tab {
        RightTab::Preview => format!(" 1 preview{focus_hint} "),
        RightTab::Logs => format!(" 2 logs{focus_hint} "),
        RightTab::Body => format!(" 3 body{focus_hint} "),
        RightTab::Vars => format!(" 4 vars{focus_hint} "),
    };

    match app.right_tab {
        RightTab::Preview => {
            let mut text = app.request_preview.clone();
            if !app.checkpoint_note.is_empty() {
                text.push_str("\n\n── checkpoint ──\n");
                text.push_str(&app.checkpoint_note);
            }
            let p = Paragraph::new(text)
                .wrap(Wrap { trim: false })
                .block(Block::default().borders(Borders::ALL).title(title));
            f.render_widget(p, area);
        }
        RightTab::Logs => {
            let h = area.height.saturating_sub(2) as usize;
            let lines: Vec<ListItem> = app
                .logs
                .iter()
                .skip(app.log_scroll)
                .take(h.max(1))
                .map(|(lvl, m)| ListItem::new(format!("[{lvl}] {m}")))
                .collect();
            let list = List::new(lines).block(Block::default().borders(Borders::ALL).title(title));
            f.render_widget(list, area);
        }
        RightTab::Body => {
            let p = Paragraph::new(app.body_text.as_str())
                .wrap(Wrap { trim: false })
                .scroll((0, app.body_scroll as u16))
                .block(Block::default().borders(Borders::ALL).title(title));
            f.render_widget(p, area);
        }
        RightTab::Vars => {
            let h = area.height.saturating_sub(2) as usize;
            let lines: Vec<ListItem> = app
                .vars_lines
                .iter()
                .skip(app.vars_scroll)
                .take(h.max(1))
                .map(|l| ListItem::new(l.as_str()))
                .collect();
            let list = List::new(lines).block(Block::default().borders(Borders::ALL).title(title));
            f.render_widget(list, area);
        }
    }
}
