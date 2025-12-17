mod app;
mod db;
mod events;
mod models;
mod ui;

use app::{App, Tab};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Clear},
    Terminal,
    Frame,
};
use std::io;

// Maximum UI dimensions (600x600 square in terminal cells)
const MAX_WIDTH: u16 = 80;
const MAX_HEIGHT: u16 = 30;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    let _ = dotenvy::dotenv();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new();
    
    // Load initial data (regions + products) from Supabase
    let _ = app.load_initial_data().await;

    let result = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {err}");
    }

    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> anyhow::Result<()> {
    while app.running {
        // Check if splash screen should transition
        app.check_splash_timeout();
        
        terminal.draw(|f| render(f, app))?;
        events::handle_events(app).await?;
    }
    Ok(())
}

fn render(f: &mut Frame, app: &App) {
    let full_area = f.area();
    
    // Clear the entire screen with background color
    f.render_widget(Clear, full_area);
    f.render_widget(
        Block::default().style(Style::default()),
        full_area,
    );

    // Calculate centered area with max dimensions
    let area = centered_rect(full_area, MAX_WIDTH, MAX_HEIGHT);

    // During splash, render only the splash screen (no header/footer)
    if app.show_splash {
        ui::render_home(f, area, app);
        return;
    }

    // Main layout: header, body, footer
    let chunks = Layout::vertical([
        Constraint::Length(3),  // Header
        Constraint::Min(10),    // Body
        Constraint::Length(3),  // Footer
    ])
    .split(area);

    // Render header
    ui::render_header(f, chunks[0], app);

    // Render body based on current tab
    let body_area = chunks[1];
    
    // Add padding to body
    let padded_body = pad_area(body_area, 2, 1);

    match app.current_tab {
        Tab::Home => ui::render_home(f, padded_body, app),
        Tab::Shop => ui::render_shop(f, padded_body, app),
        Tab::Account => ui::render_account(f, padded_body, app),
        Tab::Cart => {
            // Render checkout header for cart
            let cart_chunks = Layout::vertical([
                Constraint::Length(2),
                Constraint::Min(5),
            ])
            .split(padded_body);
            
            ui::render_checkout_header(f, cart_chunks[0], app);
            ui::render_cart(f, cart_chunks[1], app);
        }
    }

    // Render footer
    ui::render_footer(f, chunks[2], app);
}

/// Create a centered rect with max dimensions
fn centered_rect(area: Rect, max_width: u16, max_height: u16) -> Rect {
    let width = area.width.min(max_width);
    let height = area.height.min(max_height);
    
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    
    Rect::new(x, y, width, height)
}

/// Add padding to a rect
fn pad_area(area: Rect, horizontal: u16, vertical: u16) -> Rect {
    Rect {
        x: area.x + horizontal,
        y: area.y + vertical,
        width: area.width.saturating_sub(horizontal * 2),
        height: area.height.saturating_sub(vertical * 2),
    }
}
