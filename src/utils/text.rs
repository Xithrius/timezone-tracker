use color_eyre::eyre::{anyhow, Context, Result};
use regex::Regex;
use rustyline::line_buffer::LineBuffer;
use tui::{
    style::Style,
    text::{Span, Spans},
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[allow(dead_code)]
pub fn parse_timezone_offset(timezone_offset: &str) -> Result<i64> {
    let re = Regex::new("^(UTC)?([-+][0-9]{1,2})$").unwrap();

    let offset_string = re
        .captures(timezone_offset)
        .ok_or_else(|| anyhow!("Timezone string was formatted incorrectly."))?
        .get(1)
        .ok_or_else(|| anyhow!("Could not retrieve the integer offset from the timezone string."))?
        .as_str();

    offset_string.parse::<i64>().with_context(|| {
        format!(
            "Unable to convert {} to a valid integer offset.",
            timezone_offset
        )
    })
}

// pub fn parse_user_timezone(text: &str) -> (String, Result<i64>) {
//     let s = text.split(',').collect::<Vec<&str>>();

//     (s[0])
// }

pub fn align_columns(
    mut v2: Vec<Vec<String>>,
    row_len: usize,
    alignment: String,
) -> Vec<Vec<String>> {
    for i in 0..row_len {
        let column_max = if let Some(value) = v2.iter().map(|v| v[i].len()).max() {
            value as u16
        } else {
            0
        };

        for j in 0..v2[i].len() {
            let text = &v2[i][j];

            v2[i][j] = match alignment.as_str() {
                "right" => format!(
                    "{}{}",
                    " ".repeat((column_max - text.len() as u16) as usize),
                    text
                ),
                "center" => {
                    let side_spaces = " ".repeat(
                        ((column_max / 2) - (((text.len() / 2) as f32).floor() as u16)) as usize,
                    );

                    format!("{}{}{}", side_spaces, text, side_spaces)
                }
                _ => text.to_string(),
            };
        }
    }

    todo!()
}

pub fn get_cursor_position(line_buffer: &LineBuffer) -> usize {
    line_buffer
        .as_str()
        .grapheme_indices(true)
        .take_while(|(offset, _)| *offset != line_buffer.pos())
        .map(|(_, cluster)| cluster.width())
        .sum()
}

pub fn title_spans<'a>(contents: Vec<Vec<&str>>, style: Style) -> Spans<'a> {
    let mut complete = Vec::new();

    for (i, item) in contents.iter().enumerate() {
        complete.extend(vec![
            Span::raw(format!("{}[ ", if i != 0 { " " } else { "" })),
            Span::styled(item[0].to_string(), style),
            Span::raw(format!(": {} ]", item[1])),
        ]);
    }

    Spans::from(complete)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_timezone_parse() {
        if let Err(err) = parse_timezone_offset("asdfUTC+8") {
            assert_eq!(
                err.to_string(),
                "Timezone string was formatted incorrectly."
            );
        }
    }

    #[test]
    fn test_timezone_parse_into_zero() {
        assert_eq!(parse_timezone_offset("UTC-0").unwrap(), 0);
        assert_eq!(parse_timezone_offset("UTC+0").unwrap(), 0);
    }

    #[test]
    fn test_timezone_parse_into_negative_integer() {
        assert_eq!(parse_timezone_offset("UTC-8").unwrap(), -8);
    }

    #[test]
    fn test_timezone_parse_into_positive_integer() {
        assert_eq!(parse_timezone_offset("UTC+8").unwrap(), 8);
    }

    #[test]
    fn test_get_cursor_position_with_single_byte_graphemes() {
        let text = "never gonna give you up";
        let mut line_buffer = LineBuffer::with_capacity(25);
        line_buffer.insert_str(0, text);

        assert_eq!(get_cursor_position(&line_buffer), 0);
        line_buffer.move_forward(1);
        assert_eq!(get_cursor_position(&line_buffer), 1);
        line_buffer.move_forward(2);
        assert_eq!(get_cursor_position(&line_buffer), 3);
    }

    #[test]
    fn test_get_cursor_position_with_three_byte_graphemes() {
        let text = "绝对不会放弃你";
        let mut line_buffer = LineBuffer::with_capacity(25);
        line_buffer.insert_str(0, text);

        assert_eq!(get_cursor_position(&line_buffer), 0);
        line_buffer.move_forward(1);
        assert_eq!(get_cursor_position(&line_buffer), 2);
        line_buffer.move_forward(2);
        assert_eq!(get_cursor_position(&line_buffer), 6);
    }
}
