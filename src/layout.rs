//! This module provides primitives for spatial arrangement and layout constraints.
//!
//! The fundamental building block is the `Rect`, representing a 2D bounding box
//! on the terminal grid.

use std::cmp;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rect {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
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
