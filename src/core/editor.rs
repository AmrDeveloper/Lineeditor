use crate::event::MovementCommand;

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
            EditCommand::InsertChar(c) => self.buffer.insert_char(*c),
            EditCommand::InsertString(s) => self.buffer.insert_string(s),
            EditCommand::DeleteLeftChar => self.buffer.delete_left_char(),
            EditCommand::DeleteRightChar => self.buffer.delete_right_char(),
            EditCommand::DeleteSpan(from, to) => self.buffer.delete_range(*from, *to),
            EditCommand::Clear => self.buffer.clear(),
        }
    }

    /// Apply [`MovementCommand`] to the current buffer
    pub fn run_movement_commands(&mut self, command: &MovementCommand) {
        match command {
            MovementCommand::MoveToStart => self.buffer.move_to_start(),
            MovementCommand::MoveToEnd => self.buffer.move_to_end(),
            MovementCommand::MoveLeftChar => self.buffer.move_char_left(),
            MovementCommand::MoveRightChar => self.buffer.move_char_right(),
            MovementCommand::MoveLeftWord => self.buffer.move_word_left(),
            MovementCommand::MoveRightWord => self.buffer.move_word_right(),
            MovementCommand::MoveToPosition(position) => self.buffer.set_position(*position),
        }
    }
}
