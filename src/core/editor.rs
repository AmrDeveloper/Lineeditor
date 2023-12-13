use super::event::EditCommand;
use super::styled_buffer::StyledBuffer;

/// Wrapper for the Buffer to make it easy to run edit commands
pub struct Editor {
    buffer: StyledBuffer,
}

/// Create a new instance of [`Editor`]
impl Default for Editor {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
        }
    }
}

impl Editor {
    /// Get the current [`StyledBuffer`]
    pub fn styled_buffer(&mut self) -> &mut StyledBuffer {
        &mut self.buffer
    }

    /// Apply [`EditCommand`] to the current buffer
    pub fn run_edit_commands(&mut self, command: &EditCommand) {
        match command {
            EditCommand::MoveToStart => self.buffer.move_to_start(),
            EditCommand::MoveToEnd => self.buffer.move_to_end(),
            EditCommand::MoveLeftChar => self.buffer.move_left_char(),
            EditCommand::MoveRightChar => self.buffer.move_right_char(),
            EditCommand::MoveToPosition(position) => self.buffer.set_position(*position),
            EditCommand::InsertChar(c) => self.buffer.insert_char(*c),
            EditCommand::InsertString(s) => self.buffer.insert_string(s),
            EditCommand::Backspace => self.buffer.delete_left_char(),
            EditCommand::Delete => self.buffer.delete_right_char(),
            EditCommand::Clear => self.buffer.clear(),
        }
    }
}
