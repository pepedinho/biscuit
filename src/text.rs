//! This module defines the core text rendering primitives.
//!
//! Text in Zui is broken down into a hierarchy to allow fine-grained styling:
//! - `Span`: A contiguous string slice with a single unified `Style`.
//! - `Line`: A sequence of `Span`s, representing a single horizontal line of text.
//! - `Text`: A sequence of `Line`s, representing a multi-line paragraph.

use crate::Style;

pub struct Span {
    pub content: String,
    pub style: Style,
}

impl Span {
    /// Creates a new [`Span`] with the default empty style.
    pub fn raw(content: String) -> Self {
        Self {
            content,
            style: Default::default(),
        }
    }

    /// Creates a new [`Span`] with a specific style.
    pub fn styled(content: String, style: Style) -> Self {
        Self { content, style }
    }

    /// Patches the span's current style with another style.
    pub fn patch_style(mut self, other: Style) -> Self {
        self.style = self.style.patch(other);
        self
    }

    /// Calulates the visible width of the span
    /// NOTE: UTF-8 codepoint counting to correctly size accented characters.
    /// This does not account for double-width characters (like emojis or Kanjis).
    pub fn width(&self) -> usize {
        std::str::from_utf8(self.content.as_bytes())
            .map(|s| s.chars().count())
            .unwrap_or(self.content.len())
    }
}

pub struct Line {
    pub spans: Vec<Span>,
}

impl Line {
    /// Creates a new [`Line`] from a slie of `Span`s.
    pub fn new(spans: Vec<Span>) -> Self {
        Self { spans }
    }

    /// return the visible width of the line.
    pub fn width(&self) -> usize {
        self.spans.iter().fold(0, |acc, s| acc + s.width())
    }

    /// Patches the line current style with another style.
    pub fn patch_style(mut self, style: Style) -> Self {
        let new_spans: Vec<Span> = self
            .spans
            .into_iter()
            .map(|s| s.patch_style(style))
            .collect();
        self.spans = new_spans;
        self
    }
}

pub struct Text {
    lines: Vec<Line>,
}

impl Text {
    /// Creates a new [`Text`] from a slie of `Line`s.
    pub fn new(lines: Vec<Line>) -> Self {
        Self { lines }
    }

    /// Calulates the maximum width among all lines.
    /// This is useful for layout calculations (e.g. finding the widest line in a block).
    pub fn width(&self) -> usize {
        let mut max_w: usize = 0;

        self.lines.iter().for_each(|l| {
            let w = l.width();
            if w > max_w {
                max_w = w;
            }
        });

        max_w
    }

    /// Calulates the height of the text (number of lines).
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    /// Patches the text current style with another style.
    pub fn patch_style(mut self, style: Style) -> Self {
        let new_lines: Vec<Line> = self
            .lines
            .into_iter()
            .map(|s| s.patch_style(style))
            .collect();

        self.lines = new_lines;
        self
    }
}

#[cfg(test)]
mod test {

    use crate::Color;

    use super::*;

    #[test]
    fn span_creation_and_width() {
        let span1 = Span::raw("Hello".into());
        assert_eq!(span1.content, "Hello");
        assert_eq!(span1.width(), 5);

        let styled_span = Span::styled("World".into(), Style::new().fg(Color::Red));
        assert_eq!(styled_span.content, "World");
    }

    #[test]
    fn line_width_calculation() {
        let spans = vec![
            Span::raw("Hello, ".into()),
            Span::styled("Biscuit".into(), Style::new().fg(Color::Red)),
            Span::raw("!".into()),
        ];

        let line = Line::new(spans);

        assert_eq!(line.width(), 15);
    }

    #[test]
    fn text_dimensions() {
        let l1_spans = vec![Span::raw("Hello".into())];
        let l2_spans = vec![Span::styled("World!".into(), Style::new().bold())];

        let lines = vec![Line::new(l1_spans), Line::new(l2_spans)];

        let text = Text::new(lines);

        assert_eq!(text.width(), 6);
        assert_eq!(text.height(), 2);
    }
}
