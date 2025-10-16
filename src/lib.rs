use colored::{Color, Colorize};
use indicatif::{ProgressBar, ProgressStyle};
use prettytable::{format, Table};
use std::thread;
use std::time::Duration;

/// Enum for common colors (extendable)
#[derive(Clone, Copy)]
pub enum LumaColor {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    White,
    Black,
}

impl Into<Color> for LumaColor {
    fn into(self) -> Color {
        match self {
            LumaColor::Red => Color::Red,
            LumaColor::Green => Color::Green,
            LumaColor::Blue => Color::Blue,
            LumaColor::Yellow => Color::Yellow,
            LumaColor::Magenta => Color::Magenta,
            LumaColor::Cyan => Color::Cyan,
            LumaColor::White => Color::White,
            LumaColor::Black => Color::Black,
        }
    }
}

/// Print colored text to stdout.
/// 
/// # Arguments
/// * `text` - The text to print.
/// * `fg_color` - Foreground color.
/// * `bg_color` - Optional background color.
pub fn print_colored(text: &str, fg_color: LumaColor, bg_color: Option<LumaColor>) {
    let mut styled = text.color(fg_color);
    if let Some(bg) = bg_color {
        styled = styled.on_color(bg);
    }
    println!("{}", styled);
}

/// Draw a frame (box) around text using Unicode borders.
/// 
/// # Arguments
/// * `content` - The lines of text inside the box.
/// * `border_color` - Color for the border.
/// * `title` - Optional title for the box.
pub fn draw_frame(content: Vec<&str>, border_color: LumaColor, title: Option<&str>) {
    let max_len = content.iter().map(|s| s.len()).max().unwrap_or(0) + 4; // Padding
    let horizontal = "─".repeat(max_len - 2);

    let title_str = if let Some(t) = title {
        format!("┌{} {} ┐", t, horizontal[(t.len() + 1)..].to_string())
    } else {
        format!("┌{}┐", horizontal)
    };
    println!("{}", title_str.color(border_color));

    for line in content {
        let padded = format!("│ {:<width$} │", line, width = max_len - 4);
        println!("{}", padded.color(border_color));
    }

    let bottom = format!("└{}┘", horizontal);
    println!("{}", bottom.color(border_color));
}

/// Create and display a progress bar.
/// 
/// # Arguments
/// * `len` - Total steps.
/// * `message` - Message to display.
/// * `color` - Color for the bar.
pub fn progress_bar(len: u64, message: &str, color: LumaColor) {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(&format!("{{msg}} [{{bar:40.{:?}/black}}] {{pos}}/{{len}}", color))
            .unwrap()
            .progress_chars("##-"),
    );
    pb.set_message(message.to_string());

    for _ in 0..len {
        pb.inc(1);
        thread::sleep(Duration::from_millis(100));
    }
    pb.finish_with_message("Done!");
}

/// Create a styled table.
/// 
/// # Arguments
/// * `headers` - Table headers.
/// * `rows` - Table rows (Vec<Vec<String>>).
/// * `border_color` - Color for borders (simulated via text).
pub fn draw_table(headers: Vec<&str>, rows: Vec<Vec<String>>, border_color: LumaColor) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.add_row(headers.into_iter().collect());

    for row in rows {
        table.add_row(row.into_iter().collect());
    }

    // Simulate coloring by printing with color
    let table_str = table.to_string();
    println!("{}", table_str.color(border_color));
}
