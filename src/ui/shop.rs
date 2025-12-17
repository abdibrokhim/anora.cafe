use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

use super::Theme;
use crate::app::App;
use crate::models::{ProductCategory, ProductType};

pub fn render_shop(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Length(1),
        Constraint::Percentage(69),
    ])
    .split(area);

    render_product_list(f, chunks[0], app);
    render_product_details(f, chunks[2], app);
}

fn render_product_list(f: &mut Frame, area: Rect, app: &App) {
    let mut lines: Vec<Line> = Vec::new();

    // Group products by category
    let featured: Vec<_> = app
        .products
        .iter()
        .filter(|p| p.category == ProductCategory::Featured)
        .collect();
    let originals: Vec<_> = app
        .products
        .iter()
        .filter(|p| p.category == ProductCategory::Originals)
        .collect();

    // Featured section
    if !featured.is_empty() {
        lines.push(Line::from(Span::styled(
            "~ featured ~",
            Style::default().fg(Theme::FG),
        )));

        for product in &featured {
            let is_selected = app.selected_product_index < featured.len()
                && featured[app.selected_product_index].id == product.id;
            
            let color = Theme::product_color(&product.name);
            let style = if is_selected {
                Style::default().fg(Theme::FG).bg(color)
            } else {
                Style::default().fg(Theme::DIMMED)
            };

            // Create a line that spans the full width with padding
            let padding = " ".repeat(1); // 1 spaces padding on each side
            let content_width = area.width.saturating_sub(4) as usize; // Account for padding
            let product_name = if product.name.len() > content_width {
                product.name.chars().take(content_width).collect::<String>()
            } else {
                format!("{:<width$}", product.name, width = content_width)
            };
            
            lines.push(Line::from(Span::styled(
                format!("{}{}{}", padding, product_name, padding),
                style
            )));
        }

        lines.push(Line::default());
    }

    // Originals section
    if !originals.is_empty() {
        lines.push(Line::from(Span::styled(
            "~ originals ~",
            Style::default().fg(Theme::FG),
        )));

        for (i, product) in originals.iter().enumerate() {
            let global_index = featured.len() + i;
            let is_selected = app.selected_product_index == global_index;
            
            let color = Theme::product_color(&product.name);
            let style = if is_selected {
                Style::default().fg(Theme::FG).bg(color)
            } else {
                Style::default().fg(Theme::DIMMED)
            };

            // Create a line that spans the full width with padding
            let padding = " ".repeat(1); // 1 spaces padding on each side
            let content_width = area.width.saturating_sub(4) as usize; // Account for padding
            let product_name = if product.name.len() > content_width {
                product.name.chars().take(content_width).collect::<String>()
            } else {
                format!("{:<width$}", product.name, width = content_width)
            };
            
            lines.push(Line::from(Span::styled(
                format!("{}{}{}", padding, product_name, padding),
                style
            )));
        }
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, area);
}

fn render_product_details(f: &mut Frame, area: Rect, app: &App) {
    if app.products.is_empty() {
        return;
    }

    let product = &app.products[app.selected_product_index];
    let color = Theme::product_color(&product.name);

    let mut lines: Vec<Line> = vec![
        // Product name
        Line::from(Span::styled(
            product.name.clone(),
            Style::default().fg(Theme::FG),
        )),
        // Product details line
        Line::from(Span::styled(
            product.details_line(),
            Style::default().fg(Theme::DIMMED),
        )),
        Line::default(),
        // Price
        Line::from(Span::styled(
            product.price_display(),
            Style::default().fg(color),
        )),
        Line::default(),
    ];

    // Description - wrap it manually for better display
    let desc_style = Style::default().fg(Theme::DIMMED);
    lines.push(Line::from(Span::styled(product.description.clone(), desc_style)));
    lines.push(Line::default());

    // Action based on product type
    match product.product_type {
        ProductType::Subscription => {
            let padding = " ".repeat(1);
            let subscribe_text = "subscribe";
            let content_width = area.width.saturating_sub(4) as usize;
            let padded_subscribe = if subscribe_text.len() + 2 <= content_width {
                format!("{} {}{}", padding, subscribe_text, padding)
            } else {
                subscribe_text.to_string()
            };
            
            lines.push(Line::from(vec![
                Span::styled(padded_subscribe, Style::default().fg(Theme::FG).bg(color)),
                Span::styled("  enter", Style::default().fg(Theme::DIMMED)),
            ]));
        }
        ProductType::OneTime => {
            lines.push(Line::from(vec![
                Span::styled("-", Style::default().fg(Theme::DIMMED)),
                Span::styled(
                    format!(" {} ", app.product_quantity),
                    Style::default().fg(Theme::FG),
                ),
                Span::styled("+", Style::default().fg(Theme::DIMMED)),
            ]));
        }
    }

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
