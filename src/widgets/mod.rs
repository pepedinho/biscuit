use crate::{Buffer, Rect};
mod block;
mod flex;
mod label;
mod paragraph;

pub use block::{Block, Borders};
pub use flex::{HStack, VStack};
pub use label::Label;
pub use paragraph::Paragraph;

/// [`Widget`] is a mandatory trait for all UI element in biscuit
/// each widget have a render() function that is provided to the render engine
/// to be displayed.
pub trait Widget {
    fn render(self, area: Rect, buf: &mut Buffer);
}
