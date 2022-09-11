use rustyline::line_buffer::LineBuffer;
use tui::{
    style::Style,
    text::{Span, Spans},
};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::handlers::config::Alignment;

pub fn align_text(text: &str, maximum_length: u16, alignment: Alignment) -> String {
    assert!(
        maximum_length >= 1,
        "Parameter of 'maximum_length' cannot be below 1."
    );

    let dw = text.len();

    match alignment {
        Alignment::Right => {
            if dw > maximum_length as usize {
                text.to_string()
            } else {
                format!("{}{}", " ".repeat(maximum_length as usize - dw), text)
            }
        }
        Alignment::Center => {
            let side_spaces =
                " ".repeat(((maximum_length / 2) - (((dw / 2) as f32).floor() as u16)) as usize);
            format!("{}{}{}", side_spaces, text, side_spaces)
        }
        Alignment::Left => text.to_string(),
    }
}

/// Aligns all text in a column to a side depending on the longest string.
/// Sides can either be to the left, right, or center.
pub fn align_columns(
    mut v2: Vec<Vec<String>>,
    headers: Vec<String>,
    column_amount: usize,
    alignment: Alignment,
) -> (Vec<Vec<String>>, Vec<u16>) {
    let mut maximums = vec![];

    v2.push(headers);

    for i in 0..column_amount {
        let column_max = v2.iter().map(|v| v[i].len()).max().unwrap() as u16;

        maximums.push(column_max);

        (0..v2.len()).for_each(|j| {
            v2[j][i] = align_text(&v2[j][i], column_max, alignment.clone());
        });
    }

    v2.pop();

    (v2, maximums)
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
            Span::raw(format!("{}[ ", if i == 0 { "" } else { " " })),
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
    #[should_panic(expected = "Parameter of 'maximum_length' cannot be below 1.")]
    fn test_align_text_with_nothing() {
        assert_eq!(align_text("", 0, Alignment::Left), "");
    }

    #[test]
    fn test_align_text_left() {
        assert_eq!(align_text("text", 6, Alignment::Left), "text");
        assert_eq!(align_text("text", u16::MAX, Alignment::Left), "text");
    }

    #[test]
    fn test_align_text_center() {
        assert_eq!(align_text("text", 6, Alignment::Center), " text ");
    }

    #[test]
    fn test_align_text_right() {
        assert_eq!(align_text("text", 6, Alignment::Right), "  text");
    }

    #[test]
    fn test_align_text_right_smaller_max() {
        assert_eq!(align_text("text", 1, Alignment::Left), "text");
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
