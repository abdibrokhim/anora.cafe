use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::Theme;
use crate::app::{App, Tab};

pub fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .split(area);

    // Tab definitions
    let tabs = [
        (Tab::Home, "anora", ""),
        (Tab::Shop, "s", "shop"),
        (Tab::Account, "a", "account"),
        (Tab::Cart, "c", &format!("cart ${} [{}]", app.cart.subtotal_cents() / 100, app.cart.total_items())),
    ];

    for (i, (tab, key, label)) in tabs.iter().enumerate() {
        let is_active = app.current_tab == *tab;
        
        let content = if key.is_empty() {
            label.to_string()
        } else if label.is_empty() {
            key.to_string()
        } else {
            format!("{} {}", key, label)
        };

        let style = if *tab == Tab::Home {
            Style::default().fg(Theme::FG)
        } else if is_active {
            Style::default().fg(Theme::FG)
        } else {
            Style::default().fg(Theme::DIMMED)
        };

        let paragraph = Paragraph::new(Line::from(vec![Span::styled(content, style)]))
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Theme::BORDER)))
            .centered();

        f.render_widget(paragraph, chunks[i]);
    }
}

pub fn render_checkout_header(f: &mut Frame, area: Rect, app: &App) {
    use crate::app::CheckoutStep;

    let steps = ["cart", "shipping", "payment", "confirmation"];
    
    let spans: Vec<Span> = steps
        .iter()
        .enumerate()
        .flat_map(|(i, step)| {
            let is_current = match app.checkout_step {
                CheckoutStep::Cart => i == 0,
                CheckoutStep::Shipping => i == 1,
                CheckoutStep::Payment => i == 2,
                CheckoutStep::Confirmation => i == 3,
            };

            let style = if is_current {
                Style::default().fg(Theme::FG)
            } else {
                Style::default().fg(Theme::DIMMED)
            };

            let mut result = vec![Span::styled(*step, style)];
            if i < steps.len() - 1 {
                result.push(Span::styled(" / ", Style::default().fg(Theme::DIMMED)));
            }
            result
        })
        .collect();

    let paragraph = Paragraph::new(Line::from(spans));
    f.render_widget(paragraph, area);
}
