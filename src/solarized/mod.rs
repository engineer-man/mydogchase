use crossterm::style::{
    SetBackgroundColor, SetForegroundColor, ResetColor, Color,
    Attribute, SetAttribute,
};
use crossterm::{
    ExecutableCommand,
    cursor,
    terminal::{
        Clear,
        ClearType
    }
};
use rand::{Rng, thread_rng};
use std::io::stdout;

pub const BACK: Color = Color::Rgb { r:7, g:54, b:66 };
pub const VIOLET: Color = Color::Rgb { r:108, g:113, b:196 };
pub const BLUE: Color = Color::Rgb { r:38, g:139, b:210 };
pub const CYAN: Color = Color::Rgb { r:42, g:161, b:152 };
pub const GREEN: Color = Color::Rgb { r:133, g:153, b:0 };
pub const YELLOW: Color = Color::Rgb { r:181, g:137, b:0 };
pub const ORANGE: Color = Color::Rgb { r:203, g:75, b:22 };
pub const RED: Color = Color::Rgb { r:211, g:1, b:2 };
pub const MAGENTA: Color = Color::Rgb { r:211, g:54, b:130 };
pub const WHITE: Color = Color::Rgb { r:147, g:161, b:161 };
pub const GREY: Color = Color::Rgb { r:88, g:110, b:117 };
pub const BOLD: Attribute = Attribute::Bold;
pub const UNDERLINED: Attribute = Attribute::Underlined;
pub const ITALIC: Attribute = Attribute::Italic;

pub enum PrintMode {
    NewLine,
    SameLine,
}

fn format_message(message_fragments: &[(&str, Color, Vec<Attribute>)]) -> String {
    let mut formatted_message = String::new();
    for (message, color, attributes) in message_fragments {
        let lines: Vec<&str> = message.split('\n').collect();
        for (i, line) in lines.iter().enumerate() {
            formatted_message += &SetBackgroundColor(BACK).to_string();
            formatted_message += &SetForegroundColor(*color).to_string();
            for attribute in attributes {
                formatted_message += &SetAttribute(*attribute).to_string();
            }
            formatted_message.push_str(line);
            formatted_message += &ResetColor.to_string();
            if i < lines.len() - 1 {
                formatted_message.push('\n');
            }
        }
    }
    formatted_message
}

fn print_formatted(message_fragments: &[(&str, Color, Vec<Attribute>)], mode: PrintMode) {
    let formatted_message = format_message(message_fragments);
    match mode {
        PrintMode::NewLine => {
            println!(
                "{}{}{}",
                SetBackgroundColor(BACK),
                formatted_message,
                ResetColor,
            );
        },
        PrintMode::SameLine => {
            print!(
                "{}{}{}",
                SetBackgroundColor(BACK),
                formatted_message,
                ResetColor,
            );
        },
    }
}

pub fn print_colored(message: &[&str], colors: &[Color], mode: PrintMode) {
    let fragments: Vec<_> = message.iter().enumerate()
        .map(|(i, &m)| (m, colors[i % colors.len()], vec![]))
        .collect();
    print_formatted(&fragments, mode);
}

pub fn print_fancy(message_fragments: &[(&str, Color, Vec<Attribute>)], mode: PrintMode) {
    print_formatted(message_fragments, mode);
}

pub fn clear() {
    stdout()
        .execute(Clear(ClearType::All)).unwrap()
        .execute(cursor::MoveTo(0, 0)).unwrap();
}

fn random_color() -> Color {
    let mut rng = thread_rng();
    Color::Rgb {
        r: rng.gen_range(0..255),
        g: rng.gen_range(0..255),
        b: rng.gen_range(0..255),
    }
}

pub fn print_random_colored(message: &str, mode: PrintMode) {
    let mut formatted_message = String::new();
    for ch in message.chars() {
        let fg = random_color();
        let bg = random_color();
        formatted_message += &SetBackgroundColor(bg).to_string();
        formatted_message += &SetForegroundColor(fg).to_string();
        formatted_message.push(ch);
        formatted_message += &ResetColor.to_string();
    }
    match mode {
        PrintMode::NewLine => {
            println!(
                "{}{}{}",
                SetBackgroundColor(BACK),
                formatted_message,
                ResetColor,
            );
        },
        PrintMode::SameLine => {
            print!(
                "{}{}{}",
                SetBackgroundColor(BACK),
                formatted_message,
                ResetColor,
            );
        },
    }
}
