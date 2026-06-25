use std::cmp;

use crate::{Line, Span, Style};

/// Represent a single terminal tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// The content of the cell. Usually a single ASCII character,
    /// but stored in a char to support multi-byte UTF-8 graphemes.
    /// Default is a single space.
    symbol: char,
    /// The visual style (colors and modifiers) applied to this cell.
    style: Style,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            symbol: ' ',
            style: Default::default(),
        }
    }
}

impl Cell {
    /// Reset the cell to an empty space with default styling.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Sets the symbol stored in this cell.
    pub fn set_symbol(&mut self, symbol: char) {
        self.symbol = symbol;
    }

    /// Gets the symbol stored in this cell.
    pub fn get_symbol(&self) -> char {
        self.symbol
    }

    pub fn get_style(&self) -> &Style {
        &self.style
    }

    /// Sets the style of the cell.
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }
}

/// A 2D grid of `Cell`s, represented internally as a 1D array for memory efficiency.
pub struct Buffer {
    pub width: u16,
    pub height: u16,
    content: Vec<Cell>,
}

impl Buffer {
    /// Allocates a new buffer of the given dimensions.
    pub fn new(w: u16, h: u16) -> Self {
        let size = (w * h) as usize;
        Self {
            width: w,
            height: h,
            content: vec![Cell::default(); size],
        }
    }

    /// Writes a raw string to the buffer starting at (x, y).
    /// Uses a UTF-8 iterator to place one codepoint per cell.
    /// Stop if it reaches the end of the buffer width
    pub fn set_string(&mut self, x: u16, y: u16, string: &str, style: Style) {
        let mut current_x = x;
        let chars = string.chars();

        for c in chars {
            if current_x >= self.width {
                break;
            }

            if let Some(cell) = self.get_mut(current_x, y) {
                cell.set_symbol(c);
                cell.set_style(style);
            }
            current_x += 1;
        }
    }

    /// Writes a styled `Span` to the buffer starting at (x, y)
    pub fn set_span(&mut self, x: u16, y: u16, span: &Span) {
        self.set_string(x, y, &span.content, span.style);
    }

    /// Writes a [`Line`] (a sequence of Spans) to the buffer starting at (x, y).
    /// Returns the new X position after writing the line.
    pub fn set_line(&mut self, x: u16, y: u16, line: &Line, max_width: u16) -> usize {
        let mut current_x = x;
        let end_x = cmp::min(self.width, x + max_width);

        for span in &line.spans {
            if current_x >= end_x {
                break;
            }

            let chars = span.content.chars();
            for c in chars {
                if current_x >= end_x {
                    break;
                }

                if let Some(cell) = self.get_mut(current_x, y) {
                    cell.set_symbol(c);
                    cell.set_style(span.style);
                }
                current_x += 1;
            }
        }

        current_x.into()
    }

    //TODO: set_style_area (depend on layout module)

    pub fn get(&self, x: u16, y: u16) -> Option<&Cell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            let idx = ((y * self.width) + x) as usize;
            self.content.get(idx)
        }
    }

    pub fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            let idx = ((y * self.width) + x) as usize;
            self.content.get_mut(idx)
        }
    }

    pub fn reset(&mut self) {
        for cell in self.content.iter_mut() {
            cell.reset();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn buffer_initialization_and_access() {
        let mut buf = Buffer::new(10, 5);

        assert_eq!(buf.width, 10);
        assert_eq!(buf.height, 5);
        assert_eq!(buf.content.len(), 50);

        if let Some(cell) = buf.get_mut(2, 3) {
            cell.set_symbol('X');
        } else {
            panic!("Failed to access cell {{x: 2, y: 3}}");
        }

        let check_cell = buf.get(2, 3).unwrap();
        assert_eq!(check_cell.get_symbol(), 'X');
        assert_eq!(buf.get(10, 5), None);
    }

    #[test]
    fn buffer_contain_ut8() {
        let mut buf = Buffer::new(10, 5);

        buf.set_string(0, 0, "Hé", Style::default());

        let cell_0 = buf.get(0, 0).unwrap();
        let cell_1 = buf.get(1, 0).unwrap();
        let cell_2 = buf.get(2, 0).unwrap();

        assert_eq!(cell_0.get_symbol(), 'H');
        assert_eq!(cell_1.get_symbol(), 'é');
        assert_eq!(cell_2.get_symbol(), ' ');
    }
}
