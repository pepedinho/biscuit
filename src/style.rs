//! This module defines the visual appearance primitives for Zui.
//!
//! It provides the foundational building blocks (`Color`, `Modifier`, and `Style`)
//! to declaratively describe how text and widgets should look, without tying
//! them to specific terminal ANSI escape sequences.
//!
//! Styles in Zui are designed to be composable. By using optional values
//! for colors and bitsets for modifiers, styles can be patched and merged together
//! to cascade visual properties (similar to CSS inheritance).

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Color {
    #[default]
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    RGB(RGB),
    ANSI(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modifier(u16);

/// This structure is used to store the different attributes applicable to a [`Cell`].
/// They are stored in a u16 for performance reasons and are interacted with via BitWise.
impl Modifier {
    pub const BOLD: Self = Self(1 << 0); // 0000_0001
    pub const DIM: Self = Self(1 << 1); // 0000_0010
    pub const ITALIC: Self = Self(1 << 2); // 0000_0100
    pub const UNDERLINED: Self = Self(1 << 3); // 0000_1000
    pub const REVERSED: Self = Self(1 << 4); // 0001_0000
    pub const SLOW_BLINK: Self = Self(1 << 5); // 0010_0000
    pub const RAPID_BLINK: Self = Self(1 << 6); // 0100_0000
    pub const HIDDEN: Self = Self(1 << 7); // 1000_0000
    pub const CROSSED_OUT: Self = Self(1 << 8); //  0001_0000_0000

    pub const NONE: Self = Self(0);

    /// Add a modifiers (BITWISE OR)
    pub const fn insert(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// REMOVE A MODIFIER (BITWISE AND NOT)
    pub const fn remove(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// Check presence of a modifier
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl Default for Modifier {
    fn default() -> Self {
        Modifier::NONE
    }
}

pub trait Stylize: Sized {
    fn into_style(self) -> Style;

    fn bold(self) -> Style {
        self.into_style().bold()
    }

    fn dim(self) -> Style {
        self.into_style().dim()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub add_modifiers: Modifier,
    pub sub_modifiers: Modifier,
}

impl Stylize for Style {
    fn into_style(self) -> Style {
        self
    }
}

impl Stylize for Color {
    fn into_style(self) -> Style {
        Style::new().fg(self)
    }
}

impl From<Color> for Style {
    fn from(color: Color) -> Self {
        Style::new().fg(color)
    }
}

impl Style {
    const fn add_modifier(mut self, modifier: Modifier) -> Self {
        self.add_modifiers = self.add_modifiers.insert(modifier);
        self.sub_modifiers = self.sub_modifiers.remove(modifier);
        self
    }

    const fn remove_modifier(mut self, modifier: Modifier) -> Self {
        self.sub_modifiers = self.sub_modifiers.insert(modifier);
        self.add_modifiers = self.add_modifiers.remove(modifier);
        self
    }

    pub fn new() -> Self {
        Self {
            add_modifiers: Modifier::NONE,
            sub_modifiers: Modifier::NONE,
            ..Default::default()
        }
    }

    /// Set forground color
    pub const fn fg(mut self, c: Color) -> Self {
        self.fg = Some(c);
        self
    }

    /// Set background color
    pub const fn bg(mut self, c: Color) -> Self {
        self.bg = Some(c);
        self
    }

    /// turn on bold flag in internal [`Modifier`]
    pub const fn bold(self) -> Self {
        self.add_modifier(Modifier::BOLD)
    }

    /// turn on dim flag in internal [`Modifier`]
    pub const fn dim(self) -> Self {
        self.add_modifier(Modifier::DIM)
    }

    /// turn on italic flag in internal [`Modifier`]
    pub const fn italic(self) -> Self {
        self.add_modifier(Modifier::ITALIC)
    }

    /// turn on underlined flag in internal [`Modifier`]
    pub const fn underlined(self) -> Self {
        self.add_modifier(Modifier::UNDERLINED)
    }

    /// turn on reversed flag in internal [`Modifier`]
    pub const fn reversed(self) -> Self {
        self.add_modifier(Modifier::REVERSED)
    }

    /// turn on slow_blink flag in internal [`Modifier`]
    pub const fn slow_blink(self) -> Self {
        self.add_modifier(Modifier::SLOW_BLINK)
    }

    /// turn on rapid_blink flag in internal [`Modifier`]
    pub const fn rapid_blink(self) -> Self {
        self.add_modifier(Modifier::RAPID_BLINK)
    }

    /// turn on hidden flag in internal [`Modifier`]
    pub const fn hidden(self) -> Self {
        self.add_modifier(Modifier::HIDDEN)
    }

    /// turn on crossed_out flag in internal [`Modifier`]
    pub const fn crossed_out(self) -> Self {
        self.add_modifier(Modifier::CROSSED_OUT)
    }

    /// turn off dim flag in internal [`Modifier`]
    pub const fn ndim(self) -> Self {
        self.remove_modifier(Modifier::DIM)
    }

    /// turn off italic flag in internal [`Modifier`]
    pub const fn nitalic(self) -> Self {
        self.remove_modifier(Modifier::ITALIC)
    }

    /// turn off underlined flag in internal [`Modifier`]
    pub const fn nunderlined(self) -> Self {
        self.remove_modifier(Modifier::UNDERLINED)
    }

    /// turn off reversed flag in internal [`Modifier`]
    pub const fn nreversed(self) -> Self {
        self.remove_modifier(Modifier::REVERSED)
    }

    /// turn off slow_blink flag in internal [`Modifier`]
    pub const fn nslow_blink(self) -> Self {
        self.remove_modifier(Modifier::SLOW_BLINK)
    }

    /// turn off rapid_blink flag in internal [`Modifier`]
    pub const fn nrapid_blink(self) -> Self {
        self.remove_modifier(Modifier::RAPID_BLINK)
    }

    /// turn off hidden flag in internal [`Modifier`]
    pub const fn nhidden(self) -> Self {
        self.remove_modifier(Modifier::HIDDEN)
    }

    /// turn off crossed_out flag in internal [`Modifier`]
    pub const fn ncrossed_out(self) -> Self {
        self.remove_modifier(Modifier::CROSSED_OUT)
    }

    /// turn off bold flag in internal [`Modifier`]
    pub const fn nbold(self) -> Self {
        self.remove_modifier(Modifier::BOLD)
    }

    /// Returns `true` if the bold modifier is set.
    pub const fn is_bold(&self) -> bool {
        self.add_modifiers.contains(Modifier::BOLD)
    }

    /// Returns `true` if the dim modifier is set.
    pub const fn is_dim(&self) -> bool {
        self.add_modifiers.contains(Modifier::DIM)
    }

    /// Returns `true` if the italic modifier is set.
    pub const fn is_italic(&self) -> bool {
        self.add_modifiers.contains(Modifier::ITALIC)
    }

    /// Returns `true` if the underlined modifier is set.
    pub const fn is_underlined(&self) -> bool {
        self.add_modifiers.contains(Modifier::UNDERLINED)
    }

    /// Returns `true` if the reversed modifier is set.
    pub const fn is_reversed(&self) -> bool {
        self.add_modifiers.contains(Modifier::REVERSED)
    }

    /// Returns `true` if the slow_blink modifier is set.
    pub const fn is_slow_blink(&self) -> bool {
        self.add_modifiers.contains(Modifier::SLOW_BLINK)
    }

    /// Returns `true` if the rapid_blink modifier is set.
    pub const fn is_rapid_blink(&self) -> bool {
        self.add_modifiers.contains(Modifier::RAPID_BLINK)
    }

    /// Returns `true` if the hidden modifier is set.
    pub const fn is_hidden(&self) -> bool {
        self.add_modifiers.contains(Modifier::HIDDEN)
    }

    /// Returns `true` if the crossed_out modifier is set.
    pub const fn is_crossed_out(&self) -> bool {
        self.add_modifiers.contains(Modifier::CROSSED_OUT)
    }

    /// Merges another style into this one.
    /// Colors from `other` overwrite colors from `self`.
    /// `add_modifiers` from `other` are added, and `sub_modifiers` are removed.
    pub fn patch(mut self, other: Self) -> Self {
        if let Some(other_fg) = other.fg {
            self.fg = Some(other_fg);
        }

        if let Some(other_bg) = other.bg {
            self.bg = Some(other_bg);
        }

        self.add_modifiers = self
            .add_modifiers
            .insert(other.add_modifiers)
            .remove(other.sub_modifiers);

        self.sub_modifiers = self
            .sub_modifiers
            .insert(other.sub_modifiers)
            .remove(other.add_modifiers);

        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::style::Style;

    #[test]
    fn style_build_pattern() {
        let style = Style::new().fg(Color::Red).bg(Color::Black).bold();

        assert_eq!(style.fg, Some(Color::Red));
        assert_eq!(style.bg, Some(Color::Black));
    }

    #[test]
    fn style_patching() {
        let base = Style::new().fg(Color::Blue).bg(Color::Black).bold();

        let modifier = Style::new().fg(Color::Red).italic().nbold();

        let patched = base.patch(modifier);

        assert_eq!(patched.fg, Some(Color::Red));
        assert_eq!(patched.bg, Some(Color::Black));
        assert!(!patched.is_bold());
        assert!(patched.is_italic());
    }
}
