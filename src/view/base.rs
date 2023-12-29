use std::io::BufWriter;
use std::io::Result;
use std::io::Stderr;

use crossterm::style::Color;
use crossterm::style::Print;
use crossterm::style::SetAttribute;
use crossterm::style::SetBackgroundColor;
use crossterm::style::SetForegroundColor;
use crossterm::QueueableCommand;

use crate::styled_buffer::StyledBuffer;

pub fn render_styled_buffer(stdout: &mut BufWriter<Stderr>, buffer: &StyledBuffer) -> Result<()> {
    let styles = buffer.styles();
    let buffer_len = buffer.len();

    for (i, style) in styles.iter().enumerate().take(buffer_len) {
        // Set foreground Color if exists
        if let Some(color) = style.foreground_color() {
            stdout.queue(SetForegroundColor(*color))?;
        }

        // Set background Color if exists
        if let Some(color) = style.background_color() {
            stdout.queue(SetBackgroundColor(*color))?;
        }

        // Set Attributes
        for attribute in style.attributes() {
            stdout.queue(SetAttribute(*attribute))?;
        }

        // Reset Colors
        stdout.queue(Print(buffer.char_at(i).unwrap()))?;
        stdout.queue(SetForegroundColor(Color::Reset))?;
        stdout.queue(SetBackgroundColor(Color::Reset))?;
    }

    Ok(())
}
