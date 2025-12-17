use crate::app::{App, CheckoutStep, InputField, ShippingMode, Tab};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;

pub async fn handle_events(app: &mut App) -> anyhow::Result<bool> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                handle_key_event(app, key).await;
            }
        }
    }
    Ok(app.running)
}

async fn handle_key_event(app: &mut App, key: KeyEvent) {
    // During splash screen, any key skips it (except quit)
    if app.show_splash {
        match key.code {
            KeyCode::Char('q') => app.quit(),
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.quit(),
            _ => app.skip_splash(),
        }
        return;
    }

    // Handle input mode first
    if app.active_input != InputField::None {
        handle_input_mode(app, key).await;
        return;
    }

    // Global shortcuts
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.quit(),
        KeyCode::Char('r') => {
            // Cycle through regions instantly
            app.cycle_region().await;
        }
        KeyCode::Char('s') => {
            app.current_tab = Tab::Shop;
        }
        KeyCode::Char('a') => {
            app.current_tab = Tab::Account;
        }
        KeyCode::Char('c') => {
            app.current_tab = Tab::Cart;
        }
        _ => {
            // Tab-specific handling
            match app.current_tab {
                Tab::Home => handle_home_keys(app, key).await,
                Tab::Shop => handle_shop_keys(app, key).await,
                Tab::Account => handle_account_keys(app, key),
                Tab::Cart => handle_cart_keys(app, key).await,
            }
        }
    }
}

async fn handle_input_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char(c) => {
            app.handle_input_char(c);
        }
        KeyCode::Backspace => {
            app.handle_input_backspace();
        }
        KeyCode::Tab => {
            app.next_input_field();
        }
        KeyCode::Enter => {
            app.next_checkout_step().await;
        }
        KeyCode::Esc => {
            app.prev_checkout_step();
        }
        _ => {}
    }
}

async fn handle_home_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter | KeyCode::Char('s') => {
            if !app.products.is_empty() {
                app.current_tab = Tab::Shop;
            }
        }
        _ => {}
    }
}

async fn handle_shop_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => app.prev_product(),
        KeyCode::Down | KeyCode::Char('j') => app.next_product(),
        KeyCode::Char('+') | KeyCode::Char('=') => {
            app.product_quantity = (app.product_quantity + 1).min(99);
        }
        KeyCode::Char('-') | KeyCode::Char('_') => {
            app.product_quantity = (app.product_quantity - 1).max(1);
        }
        KeyCode::Enter => {
            // Add to cart or subscribe
            app.add_to_cart();
        }
        _ => {}
    }
}

fn handle_account_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => app.prev_account_section(),
        KeyCode::Down | KeyCode::Char('j') => app.next_account_section(),
        _ => {}
    }
}

async fn handle_cart_keys(app: &mut App, key: KeyEvent) {
    match app.checkout_step {
        CheckoutStep::Cart => {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => app.prev_cart_item(),
                KeyCode::Down | KeyCode::Char('j') => app.next_cart_item(),
                KeyCode::Char('+') | KeyCode::Char('=') => {
                    if let Some(item) = app.cart.items.get(app.cart_item_index) {
                        let id = item.product.id;
                        app.cart.increment_item(id);
                    }
                }
                KeyCode::Char('-') | KeyCode::Char('_') => {
                    if let Some(item) = app.cart.items.get(app.cart_item_index) {
                        let id = item.product.id;
                        app.cart.decrement_item(id);
                        // Reset index if item was removed
                        if app.cart_item_index >= app.cart.items.len() && !app.cart.items.is_empty() {
                            app.cart_item_index = app.cart.items.len() - 1;
                        }
                    }
                }
                KeyCode::Enter | KeyCode::Char('c') => {
                    app.next_checkout_step().await;
                }
                KeyCode::Esc => {
                    app.current_tab = Tab::Shop;
                }
                _ => {}
            }
        }
        CheckoutStep::Shipping if app.shipping_mode == ShippingMode::SelectAddress => {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => app.prev_address_option(),
                KeyCode::Down | KeyCode::Char('j') => app.next_address_option(),
                KeyCode::Enter => app.select_address_option(),
                KeyCode::Backspace | KeyCode::Delete | KeyCode::Char('x') => {
                    app.remove_selected_address().await;
                }
                KeyCode::Esc => app.prev_checkout_step(),
                _ => {}
            }
        }
        CheckoutStep::Payment if app.payment_method.is_none() => {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => app.prev_payment_option(),
                KeyCode::Down | KeyCode::Char('j') => app.next_payment_option(),
                KeyCode::Enter => app.select_payment_method(),
                KeyCode::Esc => app.prev_checkout_step(),
                _ => {}
            }
        }
        _ => {
            // Input mode is handled separately
            match key.code {
                KeyCode::Esc => app.prev_checkout_step(),
                _ => {}
            }
        }
    }
}

