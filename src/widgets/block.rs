use crate::{Rect, Style, widgets::Widget};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Borders(u8);

impl Borders {
    pub const NONE: Self = Self(0);
    pub const TOP: Self = Self(1 << 0);
    pub const RIGHT: Self = Self(1 << 1);
    pub const BOTTOM: Self = Self(1 << 2);
    pub const LEFT: Self = Self(1 << 3);
    pub const ALL: Self = Self(0b1111);

    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for Borders {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

// ===============
// BLOCK WIDGET
// ===============

const BOX_TL: char = '┌';
const BOX_TR: char = '┐';
const BOX_BL: char = '└';
const BOX_BR: char = '┘';
const BOX_H: char = '─';
const BOX_V: char = '│';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    borders: Borders,
    style: Style,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            borders: Borders::NONE,
            style: Style::default(),
        }
    }
}

impl Block {
    /// crates a non-bordered block.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a block and apply borders on it.
    pub const fn borders(mut self, borders: Borders) -> Self {
        self.borders = borders;
        self
    }

    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Computes the area inside the borders
    pub fn inner(&self, area: Rect) -> Rect {
        let mut inner_area = area;

        if self.borders.contains(Borders::LEFT) {
            inner_area.x = inner_area.x.saturating_add(1);
            inner_area.width = inner_area.width.saturating_sub(1);
        }
        if self.borders.contains(Borders::TOP) {
            inner_area.y = inner_area.y.saturating_add(1);
            inner_area.height = inner_area.height.saturating_sub(1);
        }
        if self.borders.contains(Borders::RIGHT) {
            inner_area.width = inner_area.width.saturating_sub(1);
        }
        if self.borders.contains(Borders::BOTTOM) {
            inner_area.height = inner_area.height.saturating_sub(1);
        }

        inner_area
    }
}

impl Widget for Block {
    fn render(self, area: Rect, buf: &mut crate::Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        for y in area.y..(area.y + area.height) {
            for x in area.x..(area.x + area.width) {
                if let Some(cell) = buf.get_mut(x, y) {
                    cell.set_style(self.style);
                }
            }
        }

        let right_x = area.x + area.width - 1;
        let bottom_y = area.y + area.height - 1;

        if self.borders.contains(Borders::TOP) {
            for x in area.x..=right_x {
                if let Some(c) = buf.get_mut(x, area.y) {
                    c.set_symbol(BOX_H);
                }
            }
        }

        if self.borders.contains(Borders::BOTTOM) {
            for x in area.x..=right_x {
                if let Some(c) = buf.get_mut(x, bottom_y) {
                    c.set_symbol(BOX_H);
                }
            }
        }

        // --- Vertical lines ---
        if self.borders.contains(Borders::LEFT) {
            for y in area.y..=bottom_y {
                if let Some(c) = buf.get_mut(area.x, y) {
                    c.set_symbol(BOX_V);
                }
            }
        }
        if self.borders.contains(Borders::RIGHT) {
            for y in area.y..=bottom_y {
                if let Some(c) = buf.get_mut(right_x, y) {
                    c.set_symbol(BOX_V);
                }
            }
        }

        // --- Corners ---
        if self.borders.contains(Borders::TOP | Borders::LEFT)
            && let Some(c) = buf.get_mut(area.x, area.y)
        {
            c.set_symbol(BOX_TL);
        }
        if self.borders.contains(Borders::TOP | Borders::RIGHT)
            && let Some(c) = buf.get_mut(right_x, area.y)
        {
            c.set_symbol(BOX_TR);
        }
        if self.borders.contains(Borders::BOTTOM | Borders::LEFT)
            && let Some(c) = buf.get_mut(area.x, bottom_y)
        {
            c.set_symbol(BOX_BL);
        }
        if self.borders.contains(Borders::BOTTOM | Borders::RIGHT)
            && let Some(c) = buf.get_mut(right_x, bottom_y)
        {
            c.set_symbol(BOX_BR);
        }
    }
}
