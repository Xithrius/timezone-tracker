use chrono::{Local, NaiveDateTime, Utc};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    terminal::Frame,
    widgets::{Block, Borders, Paragraph, Row, Table},
};

use crate::{
    handlers::{
        app::{App, State},
        config::CompleteConfig,
    },
    utils::{
        styles,
        text::{align_columns, get_cursor_position, title_spans},
    },
};

pub fn draw_ui<T: Backend>(f: &mut Frame<T>, app: &mut App, config: &CompleteConfig) {
    let mut vertical_chunk_constraints = vec![Constraint::Min(1)];

    if let State::Input = app.state {
        vertical_chunk_constraints.push(Constraint::Length(3));
    }

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(config.frontend.padding)
        .constraints(vertical_chunk_constraints.as_ref())
        .split(f.size());

    let time_rows = app
        .storage
        .get_all()
        .iter()
        .map(|(k, v)| {
            vec![
                k.to_string(),
                v.to_string(),
                NaiveDateTime::from_timestamp(Utc::now().timestamp() + v * 3600, 0)
                    .format(config.frontend.time_format.as_str())
                    .to_string(),
            ]
        })
        .collect::<Vec<Vec<String>>>();

    let titles = vec!["User", "Offset", "Time"];

    let table = Table::new(
        if time_rows.len() > 1 {
            align_columns(time_rows, titles.len(), config.frontend.alignment.clone())
        } else {
            time_rows
        }
        .iter()
        .map(|cells| Row::new(cells.iter().map(|s| s.to_string()))),
    )
    .header(Row::new(titles).style(styles::COLUMN_TITLE))
    .block(
        Block::default()
            .style(styles::BORDER_NAME)
            .borders(Borders::ALL)
            .title(title_spans(
                vec![
                    vec!["Local offset", &config.terminal.timezone_offset.to_string()],
                    vec![
                        "Local time",
                        &Local::now()
                            .format(config.frontend.time_format.as_str())
                            .to_string(),
                    ],
                ],
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )),
    )
    .widths(
        [
            Constraint::Length(15),
            Constraint::Length(6),
            Constraint::Percentage(100),
        ]
        .as_ref(),
    )
    .column_spacing(1);

    f.render_widget(table, vertical_chunks[0]);

    if let State::Input = app.state {
        let text = &app.input_buffer;

        let cursor_pos = get_cursor_position(text);

        let input_rect = vertical_chunks[vertical_chunk_constraints.len() - 1];

        f.set_cursor(
            (input_rect.x + cursor_pos as u16 + 1)
                .min(input_rect.x + input_rect.width.saturating_sub(2)),
            input_rect.y + 1,
        );

        let paragraph = Paragraph::new(text.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .style(styles::BORDER_NAME)
                    .borders(Borders::ALL)
                    .title("[ Input ]"),
            )
            .scroll((
                0,
                ((cursor_pos + 3) as u16).saturating_sub(input_rect.width),
            ));

        f.render_widget(
            paragraph,
            vertical_chunks[vertical_chunk_constraints.len() - 1],
        );
    }
}
