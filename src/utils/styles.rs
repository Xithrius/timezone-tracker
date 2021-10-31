use tui::style::{Color, Modifier, Style};

pub const BORDER_NAME: Style = Style {
    fg: Some(Color::White),
    bg: None,
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};
pub const COLUMN_TITLE: Style = Style {
    fg: Some(Color::LightCyan),
    bg: None,
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
