//! This module handles the translation of Biscuit primitives into ANSI escape sequences.
//!
//! It provides functions to format colors, modifiers, and cursor movements
//! to be written directly to a standard `std::io::Write` implementor.

use crate::style::{Color, Modifier};
use std::io::{self, Write};

pub const ESC: &str = "\x1B[";

pub struct AnsiBackend<W: Write> {
    writer: W,
}

impl<W: Write> AnsiBackend<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }

    pub fn write_char(&mut self, c: char) -> io::Result<()> {
        write!(self.writer, "{}", c)
    }

    // =======================
    // ANSI CODE GENERATOR
    // =======================

    /// Hides the terminal cursor.
    pub fn hide_cursor(&mut self) -> io::Result<()> {
        write!(self.writer, "{}?25l", ESC)
    }

    /// Shows the terminal cursor.
    pub fn show_cursor(&mut self) -> io::Result<()> {
        write!(self.writer, "{}?25h", ESC)
    }

    /// Clears the entire screen and moves the cursor to the home position (0, 0).
    pub fn clear_screen(&mut self) -> io::Result<()> {
        // 2J clears the entire screen. H moves cursor to home.
        write!(self.writer, "{}2J{}H", ESC, ESC)
    }

    /// Enters the alternate screen buffer (useful for full-screen TUI apps).
    pub fn enter_alternate_screen(&mut self) -> io::Result<()> {
        write!(self.writer, "{}?1049h", ESC)
    }

    /// Leaves the alternate screen buffer, restoring the previous terminal state.
    pub fn leave_alternate_screen(&mut self) -> io::Result<()> {
        write!(self.writer, "{}?1049l", ESC)
    }

    /// Moves the cursor to the specified (x, y) coordinates.
    pub fn move_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        // ANSI coordinates are 1-based, internal representation is 0-based.
        write!(self.writer, "{}{};{}H", ESC, y + 1, x + 1)
    }

    /// Resets all styling attributes to the terminal defaults.
    pub fn reset_style(&mut self) -> io::Result<()> {
        write!(self.writer, "{}0m", ESC)
    }

    // ==========================
    // FG/BG COLOR TRANSLATION
    // ==========================

    /// Writes the foreground color ANSI sequence.
    pub fn write_fg(&mut self, color: Color) -> io::Result<()> {
        match color {
            Color::Reset => write!(self.writer, "{}39m", ESC),
            Color::Black => write!(self.writer, "{}30m", ESC),
            Color::Red => write!(self.writer, "{}31m", ESC),
            Color::Green => write!(self.writer, "{}32m", ESC),
            Color::Yellow => write!(self.writer, "{}33m", ESC),
            Color::Blue => write!(self.writer, "{}34m", ESC),
            Color::Magenta => write!(self.writer, "{}35m", ESC),
            Color::Cyan => write!(self.writer, "{}36m", ESC),
            Color::White => write!(self.writer, "{}37m", ESC),
            Color::ANSI(idx) => write!(self.writer, "{}38;5;{}m", ESC, idx),
            Color::RGB(rgb) => write!(self.writer, "{}38;2;{};{};{}m", ESC, rgb.r, rgb.g, rgb.b),
        }
    }

    /// Writes the background color ANSI sequence.
    pub fn write_bg(&mut self, color: Color) -> io::Result<()> {
        match color {
            Color::Reset => write!(self.writer, "{}49m", ESC),
            Color::Black => write!(self.writer, "{}40m", ESC),
            Color::Red => write!(self.writer, "{}41m", ESC),
            Color::Green => write!(self.writer, "{}42m", ESC),
            Color::Yellow => write!(self.writer, "{}43m", ESC),
            Color::Blue => write!(self.writer, "{}44m", ESC),
            Color::Magenta => write!(self.writer, "{}45m", ESC),
            Color::Cyan => write!(self.writer, "{}46m", ESC),
            Color::White => write!(self.writer, "{}47m", ESC),
            Color::ANSI(idx) => write!(self.writer, "{}48;5;{}m", ESC, idx),
            Color::RGB(rgb) => write!(self.writer, "{}48;2;{};{};{}m", ESC, rgb.r, rgb.g, rgb.b),
        }
    }

    // ==========================
    // MODIFIERS TRANSLATION
    // ==========================

    /// Writes the ANSI sequences for the active text modifiers.
    pub fn write_modifiers(&mut self, modifier: Modifier) -> io::Result<()> {
        if modifier.contains(Modifier::BOLD) {
            write!(self.writer, "{}1m", ESC)?;
        }
        if modifier.contains(Modifier::DIM) {
            write!(self.writer, "{}2m", ESC)?;
        }
        if modifier.contains(Modifier::ITALIC) {
            write!(self.writer, "{}3m", ESC)?;
        }
        if modifier.contains(Modifier::UNDERLINED) {
            write!(self.writer, "{}4m", ESC)?;
        }
        if modifier.contains(Modifier::SLOW_BLINK) {
            write!(self.writer, "{}5m", ESC)?;
        }
        if modifier.contains(Modifier::RAPID_BLINK) {
            write!(self.writer, "{}6m", ESC)?;
        }
        if modifier.contains(Modifier::REVERSED) {
            write!(self.writer, "{}7m", ESC)?;
        }
        if modifier.contains(Modifier::HIDDEN) {
            write!(self.writer, "{}8m", ESC)?;
        }
        if modifier.contains(Modifier::CROSSED_OUT) {
            write!(self.writer, "{}9m", ESC)?;
        }

        Ok(())
    }
}
