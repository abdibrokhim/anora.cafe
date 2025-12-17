use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use std::time::{SystemTime, UNIX_EPOCH};

use super::Theme;
use crate::app::App;

pub fn render_home(f: &mut Frame, area: Rect, app: &App) {
    if app.show_splash {
        render_splash(f, area);
    } else {
        render_home_content(f, area, app);
    }
}

/// Render the splash screen with "anora" and blinking cursor
fn render_splash(f: &mut Frame, area: Rect) {
    // Center the content vertically
    let chunks = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .split(area);

    // Calculate blink state based on time (blink every 500ms)
    let blink_on = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        / 500
        % 2
        == 0;

    let cursor = if blink_on { "█" } else { " " };

    let line = Line::from(vec![
        Span::styled("anora", Style::default().fg(Theme::FG)),
        Span::styled(cursor, Style::default().fg(Theme::PINK)),
    ]);

    let paragraph = Paragraph::new(line).centered();
    f.render_widget(paragraph, chunks[1]);
}

/// Render the main home content after splash
fn render_home_content(f: &mut Frame, area: Rect, app: &App) {
    // Center the content vertically
    let chunks = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(5),
        Constraint::Fill(1),
    ])
    .split(area);

    let has_products = !app.products.is_empty();

    if has_products {
        let lines = vec![
            Line::from(Span::styled(
                "welcome to ANORA Labs",
                Style::default().fg(Theme::FG),
            )),
            Line::default(),
            Line::from(Span::styled(
                "press 's' to browse the shop",
                Style::default().fg(Theme::DIMMED),
            )),
        ];

        let paragraph = Paragraph::new(lines).centered();
        f.render_widget(paragraph, chunks[1]);
    } else {
        let lines = vec![
            Line::from(Span::styled(
                "no products available for this region",
                Style::default().fg(Theme::FG),
            )),
            Line::default(),
            Line::from(Span::styled(
                "press 'r' to change region",
                Style::default().fg(Theme::DIMMED),
            )),
        ];

        let paragraph = Paragraph::new(lines).centered();
        f.render_widget(paragraph, chunks[1]);
    }
}
