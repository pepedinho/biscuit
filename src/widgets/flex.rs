use crate::{Buffer, Constraint, Direction, Layout, Rect, widgets::Widget};

pub trait RenderBox {
    fn render_box(self: Box<Self>, area: Rect, buf: &mut Buffer);
}

impl<T: Widget> RenderBox for T {
    fn render_box(self: Box<Self>, area: Rect, buf: &mut Buffer) {
        (*self).render(area, buf);
    }
}

pub struct Child<'a> {
    constraint: Constraint,
    widget: Box<dyn RenderBox + 'a>,
}

pub struct VStack<'a> {
    children: Vec<Child<'a>>,
}

impl<'a> Default for VStack<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> VStack<'a> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: impl IntoChild<'a>) -> Self {
        self.children.push(child.into_child());
        self
    }
}

impl<'a> Widget for VStack<'a> {
    fn render(self, area: Rect, buf: &mut crate::Buffer) {
        let constraints: Vec<Constraint> = self.children.iter().map(|c| c.constraint).collect();

        let chunks = Layout::new()
            .direction(Direction::Vertical)
            .constraints(&constraints)
            .split(area);

        for (child, chunk) in self.children.into_iter().zip(chunks) {
            child.widget.render_box(chunk, buf);
        }
    }
}

pub struct HStack<'a> {
    children: Vec<Child<'a>>,
}

impl<'a> Default for HStack<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> HStack<'a> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: impl IntoChild<'a>) -> Self {
        self.children.push(child.into_child());
        self
    }
}

impl<'a> Widget for HStack<'a> {
    fn render(self, area: Rect, buf: &mut crate::Buffer) {
        let constraints: Vec<Constraint> = self.children.iter().map(|c| c.constraint).collect();

        let chunks = Layout::new()
            .direction(Direction::Horizontal)
            .constraints(&constraints)
            .split(area);

        for (child, chunk) in self.children.into_iter().zip(chunks) {
            child.widget.render_box(chunk, buf);
        }
    }
}

pub trait IntoChild<'a> {
    fn into_child(self) -> Child<'a>;
}

impl<'a, W: Widget + 'a> IntoChild<'a> for W {
    fn into_child(self) -> Child<'a> {
        Child {
            constraint: Constraint::Fill(1),
            widget: Box::new(self),
        }
    }
}

impl<'a, W: Widget + 'a> IntoChild<'a> for (Constraint, W) {
    fn into_child(self) -> Child<'a> {
        Child {
            constraint: self.0,
            widget: Box::new(self.1),
        }
    }
}
