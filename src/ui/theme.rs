#![allow(dead_code)]

use ratatui::style::Color;

/// ANORA Labs color theme
pub struct Theme;

impl Theme {
    // Brand colors
    pub const PINK: Color = Color::Rgb(255, 36, 189);          // #ff24bd - Brand color
    // pub const BLUE: Color = Color::Rgb(13, 153, 255);          // #0d99ff - segfault
    // pub const GREEN: Color = Color::Rgb(20, 174, 92);          // #14ae5c - dark mode
    // pub const YELLOW: Color = Color::Rgb(255, 205, 41);        // #ffcd29 - [object Object]
    pub const PALE_PINK: Color = Color::Rgb(171, 89, 152);     // #ab5998 - 404
    // -ramp-pale_yellow-500-light: #ad7f00;
    pub const YELLOW: Color = Color::Rgb(173, 127, 0);    // #ad7f00
    // --ramp-pale_green-500-light: #678e79;
    pub const GREEN: Color = Color::Rgb(103, 142, 121);    // #678e79
    // --ramp-pale_blue-500-light: #667799;
    pub const BLUE: Color = Color::Rgb(102, 119, 153);    // #667799
    // --ramp-red-500-light: #f24822;
    pub const RED: Color = Color::Rgb(242, 72, 34);    // #f24822

    // UI colors
    pub const BG: Color = Color::Rgb(22, 22, 26);              // Dark background
    pub const FG: Color = Color::Rgb(255, 255, 255);           // White text
    pub const DIMMED: Color = Color::Rgb(128, 128, 128);       // Dimmed/gray text
    pub const BORDER: Color = Color::Rgb(64, 64, 64);          // Border color
    pub const HIGHLIGHT_BG: Color = Color::Rgb(45, 45, 50);    // Highlighted item background

    /// Get highlight color for a product by name
    pub fn product_color(product_name: &str) -> Color {
        match product_name.to_lowercase().as_str() {
            "cron" => Self::PINK,
            "[object object]" => Self::YELLOW,
            "segfault" => Self::BLUE,
            "dark mode" => Self::GREEN,
            "404" => Self::PALE_PINK,
            _ => Self::PINK,
        }
    }
}

