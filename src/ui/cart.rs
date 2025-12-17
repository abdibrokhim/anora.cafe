use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

use super::Theme;
use crate::app::{App, CheckoutStep, InputField, PaymentMethod, ShippingMode};

pub fn render_cart(f: &mut Frame, area: Rect, app: &App) {
    match app.checkout_step {
        CheckoutStep::Cart => render_cart_items(f, area, app),
        CheckoutStep::Shipping => render_shipping(f, area, app),
        CheckoutStep::Payment => render_payment(f, area, app),
        CheckoutStep::Confirmation => render_confirmation(f, area, app),
    }
}

fn render_cart_items(f: &mut Frame, area: Rect, app: &App) {
    if app.cart.is_empty() {
        let chunks = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(area);

        let empty = Paragraph::new(Line::from(Span::styled(
            "Your cart is empty.",
            Style::default().fg(Theme::DIMMED),
        )))
        .centered();
        f.render_widget(empty, chunks[1]);
        return;
    }

    // Each item: 4 lines height (reduced from 6)
    let item_height = 4u16;
    let gap_height = 0u16;
    
    let mut constraints: Vec<Constraint> = Vec::new();
    for i in 0..app.cart.items.len() {
        constraints.push(Constraint::Length(item_height));
        if i < app.cart.items.len() - 1 {
            constraints.push(Constraint::Length(gap_height));
        }
    }
    constraints.push(Constraint::Fill(1));

    let chunks = Layout::vertical(constraints).split(area);

    for (i, item) in app.cart.items.iter().enumerate() {
        let is_selected = i == app.cart_item_index;
        let chunk_index = i * 2;
        let item_area = chunks[chunk_index];

        let border_style = if is_selected {
            Style::default().fg(Theme::FG)
        } else {
            Style::default().fg(Theme::BORDER)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .padding(Padding::horizontal(1));

        let inner = block.inner(item_area);
        f.render_widget(block, item_area);

        let content_chunks = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner);

        let name_chunks = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(18),
        ])
        .split(content_chunks[0]);

        let details_chunks = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(18),
        ])
        .split(content_chunks[1]);

        let name_para = Paragraph::new(Line::from(Span::styled(
            item.product.name.clone(),
            Style::default().fg(Theme::FG),
        )));
        f.render_widget(name_para, name_chunks[0]);

        let details_para = Paragraph::new(Line::from(Span::styled(
            item.product.details_line(),
            Style::default().fg(Theme::DIMMED),
        )));
        f.render_widget(details_para, details_chunks[0]);

        let qty_price = if is_selected {
            Line::from(vec![
                Span::styled(" - ", Style::default().fg(Theme::DIMMED)),
                Span::styled(format!(" {} ", item.quantity), Style::default().fg(Theme::FG)),
                Span::styled(" + ", Style::default().fg(Theme::DIMMED)),
                Span::styled(
                    format!("   {}", item.total_display()),
                    Style::default().fg(Theme::DIMMED),
                ),
            ])
        } else {
            Line::from(vec![
                Span::styled(format!("{}      ", item.quantity), Style::default().fg(Theme::FG)),
                Span::styled(item.total_display(), Style::default().fg(Theme::DIMMED)),
            ])
        };

        let right_para = Paragraph::new(qty_price).right_aligned();
        f.render_widget(right_para, name_chunks[1]);
    }
}

fn render_shipping(f: &mut Frame, area: Rect, app: &App) {
    match app.shipping_mode {
        ShippingMode::SelectAddress => render_address_selection(f, area, app),
        ShippingMode::AddNewAddress => render_address_form(f, area, app),
    }
}

fn render_address_selection(f: &mut Frame, area: Rect, app: &App) {
    // Title
    let title_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: 2,
    };
    let title = Paragraph::new(Line::from(Span::styled(
        "select shipping address",
        Style::default().fg(Theme::DIMMED),
    )));
    f.render_widget(title, title_area);

    // Calculate layout for addresses + add new option
    let content_area = Rect {
        x: area.x,
        y: area.y + 2,
        width: area.width,
        height: area.height.saturating_sub(2),
    };

    let item_height = 3u16;
    let total_items = app.saved_addresses.len() + 1; // +1 for "add new address"
    
    let mut constraints: Vec<Constraint> = Vec::new();
    for _ in 0..total_items {
        constraints.push(Constraint::Length(item_height));
    }
    constraints.push(Constraint::Fill(1));

    let chunks = Layout::vertical(constraints).split(content_area);

    // Render saved addresses
    for (i, address) in app.saved_addresses.iter().enumerate() {
        let is_selected = i == app.address_select_index;
        let border_style = if is_selected {
            Style::default().fg(Theme::FG)
        } else {
            Style::default().fg(Theme::BORDER)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .padding(Padding::horizontal(2));

        let inner = block.inner(chunks[i]);
        f.render_widget(block, chunks[i]);

        // Address content
        let content = Line::from(vec![
            Span::styled("◉ ", Style::default().fg(if is_selected { Theme::FG } else { Theme::DIMMED })),
            Span::styled(address.display_line(), Style::default().fg(Theme::FG)),
            if is_selected {
                Span::styled("                    enter", Style::default().fg(Theme::DIMMED))
            } else {
                Span::styled("", Style::default())
            },
        ]);
        let para = Paragraph::new(content);
        f.render_widget(para, inner);
    }

    // Render "add new address" option
    let add_new_index = app.saved_addresses.len();
    let is_add_selected = app.address_select_index == add_new_index;
    let add_border_style = if is_add_selected {
        Style::default().fg(Theme::FG)
    } else {
        Style::default().fg(Theme::BORDER)
    };

    let add_block = Block::default()
        .borders(Borders::ALL)
        .border_style(add_border_style)
        .padding(Padding::horizontal(2));

    let add_inner = add_block.inner(chunks[add_new_index]);
    f.render_widget(add_block, chunks[add_new_index]);

    let add_content = Line::from(vec![
        Span::styled("add new address", Style::default().fg(Theme::DIMMED)),
    ]);
    let add_para = Paragraph::new(add_content);
    f.render_widget(add_para, add_inner);
}

fn render_address_form(f: &mut Frame, area: Rect, app: &App) {
    // Two columns layout
    let form_chunks = Layout::horizontal([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(area);

    // Left column fields: name, street 1, city
    let left_fields = [
        (InputField::Name, "name", &app.shipping_address.name),
        (InputField::Street1, "street", &app.shipping_address.street_1),
        (InputField::City, "city", &app.shipping_address.city),
    ];

    let left_lines: Vec<Line> = left_fields
        .iter()
        .flat_map(|(field, label, value)| {
            let is_active = app.active_input == *field;
            render_form_field(label, value, is_active)
        })
        .collect();

    let left_para = Paragraph::new(left_lines);
    f.render_widget(left_para, form_chunks[0]);

    // Right column fields: country, phone, postal code
    let right_fields = [
        (InputField::Country, "country", &app.shipping_address.country),
        (InputField::Phone, "phone", &app.shipping_address.phone),
        (InputField::PostalCode, "postal code", &app.shipping_address.postal_code),
    ];

    let right_lines: Vec<Line> = right_fields
        .iter()
        .flat_map(|(field, label, value)| {
            let is_active = app.active_input == *field;
            render_form_field(label, value, is_active)
        })
        .collect();

    let right_para = Paragraph::new(right_lines);
    f.render_widget(right_para, form_chunks[1]);
}

fn render_payment(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(2),
        Constraint::Fill(1),
    ])
    .split(area);

    // Order summary
    let shipping_cents = if app.cart.subtotal_cents() >= app.region.free_shipping_threshold * 100 {
        0
    } else {
        800
    };
    let total = app.cart.subtotal_cents() + shipping_cents;

    let summary = Paragraph::new(vec![
        Line::default(),
        Line::from(vec![
            Span::styled("subtotal: ", Style::default().fg(Theme::DIMMED)),
            Span::styled(format!("${:.2}", app.cart.subtotal_cents() as f64 / 100.0), Style::default().fg(Theme::FG)),
            Span::styled(",  shipping: ", Style::default().fg(Theme::DIMMED)),
            Span::styled(format!("${:.2}", shipping_cents as f64 / 100.0), Style::default().fg(Theme::FG)),
            Span::styled(",  total: ", Style::default().fg(Theme::DIMMED)),
            Span::styled(format!("${:.2}", total as f64 / 100.0), Style::default().fg(Theme::PINK)),
        ]),
    ]);
    f.render_widget(summary, chunks[0]);

    // Title
    let title = Paragraph::new(vec![
        Line::from(Span::styled(
            "select payment method",
            Style::default().fg(Theme::DIMMED),
        )),
    ]);
    f.render_widget(title, chunks[1]);

    match app.payment_method {
        None => render_payment_selection(f, chunks[2], app),
        Some(PaymentMethod::Ssh) => render_ssh_payment(f, chunks[2], app),
        Some(PaymentMethod::Browser) => render_browser_payment(f, chunks[2], app),
    }
}

fn render_payment_selection(f: &mut Frame, area: Rect, app: &App) {
    let options = [
        ("add payment information via ssh", PaymentMethod::Ssh),
        ("add payment information via browser", PaymentMethod::Browser),
    ];

    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Fill(1),
    ])
    .split(area);

    for (i, (label, _method)) in options.iter().enumerate() {
        let is_selected = i == app.payment_option_index;
        let border_style = if is_selected {
            Style::default().fg(Theme::FG)
        } else {
            Style::default().fg(Theme::DIMMED)
        };

        let content = Line::from(vec![
            Span::styled("◉ ", Style::default().fg(if is_selected { Theme::FG } else { Theme::DIMMED })),
            Span::styled(*label, Style::default().fg(Theme::FG)),
            Span::styled("                            enter", Style::default().fg(Theme::DIMMED)),
        ]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .padding(Padding::horizontal(2));
        let para = Paragraph::new(content).block(block);
        
        let target_chunk = if i == 0 { chunks[0] } else { chunks[2] };
        f.render_widget(para, target_chunk);
    }
}

fn render_ssh_payment(f: &mut Frame, area: Rect, app: &App) {
    // Two columns layout (same as shipping)
    let form_chunks = Layout::horizontal([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ])
    .split(area);

    // Left column: name, email, card number
    let left_fields = [
        (InputField::PaymentName, "name", &app.payment_info.name),
        (InputField::PaymentEmail, "email", &app.payment_info.email),
        (InputField::CardNumber, "card number", &app.payment_info.card_number),
    ];

    let left_lines: Vec<Line> = left_fields
        .iter()
        .flat_map(|(field, label, value)| {
            let is_active = app.active_input == *field;
            render_form_field(label, value, is_active)
        })
        .collect();

    let left_para = Paragraph::new(left_lines);
    f.render_widget(left_para, form_chunks[0]);

    // Right column: expiry month, expiry year, cvv
    let right_fields = [
        (InputField::ExpiryMonth, "expiry month", &app.payment_info.expiry_month),
        (InputField::ExpiryYear, "expiry year", &app.payment_info.expiry_year),
        (InputField::Cvv, "cvv (3 digits)", &app.payment_info.cvv),
    ];

    let right_lines: Vec<Line> = right_fields
        .iter()
        .flat_map(|(field, label, value)| {
            let is_active = app.active_input == *field;
            render_form_field(label, value, is_active)
        })
        .collect();

    let right_para = Paragraph::new(right_lines);
    f.render_widget(right_para, form_chunks[1]);
}

fn render_browser_payment(f: &mut Frame, area: Rect, _app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(9),
        Constraint::Length(2),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .split(area);

    let qr_placeholder = vec![
        Line::from("┌───────────────────┐"),
        Line::from("│  ██ ██ ██  ██ ██  │"),
        Line::from("│  ██    ██  ██     │"),
        Line::from("│      ██  ██  ██   │"),
        Line::from("│  ██ ██    ██      │"),
        Line::from("│      ██ ██    ██  │"),
        Line::from("│  ██    ██  ██ ██  │"),
        Line::from("│  ██ ██ ██  ██     │"),
        Line::from("└───────────────────┘"),
    ];
    let qr = Paragraph::new(qr_placeholder).centered();
    f.render_widget(qr, chunks[1]);

    let instruction = Paragraph::new(Line::from(Span::styled(
        "scan or copy to enter payment information",
        Style::default().fg(Theme::DIMMED),
    )))
    .centered();
    f.render_widget(instruction, chunks[3]);

    let url = Paragraph::new(Line::from(Span::styled(
        "https://pay.anoralabs.com/checkout/abc123",
        Style::default().fg(Theme::PINK),
    )))
    .centered();
    f.render_widget(url, chunks[4]);
}

fn render_confirmation(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(12),
        Constraint::Fill(1),
    ])
    .split(area);

    let shipping_cents = if app.cart.subtotal_cents() >= app.region.free_shipping_threshold * 100 {
        0
    } else {
        800
    };
    let total = app.cart.subtotal_cents() + shipping_cents;

    let lines = vec![
        Line::from(Span::styled(
            "order confirmation",
            Style::default().fg(Theme::DIMMED),
        )),
        Line::default(),
        Line::default(),
        Line::from(vec![
            Span::styled("shipping to: ", Style::default().fg(Theme::DIMMED)),
            Span::styled(app.shipping_address.name.clone(), Style::default().fg(Theme::FG)),
        ]),
        Line::from(Span::styled(
            format!("{}, {}", app.shipping_address.street_1, app.shipping_address.city),
            Style::default().fg(Theme::DIMMED),
        )),
        Line::default(),
        Line::default(),
        Line::from(vec![
            Span::styled("items: ", Style::default().fg(Theme::DIMMED)),
            Span::styled(format!("{}", app.cart.total_items()), Style::default().fg(Theme::FG)),
        ]),
        Line::from(vec![
            Span::styled("total: ", Style::default().fg(Theme::DIMMED)),
            Span::styled(format!("${:.2}", total as f64 / 100.0), Style::default().fg(Theme::PINK)),
        ]),
        Line::default(),
        Line::default(),
        Line::from(Span::styled(
            "press enter to confirm your order",
            Style::default().fg(Theme::GREEN),
        )),
    ];

    let para = Paragraph::new(lines).centered();
    f.render_widget(para, chunks[1]);
}
/// Render a form field with label and value in the terminal.shop style
/// Format:
///   label
/// > value (or cursor if active and empty)
fn render_form_field<'a>(label: &'a str, value: &'a str, is_active: bool) -> Vec<Line<'a>> {
    let label_style = Style::default().fg(Theme::DIMMED);
    
    // Build the value line with ">" prefix
    let value_line = if is_active {
        if value.is_empty() {
            // Show pink cursor block when active and empty
            Line::from(vec![
                Span::styled("│ ", Style::default().fg(Theme::FG)),
                Span::styled("> ", Style::default().fg(Theme::FG)),
                Span::styled("█", Style::default().fg(Theme::PINK)),
            ])
        } else {
            // Show value with cursor at end
            Line::from(vec![
                Span::styled("│ ", Style::default().fg(Theme::FG)),
                Span::styled("> ", Style::default().fg(Theme::FG)),
                Span::styled(value, Style::default().fg(Theme::FG)),
                Span::styled("█", Style::default().fg(Theme::PINK)),
            ])
        }
    } else {
        if value.is_empty() {
            Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled(">", Style::default().fg(Theme::DIMMED)),
            ])
        } else {
            Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled("> ", Style::default().fg(Theme::DIMMED)),
                Span::styled(value, Style::default().fg(Theme::FG)),
            ])
        }
    };

    // Add left border indicator for active field
    let label_line = if is_active {
        Line::from(vec![
            Span::styled("│ ", Style::default().fg(Theme::FG)),
            Span::styled(label, label_style),
        ])
    } else {
        Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(label, label_style),
        ])
    };

    vec![
        label_line,
        value_line,
        Line::default(),
    ]
}
