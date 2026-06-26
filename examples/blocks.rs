use std::{io, thread, time::Duration};

use biscuit::{
    Color, Constraint, Direction, Frame, Layout, Style, Terminal,
    widgets::{Block, Borders, Widget},
};

struct App {
    counter: usize,
}

fn render(app: &App, frame: &mut Frame) {
    let screen = frame.size();

    let vertical_chunks = Layout::new()
        .direction(Direction::Vertical)
        .constraints(&[Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(screen);

    let horizontal_chunks = Layout::new()
        .direction(Direction::Horizontal)
        .constraints(&[Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(vertical_chunks[1]);

    let header_area = vertical_chunks[0];

    let left_area = horizontal_chunks[0];
    let right_area = horizontal_chunks[1];

    let header_block = Block::new()
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::Yellow));
    let header_inner = header_block.inner(header_area);
    header_block.render(header_area, frame.buffer);

    let left_block = Block::new()
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::Cyan));
    let left_inner = left_block.inner(left_area);
    left_block.render(left_area, frame.buffer);

    // Right Block
    let right_block = Block::new()
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::Green));
    let right_inner = right_block.inner(right_area);
    right_block.render(right_area, frame.buffer);

    frame.buffer.set_string(
        header_inner.x + 2,
        header_inner.y,
        " Biscuit Dashboard",
        Style::new().bold(),
    );

    let text = format!("🚀 Frame: {}", app.counter);
    frame.buffer.set_string(
        left_inner.x,
        left_inner.y,
        &text,
        Style::new().fg(Color::Cyan).bold(),
    );

    frame.buffer.set_string(
        right_inner.x,
        right_inner.y,
        "Patientez 5 secondes...",
        Style::new().dim(),
    );
}

fn main() -> io::Result<()> {
    let mut terminal = Terminal::new(80, 24)?;

    for i in 0..50 {
        terminal.draw(|frame| {
            let screen = frame.size();

            let vertical_chunks = Layout::new()
                .direction(Direction::Vertical)
                .constraints(&[Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(screen);

            let horizontal_chunks = Layout::new()
                .direction(Direction::Horizontal)
                .constraints(&[Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(vertical_chunks[1]);

            let header_area = vertical_chunks[0];

            let left_area = horizontal_chunks[0];
            let right_area = horizontal_chunks[1];

            let header_block = Block::new()
                .borders(Borders::ALL)
                .style(Style::new().fg(Color::Yellow));
            let header_inner = header_block.inner(header_area);
            header_block.render(header_area, frame.buffer);

            let left_block = Block::new()
                .borders(Borders::ALL)
                .style(Style::new().fg(Color::Cyan));
            let left_inner = left_block.inner(left_area);
            left_block.render(left_area, frame.buffer);

            // Right Block
            let right_block = Block::new()
                .borders(Borders::ALL)
                .style(Style::new().fg(Color::Green));
            let right_inner = right_block.inner(right_area);
            right_block.render(right_area, frame.buffer);

            frame.buffer.set_string(
                header_inner.x + 2,
                header_inner.y,
                " Biscuit Dashboard",
                Style::new().bold(),
            );

            let text = format!("🚀 Frame: {}", i);
            frame.buffer.set_string(
                left_inner.x,
                left_inner.y,
                &text,
                Style::new().fg(Color::Cyan).bold(),
            );

            frame.buffer.set_string(
                right_inner.x,
                right_inner.y,
                "Patientez 5 secondes...",
                Style::new().dim(),
            );
        })?;

        thread::sleep(Duration::from_millis(100));
    }

    todo!();
}
