use std::io::Result;
use std::io::Write;

use crossterm::cursor;
use crossterm::cursor::SetCursorStyle;
use crossterm::style::Color;
use crossterm::style::Print;
use crossterm::style::SetAttribute;
use crossterm::style::SetBackgroundColor;
use crossterm::style::SetForegroundColor;
use crossterm::terminal;
use crossterm::QueueableCommand;

use crate::core::styled_buffer::StyledBuffer;

pub struct Render {
    stdout: std::io::BufWriter<std::io::Stderr>,
    start_position: (u16, u16),
    terminal_size: (u16, u16),
}

/// Create default instance of Painter
impl Default for Render {
    fn default() -> Self {
        Self {
            stdout: std::io::BufWriter::new(std::io::stderr()),
            start_position: (0, 0),
            terminal_size: terminal::size().unwrap_or((0, 0)),
        }
    }
}

impl Render {
    /// Render the current line styled buffer
    pub fn render_line_buffer(&mut self, buffer: &StyledBuffer) -> Result<()> {
        let buffer_position = buffer.position() as u16;

        // Move to the start position, exacily after the promot
        self.stdout
            .queue(cursor::MoveToRow(self.start_position.1))?;
        self.stdout
            .queue(cursor::MoveToColumn(self.start_position.0))?;

        // Clear line
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?;

        self.render_styled_buffer(buffer)?;

        // Move the cursor to the current insertion position
        self.update_cursor_position(buffer_position)?;
        self.flush()?;
        Ok(())
    }

    /// Receving the insertion position on buffer and update the position on ui
    /// by calculating the right position using the promot length
    pub fn update_cursor_position(&mut self, position: u16) -> Result<()> {
        let mut move_to_position = self.start_position.0 + position;
        while self.terminal_size.0 > 0 && move_to_position > self.terminal_size.0 {
            move_to_position -= self.terminal_size.0;
        }
        self.stdout.queue(cursor::MoveToColumn(move_to_position))?;
        Ok(())
    }

    /// Render the prompt styled buffer
    pub fn render_prompt_buffer(&mut self, prompt: &StyledBuffer) -> Result<()> {
        self.render_styled_buffer(prompt)?;
        self.flush()?;
        Ok(())
    }

    /// Render hint at the end of buffer
    pub fn render_hint(&mut self, hint: &StyledBuffer) -> Result<()> {
        self.render_styled_buffer(hint)?;

        // Move the cursor to the current insertion position
        let (column, _) = cursor::position()?;
        self.stdout.queue(cursor::MoveToColumn(column))?;

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

    /// Update the stdout cursor style
    pub fn set_cursor_style(&mut self, style: SetCursorStyle) -> Result<()> {
        self.stdout.queue(style)?;
        Ok(())
    }

    /// Set the current line start position, after promot
    pub fn set_start_position(&mut self, position: (u16, u16)) {
        self.start_position = position;
    }

    /// Flush the current output stream,
    pub fn flush(&mut self) -> Result<()> {
        self.stdout.flush()?;
        Ok(())
    }
}
