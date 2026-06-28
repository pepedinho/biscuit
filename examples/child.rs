use std::{io, thread, time::Duration};

use biscuit::{
    Color, Constraint, Style, Stylize, Terminal,
    widgets::{Block, HStack, Paragraph, VStack, Widget},
};

fn main() -> io::Result<()> {
    let mut terminal = Terminal::new(80, 24)?;

    for i in 0..500 {
        terminal.draw(|frame| {
            VStack::new()
                .child((
                    Constraint::Length(3),
                    Paragraph::new("Biscuit Dashboard")
                        .block(Block::bordered().style(Color::Yellow))
                        .style(Style::new().bold()),
                ))
                .child(
                    HStack::new()
                        .child(
                            Paragraph::new(&format!("🚀 Frame: {}", i))
                                .block(Block::bordered().style(Color::Cyan))
                                .style(Color::Cyan.bold()),
                        )
                        .child(
                            Paragraph::new("Wait 5 seconds...")
                                .block(Block::bordered().style(Color::Green))
                                .style(Style::new().dim()),
                        ),
                )
                .render(frame.size(), frame.buffer);
        })?;

        thread::sleep(Duration::from_millis(30));
    }
    todo!();
}
