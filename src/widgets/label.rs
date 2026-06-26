use crate::{Style, widgets::Widget};

/// A widget that represent a simple line of text
pub struct Label<'a> {
    text: &'a str,
    style: Style,
}

impl<'a> Label<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            style: Style::new(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a> Widget for Label<'a> {
    fn render(self, area: crate::Rect, buf: &mut crate::Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        buf.set_string(area.x, area.y, self.text, self.style);
    }
}
