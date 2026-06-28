use std::{io, thread, time::Duration};

use biscuit::{
    Color, Constraint, Stylize, Terminal,
    widgets::{Block, HStack, Paragraph, VStack, Widget},
};

fn main() -> io::Result<()> {
    let mut terminal = Terminal::new(80, 24)?;

    for i in 0..300 {
        terminal.draw(|frame| {
            let cpu_val = (i % 20) as usize;
            let cpu_bar = format!(
                "CPU: [{}{}] {}%",
                "█".repeat(cpu_val),
                "░".repeat(20_usize.saturating_sub(cpu_val)),
                cpu_val * 5
            );

            let ram_val = 12 + (f32::sin(i as f32 * 0.2) * 3.0) as usize;
            let ram_bar = format!(
                "RAM: [{}{}] {}GB",
                "▓".repeat(ram_val),
                " ".repeat(20_usize.saturating_sub(ram_val)),
                ram_val
            );

            let (status_text, status_color) = if i % 10 < 5 {
                ("● LIVE", Color::Red)
            } else {
                ("○ LIVE", Color::Black) 
            };

            VStack::new()
                .child((
                    Constraint::Length(3),
                    HStack::new()
                        .child(
                            Paragraph::new(" 🍪 Biscuit OS Monitor v1.0")
                                .style(Color::Yellow.bold())
                        )
                        .child(
                            Paragraph::new(status_text)
                                .style(status_color.bold())
                        )
                ))
                .child((
                    Constraint::Length(5),
                    HStack::new()
                        .child(
                            Paragraph::new(&cpu_bar)
                                .block(Block::bordered().style(Color::Cyan))
                                .style(Color::Cyan)
                        )
                        .child(
                            Paragraph::new(&ram_bar)
                                .block(Block::bordered().style(Color::Magenta))
                                .style(Color::Magenta)
                        )
                ))
                .child(
                    Paragraph::new(&format!(
                        "System logs (Frame: {})\n> Loading modules...\n> Connecting to database...\n> Fetching telemetry...",
                        i
                    ))
                    .block(Block::bordered().style(Color::White))
                )
                .render(frame.size(), frame.buffer);
        })?;

        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
