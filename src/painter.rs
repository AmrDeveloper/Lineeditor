use std::io::Result;
use std::io::Write;

use crossterm::cursor;
use crossterm::style::Color;
use crossterm::style::Print;
use crossterm::style::SetAttribute;
use crossterm::style::SetBackgroundColor;
use crossterm::style::SetForegroundColor;
use crossterm::terminal;
use crossterm::QueueableCommand;

use crate::core::styled_buffer::StyledBuffer;

pub struct Painter {
    stdout: std::io::BufWriter<std::io::Stderr>,
    buffer_column_start: u16,
}

/// Create default instance of Painter
impl Default for Painter {
    fn default() -> Self {
        Self {
            stdout: std::io::BufWriter::new(std::io::stderr()),
            buffer_column_start: 0,
        }
    }
}

impl Painter {
    pub fn repaint_buffer(&mut self, buffer: &StyledBuffer) -> Result<()> {
        let buffer_position = buffer.position() as u16;

        // Reset cursor position
        self.stdout
            .queue(cursor::MoveToColumn(self.buffer_column_start))?;

        // Clear line
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?;

        self.render_styled_buffer(buffer)?;

        // Move the cursor to the current insertion position
        self.stdout.queue(cursor::MoveToColumn(
            self.buffer_column_start + buffer_position,
        ))?;

        // Flush the output stream
        self.stdout.flush()?;
        Ok(())
    }

    fn render_styled_buffer(&mut self, buffer: &StyledBuffer) -> Result<()> {
        let styles = buffer.styles();
        let buffer_len = buffer.len();

        for (i, style) in styles.iter().enumerate().take(buffer_len) {
            // Set forground Color if exists
            if let Some(color) = style.forground_color() {
                self.stdout.queue(SetForegroundColor(*color))?;
            }

            // Set background Color if exists
            if let Some(color) = style.background_color() {
                self.stdout.queue(SetBackgroundColor(*color))?;
            }

            // Set Attributes
            for attribute in style.attributes() {
                self.stdout.queue(SetAttribute(*attribute))?;
            }

            // Reset Colors
            self.stdout.queue(Print(buffer.char_at(i).unwrap()))?;
            self.stdout.queue(SetForegroundColor(Color::Reset))?;
            self.stdout.queue(SetBackgroundColor(Color::Reset))?;
        }

        Ok(())
    }

    pub fn set_buffer_column_start(&mut self, start: u16) {
        self.buffer_column_start = start;
    }
}
