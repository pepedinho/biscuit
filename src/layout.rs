//! This module provides primitives for spatial arrangement and layout constraints.
//!
//! The fundamental building block is the `Rect`, representing a 2D bounding box
//! on the terminal grid.

use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    Horizontal,
    #[default]
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constraint {
    Length(u16),
    Percentage(u16),
    Min(u16),
    Fill(u16),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn area(&self) -> u32 {
        (self.width as u32) * (self.height as u32)
    }

    pub fn contains(&self, target_x: u16, target_y: u16) -> bool {
        target_x >= self.x
            && target_x < self.x + self.width
            && target_y >= self.y
            && target_y < self.y + self.height
    }

    /// Computes the intersection of this rectangle with another.
    /// Return a new [`Rect`] that is the overlapping area.
    /// If they do not intersect, return a [`Option::None`].
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let x1 = cmp::max(self.x, other.x);
        let y1 = cmp::max(self.y, other.y);

        let self_x2 = self.x + self.width;
        let other_x2 = other.x + other.width;
        let x2 = cmp::min(self_x2, other_x2);

        let self_y2 = self.y + self.height;
        let other_y2 = other.y + other.height;
        let y2 = cmp::min(self_y2, other_y2);

        if x1 >= x2 || y1 >= y2 {
            None
        } else {
            Some(Rect::new(x1, y1, x2 - x1, y2 - y1))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Layout<'a> {
    direction: Direction,
    constraints: &'a [Constraint],
}

impl<'a> Default for Layout<'a> {
    fn default() -> Self {
        Self {
            direction: Direction::Vertical,
            constraints: &[],
        }
    }
}

impl<'a> Layout<'a> {
    /// Creates a nesw [`Layout`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder pattern for direction
    pub const fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Builder pattern for constraints
    pub const fn constraints(mut self, constraints: &'a [Constraint]) -> Self {
        self.constraints = constraints;
        self
    }

    /// Split a [`Rect`] into a Vec of sub-[`Rect`] while respecting the constraints.
    pub fn split(&self, area: Rect) -> Vec<Rect> {
        let mut results = Vec::with_capacity(self.constraints.len());

        let available_space = if self.direction == Direction::Horizontal {
            area.width
        } else {
            area.height
        };

        let mut fill_total: u16 = 0;
        let mut fixed_space: u16 = 0;
        let mut sizes = vec![0; self.constraints.len()];

        for (i, constraint) in self.constraints.iter().enumerate() {
            match constraint {
                Constraint::Length(l) => {
                    sizes[i] = *l;
                    fixed_space += *l;
                }
                Constraint::Percentage(p) => {
                    let l = ((available_space as u32 * *p as u32) / 100) as u16;
                    sizes[i] = l;
                    fixed_space += l;
                }
                Constraint::Min(m) => {
                    sizes[i] = *m;
                    fixed_space += *m;
                    fill_total += 1;
                }
                Constraint::Fill(f) => {
                    sizes[i] = 0;
                    fill_total += *f;
                }
            }
        }

        let remaining = available_space.saturating_sub(fixed_space);

        if remaining > 0 && fill_total > 0 {
            let mut space_to_distribute = remaining;
            let mut fill_left = fill_total;

            for (i, constraint) in self.constraints.iter().enumerate() {
                let weight = match constraint {
                    Constraint::Min(_) => 1,
                    Constraint::Fill(f) => *f,
                    _ => 0,
                };

                if weight > 0 {
                    let extra =
                        ((space_to_distribute as u32 * weight as u32) / fill_left as u32) as u16;
                    sizes[i] += extra;
                    space_to_distribute -= extra;
                    fill_left -= weight;
                }
            }
        }

        let mut current_x = area.x;
        let mut current_y = area.y;

        for size in sizes {
            if self.direction == Direction::Horizontal {
                results.push(Rect::new(current_x, current_y, size, area.height));
                current_x += size;
            } else {
                results.push(Rect::new(current_x, current_y, area.width, size));
                current_y += size;
            }
        }

        results
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rect_contain_point() {
        let r = Rect::new(5, 5, 10, 10);

        assert!(r.contains(5, 5));
        assert!(r.contains(14, 14));

        assert!(!r.contains(4, 5));
        assert!(!r.contains(15, 15));
    }

    #[test]
    fn rect_intersection() {
        let r1 = Rect::new(0, 0, 10, 10);
        let r2 = Rect::new(5, 5, 10, 10);

        let intersect = r1.intersection(&r2).unwrap();

        assert_eq!(intersect.x, 5);
        assert_eq!(intersect.y, 5);
        assert_eq!(intersect.width, 5);
        assert_eq!(intersect.height, 5);

        let r3 = Rect::new(20, 20, 5, 5);
        let no_intersect = r1.intersection(&r3);

        assert_eq!(no_intersect, None);
    }
}
