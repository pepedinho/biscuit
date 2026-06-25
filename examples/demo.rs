use std::{thread, time::Duration};

use biscuit::{Color, Style, Terminal};

fn main() -> Result<(), std::io::Error> {
    let mut terminal = Terminal::new(80, 24)?;

    for i in 0..50 {
        terminal.draw(|frame| {
            let msg = format!("🚀 Biscuit Rust Engine - Frame: {}", i);
            let style = Style::new().fg(Color::Cyan).bold();

            frame.buffer.set_string(10, 10, &msg, style);

            let help = "Wait 5 seconds...";
            let help_style = Style::new().fg(Color::Yellow).dim();
            frame.buffer.set_string(10, 12, help, help_style);
        })?;

        thread::sleep(Duration::from_millis(100));
    }

    todo!()
}
