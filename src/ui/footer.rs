use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use super::Theme;
use crate::app::{App, ShippingMode, Tab};

pub fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(1), // Notification or shipping text
        Constraint::Length(1), // Divider
        Constraint::Length(1), // Navigation hints
    ])
    .split(area);

    // Show notification if present, otherwise show shipping text
    if let Some(notification) = &app.notification {
        let notification_para = Paragraph::new(Line::from(Span::styled(
            notification.clone(),
            Style::default().fg(Theme::RED),
        )))
        .centered();
        f.render_widget(notification_para, chunks[0]);
    } else {
        // Free shipping text
        let shipping_text = format!(
            "free shipping on {} orders over ${}",
            app.region.code, app.region.free_shipping_threshold
        );
        let shipping = Paragraph::new(Line::from(Span::styled(
            shipping_text,
            Style::default().fg(Theme::DIMMED),
        )))
        .centered();
        f.render_widget(shipping, chunks[0]);
    }

    // Divider
    let divider = Paragraph::new(Line::from(Span::styled(
        "─".repeat(area.width as usize),
        Style::default().fg(Theme::BORDER),
    )));
    f.render_widget(divider, chunks[1]);

    // Navigation hints based on current tab
    let nav_hints = get_navigation_hints(app);
    let nav = Paragraph::new(Line::from(nav_hints)).centered();
    f.render_widget(nav, chunks[2]);
}

fn get_navigation_hints(app: &App) -> Vec<Span<'static>> {
    match app.current_tab {
        Tab::Home => vec![
            Span::styled("r ", Style::default().fg(Theme::FG)),
            Span::styled(format!("{} ({})", app.region.flag, app.region.code), Style::default().fg(Theme::DIMMED)),
            Span::styled("   ", Style::default()),
            Span::styled("q ", Style::default().fg(Theme::FG)),
            Span::styled("quit", Style::default().fg(Theme::DIMMED)),
        ],
        Tab::Shop => vec![
            Span::styled("r ", Style::default().fg(Theme::FG)),
            Span::styled(format!("{} ({})", app.region.flag, app.region.code), Style::default().fg(Theme::DIMMED)),
            Span::styled("   ", Style::default()),
            Span::styled("↑/↓ ", Style::default().fg(Theme::FG)),
            Span::styled("products", Style::default().fg(Theme::DIMMED)),
            Span::styled("   ", Style::default()),
            Span::styled("+/- ", Style::default().fg(Theme::FG)),
            Span::styled("qty", Style::default().fg(Theme::DIMMED)),
            Span::styled("   ", Style::default()),
            Span::styled("c ", Style::default().fg(Theme::FG)),
            Span::styled("cart", Style::default().fg(Theme::DIMMED)),
            Span::styled("   ", Style::default()),
            Span::styled("q ", Style::default().fg(Theme::FG)),
            Span::styled("quit", Style::default().fg(Theme::DIMMED)),
        ],
        Tab::Account => vec![
            Span::styled("↑/↓ ", Style::default().fg(Theme::FG)),
            Span::styled("navigate", Style::default().fg(Theme::DIMMED)),
            Span::styled("   ", Style::default()),
            Span::styled("enter ", Style::default().fg(Theme::FG)),
            Span::styled("select", Style::default().fg(Theme::DIMMED)),
        ],
        Tab::Cart => {
            use crate::app::CheckoutStep;
            match app.checkout_step {
                CheckoutStep::Cart => vec![
                    Span::styled("esc ", Style::default().fg(Theme::FG)),
                    Span::styled("back", Style::default().fg(Theme::DIMMED)),
                    Span::styled("   ", Style::default()),
                    Span::styled("↑/↓ ", Style::default().fg(Theme::FG)),
                    Span::styled("items", Style::default().fg(Theme::DIMMED)),
                    Span::styled("   ", Style::default()),
                    Span::styled("+/- ", Style::default().fg(Theme::FG)),
                    Span::styled("qty", Style::default().fg(Theme::DIMMED)),
                    Span::styled("   ", Style::default()),
                    Span::styled("c ", Style::default().fg(Theme::FG)),
                    Span::styled("checkout", Style::default().fg(Theme::DIMMED)),
                ],
                CheckoutStep::Shipping => {
                    if app.shipping_mode == ShippingMode::SelectAddress {
                        vec![
                            Span::styled("esc ", Style::default().fg(Theme::FG)),
                            Span::styled("back", Style::default().fg(Theme::DIMMED)),
                            Span::styled("   ", Style::default()),
                            Span::styled("↑/↓ ", Style::default().fg(Theme::FG)),
                            Span::styled("addresses", Style::default().fg(Theme::DIMMED)),
                            Span::styled("   ", Style::default()),
                            Span::styled("x/del ", Style::default().fg(Theme::FG)),
                            Span::styled("remove", Style::default().fg(Theme::DIMMED)),
                            Span::styled("   ", Style::default()),
                            Span::styled("enter ", Style::default().fg(Theme::FG)),
                            Span::styled("select", Style::default().fg(Theme::DIMMED)),
                        ]
                    } else {
                        vec![
                            Span::styled("esc ", Style::default().fg(Theme::FG)),
                            Span::styled("back", Style::default().fg(Theme::DIMMED)),
                            Span::styled("   ", Style::default()),
                            Span::styled("↑/↓ ", Style::default().fg(Theme::FG)),
                            Span::styled("fields", Style::default().fg(Theme::DIMMED)),
                            Span::styled("   ", Style::default()),
                            Span::styled("tab ", Style::default().fg(Theme::FG)),
                            Span::styled("next", Style::default().fg(Theme::DIMMED)),
                            Span::styled("   ", Style::default()),
                            Span::styled("enter ", Style::default().fg(Theme::FG)),
                            Span::styled("continue", Style::default().fg(Theme::DIMMED)),
                        ]
                    }
                }
                CheckoutStep::Payment => vec![
                    Span::styled("esc ", Style::default().fg(Theme::FG)),
                    Span::styled("back", Style::default().fg(Theme::DIMMED)),
                    Span::styled("   ", Style::default()),
                    Span::styled("↑/↓ ", Style::default().fg(Theme::FG)),
                    Span::styled("fields", Style::default().fg(Theme::DIMMED)),
                    Span::styled("   ", Style::default()),
                    Span::styled("tab ", Style::default().fg(Theme::FG)),
                    Span::styled("next", Style::default().fg(Theme::DIMMED)),
                    Span::styled("   ", Style::default()),
                    Span::styled("enter ", Style::default().fg(Theme::FG)),
                    Span::styled("continue", Style::default().fg(Theme::DIMMED)),
                ],
                CheckoutStep::Confirmation => vec![
                    Span::styled("esc ", Style::default().fg(Theme::FG)),
                    Span::styled("back", Style::default().fg(Theme::DIMMED)),
                    Span::styled("   ", Style::default()),
                    Span::styled("enter ", Style::default().fg(Theme::FG)),
                    Span::styled("confirm order", Style::default().fg(Theme::DIMMED)),
                ],
            }
        }
    }
}

