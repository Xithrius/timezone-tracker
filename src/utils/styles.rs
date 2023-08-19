use tui::style::{Color, Modifier, Style};

pub const BORDER_NAME: Style = Style {
    fg: Some(Color::White),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
    underline_color: None,
};
pub const COLUMN_TITLE: Style = Style {
    fg: Some(Color::LightCyan),
    bg: None,
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
    underline_color: None,
};

pub const VALID: Style = Style {
    fg: Some(Color::Green),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
    underline_color: None,
};
pub const EXISTS: Style = Style {
    fg: Some(Color::Yellow),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
    underline_color: None,
};
pub const INVALID: Style = Style {
    fg: Some(Color::Red),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
    underline_color: None,
};
