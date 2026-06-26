use std::{io, thread, time::Duration};

use biscuit::{
    Color, Constraint, Style, Terminal,
    widgets::{Block, Borders, HStack, Paragraph, VStack, Widget},
};

fn main() -> io::Result<()> {
    let mut terminal = Terminal::new(80, 24)?;

    for i in 0..50 {
        terminal.draw(|frame| {
            VStack::new()
                .child((
                    Constraint::Length(3),
                    Paragraph::new("Biscuit Dashboard")
                        .block(
                            Block::new()
                                .borders(Borders::ALL)
                                .style(Style::new().fg(Color::Yellow)),
                        )
                        .style(Style::new().bold()),
                ))
                .child(
                    HStack::new()
                        .child(
                            Paragraph::new(&format!("🚀 Frame: {}", i))
                                .block(
                                    Block::new()
                                        .borders(Borders::ALL)
                                        .style(Style::new().fg(Color::Cyan)),
                                )
                                .style(Style::new().fg(Color::Cyan).bold()),
                        )
                        .child(
                            Paragraph::new("Wait 5 seconds...")
                                .block(
                                    Block::new()
                                        .borders(Borders::ALL)
                                        .style(Style::new().fg(Color::Green)),
                                )
                                .style(Style::new().dim()),
                        ),
                )
                .render(frame.size(), frame.buffer);
        })?;

        thread::sleep(Duration::from_millis(100));
    }
    todo!();
}
