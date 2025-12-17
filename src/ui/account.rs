use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

use super::Theme;
use crate::app::{AccountSection, App};

pub fn render_account(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Length(1),
        Constraint::Percentage(69),
    ])
    .split(area);

    render_account_menu(f, chunks[0], app);
    render_account_content(f, chunks[2], app);
}

fn render_account_menu(f: &mut Frame, area: Rect, app: &App) {
    let sections = [
        (AccountSection::OrderHistory, "order history"),
        (AccountSection::Subscriptions, "subscriptions"),
        (AccountSection::Faq, "faq"),
        (AccountSection::About, "about"),
    ];

    let lines: Vec<Line> = sections
        .iter()
        .map(|(section, label)| {
            let is_selected = app.account_section == *section;
            let style = if is_selected {
                Style::default().fg(Theme::FG).bg(Theme::PINK)
            } else {
                Style::default().fg(Theme::DIMMED)
            };

            // Create a line that spans the full width with padding (same as shop.rs)
            let padding = " ".repeat(1);
            let content_width = area.width.saturating_sub(4) as usize;
            let menu_label = if label.len() > content_width {
                label.chars().take(content_width).collect::<String>()
            } else {
                format!("{:<width$}", label, width = content_width)
            };

            Line::from(Span::styled(
                format!("{}{}{}", padding, menu_label, padding),
                style,
            ))
        })
        .collect();

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, area);
}

fn render_account_content(f: &mut Frame, area: Rect, app: &App) {
    let (content, is_empty_state) = match app.account_section {
        AccountSection::OrderHistory => render_order_history(app),
        AccountSection::Subscriptions => render_subscriptions(app),
        AccountSection::Faq => (render_faq(), false),
        AccountSection::About => (render_about(), false),
    };

    let paragraph = if is_empty_state {
        // Center empty state messages
        Paragraph::new(content)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
    } else {
        Paragraph::new(content).wrap(Wrap { trim: true })
    };
    f.render_widget(paragraph, area);
}

fn render_order_history(app: &App) -> (Vec<Line<'static>>, bool) {
    if app.orders.is_empty() {
        (
            vec![Line::from(Span::styled(
                "no orders found",
                Style::default().fg(Theme::DIMMED),
            ))],
            true,
        )
    } else {
        (
            app.orders
                .iter()
                .map(|order| {
                    Line::from(vec![
                        Span::styled(
                            format!("Order #{} - ", &order.id.to_string()[..8]),
                            Style::default().fg(Theme::FG),
                        ),
                        Span::styled(
                            order.total_display(),
                            Style::default().fg(Theme::PINK),
                        ),
                        Span::styled(
                            format!(" - {}", order.status),
                            Style::default().fg(Theme::DIMMED),
                        ),
                    ])
                })
                .collect(),
            false,
        )
    }
}

fn render_subscriptions(app: &App) -> (Vec<Line<'static>>, bool) {
    if app.subscriptions.is_empty() {
        (
            vec![Line::from(Span::styled(
                "no active subscriptions",
                Style::default().fg(Theme::DIMMED),
            ))],
            true,
        )
    } else {
        (
            app.subscriptions
                .iter()
                .map(|sub| {
                    Line::from(vec![
                        Span::styled(
                            sub.product_name.clone(),
                            Style::default().fg(Theme::FG),
                        ),
                        Span::styled(
                            format!(" - {}", sub.status),
                            Style::default().fg(Theme::DIMMED),
                        ),
                    ])
                })
                .collect(),
            false,
        )
    }
}

fn render_faq() -> Vec<Line<'static>> {
    vec![
        Line::from(Span::styled(
            "help, i have a question about my order!",
            Style::default().fg(Theme::FG),
        )),
        Line::from(Span::styled(
            "send us an email at support@anoralabs.com",
            Style::default().fg(Theme::DIMMED),
        )),
        Line::default(),
        Line::from(Span::styled(
            "when was my coffee roasted? the roast date is blank on my bag.",
            Style::default().fg(Theme::FG),
        )),
        Line::from(Span::styled(
            "we roast your coffee within 24 hours of shipping, and you'll receive an email once your order ships. we're working on a solution to stamping the roast date on the bags, but so far all attempts have failed (the ink wipes off).",
            Style::default().fg(Theme::DIMMED),
        )),
        Line::default(),
        Line::from(Span::styled(
            "where do you ship?",
            Style::default().fg(Theme::FG),
        )),
        Line::from(Span::styled(
            "we ship all of our blends in the UZ. Unfortunately, we don't ship to other countries at this time. We are actively looking for other small-batch roasters in other countries and regions though!",
            Style::default().fg(Theme::DIMMED),
        )),
        Line::default(),
        Line::from(Span::styled(
            "is your coffee ethically sourced?",
            Style::default().fg(Theme::FG),
        )),
        Line::from(Span::styled(
            "absolutely.",
            Style::default().fg(Theme::DIMMED),
        )),
        Line::default(),
        Line::from(Span::styled(
            "is ordering via ssh secure?",
            Style::default().fg(Theme::FG),
        )),
        Line::from(Span::styled(
            "yes! all payment information is securely processed.",
            Style::default().fg(Theme::DIMMED),
        )),
    ]
}

fn render_about() -> Vec<Line<'static>> {
    // Simple blinking cursor using time-based toggle
    // Use unwrap_or_default() to gracefully handle system time errors
    let cursor = if (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() / 500) % 2 == 0 {
        "█"
    } else {
        " "
    };

    vec![
        Line::from(Span::styled(
            "Amazingly awesome products for developers brought to you by a group of talented, good looking, and humble heroes...",
            Style::default().fg(Theme::DIMMED),
        )),
        Line::default(),
        Line::from(Span::styled(
            "1. @abdibrokhim",
            Style::default().fg(Theme::FG),
        )),
        Line::default(),
        Line::from(Span::styled(
            "2. @asadbekmake",
            Style::default().fg(Theme::FG),
        )),
        Line::default(),
        Line::from(vec![
            Span::styled(
                "3. ANORA Labs, Inc.",
                Style::default().fg(Theme::FG),
            ),
            Span::styled(
                cursor,
                Style::default().fg(Theme::PINK),
            ),
        ]),
    ]
}
