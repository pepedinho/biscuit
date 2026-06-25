//! The main Terminal interface.
//!
//! This module coordinates the double-buffering and the ANSI backend.
//! Users interact with this module to run the rendering loop.

use std::io::{self, Stdout};

use crate::{Buffer, Style, backend::AnsiBackend, layout::Rect};

/// A Frame provides access to the temporary buffer being drawn.
/// The lifetime `'a` guarantees that the Frame cannot outlive
/// the Buffer reference it borrows from the Terminal.
pub struct Frame<'a> {
    pub buffer: &'a mut Buffer,
}

impl<'a> Frame<'a> {
    /// Creates a new Frame wrapping a mutable reference to a [`Buffer`].
    pub fn new(buffer: &'a mut Buffer) -> Self {
        Self { buffer }
    }

    /// Return the full usable area of the current frame.
    pub fn size(&self) -> Rect {
        Rect::new(0, 0, self.buffer.width, self.buffer.height)
    }
}

pub struct Terminal {
    /// What is currently visible on the screen.
    current_buffer: Buffer,
    /// What the user is drawing for the next frame.
    next_buffer: Buffer,
    /// Internal terminal backend
    backend: AnsiBackend<Stdout>,
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = self.backend.leave_alternate_screen();
        let _ = self.backend.show_cursor();
        let _ = self.backend.flush();
    }
}

impl Terminal {
    pub fn new(width: u16, height: u16) -> io::Result<Self> {
        let mut term = Terminal {
            current_buffer: Buffer::new(width, height),
            next_buffer: Buffer::new(width, height),
            backend: AnsiBackend::new(io::stdout()),
        };

        term.backend.enter_alternate_screen()?;
        term.backend.hide_cursor()?;
        term.backend.clear_screen()?;

        Ok(term)
    }

    pub fn draw<F>(&mut self, render_callback: F) -> io::Result<()>
    where
        F: FnOnce(&mut Frame),
    {
        self.next_buffer.reset();
        let mut frame = Frame::new(&mut self.next_buffer);
        render_callback(&mut frame);
        self.flush()
    }

    /// The diffing engine: compares next_buffer with current_buffer an prints ANSI codes.
    pub fn flush(&mut self) -> io::Result<()> {
        let mut cursor_x: Option<u16> = None;
        let mut cursor_y: Option<u16> = None;
        let mut current_style: Option<Style> = None;

        for y in 0..self.next_buffer.height {
            for x in 0..self.next_buffer.width {
                let Some(next_cell) = self.next_buffer.get(x, y) else {
                    continue;
                };
                let Some(curr_cell) = self.current_buffer.get(x, y) else {
                    continue;
                };

                let symbol_changed = next_cell.get_symbol() != curr_cell.get_symbol();
                let style_changed = next_cell.get_style() != curr_cell.get_style();

                if symbol_changed || style_changed {
                    if cursor_x.is_none()
                        || cursor_y.is_none()
                        || cursor_x.unwrap() != x
                        || cursor_y.unwrap() != y
                    {
                        self.backend.move_cursor(x, y)?;
                    }

                    if current_style.is_none() || &current_style.unwrap() != next_cell.get_style() {
                        self.backend.reset_style()?;
                        if let Some(fg) = next_cell.get_style().fg {
                            self.backend.write_fg(fg)?;
                        }
                        if let Some(bg) = next_cell.get_style().bg {
                            self.backend.write_bg(bg)?;
                        }

                        self.backend
                            .write_modifiers(next_cell.get_style().add_modifiers)?;
                        current_style = Some(*next_cell.get_style());
                    }

                    self.backend.write_char(next_cell.get_symbol())?;
                    cursor_x = Some(x + 1);
                    cursor_y = Some(y);
                }
            }
        }

        self.backend.flush()?;
        std::mem::swap(&mut self.current_buffer, &mut self.next_buffer);
        Ok(())
    }
}
