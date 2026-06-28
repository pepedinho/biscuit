use crate::{Buffer, Rect, Style, widgets::Block};

pub struct Paragraph<'a> {
    text: &'a str,
    style: Style,
    block: Option<Block>,
}

impl<'a> Paragraph<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            style: Style::default(),
            block: None,
        }
    }

    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    pub const fn block(mut self, block: Block) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> crate::widgets::Widget for Paragraph<'a> {
    fn render(self, mut area: Rect, buf: &mut Buffer) {
        if let Some(block) = self.block {
            let inner = block.inner(area);
            block.render(area, buf);
            area = inner;
        }

        if area.width == 0 || area.height == 0 {
            return;
        }
        buf.set_string(area.x, area.y, self.text, self.style);
    }
}
