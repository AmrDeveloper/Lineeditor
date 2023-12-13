use std::io::Result;

use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyModifiers;
use crossterm::terminal;

use crate::editor::Editor;
use crate::event::EditCommand;
use crate::event::LineEditorEvent;
use crate::Painter;

/// A Result can return from`LineEditor::read_line()`
#[derive(Debug)]
pub enum LineEditorResult {
    /// Entry succeeded with the provided content
    Success(String),

    /// Interrupt current editing
    Interrupted,

    /// End terminal session
    EndTerminalSession,
}

/// An internal Status returnd after applying event
enum EventStatus {
    /// Event is handled
    Handled,
    /// Event is in applicable to handle
    Inapplicable,
    /// Exit with Result or Error
    Exits(LineEditorResult),
}

/// Line Editor Engine
pub struct LineEditor {
    editor: Editor,
    painter: Painter,
}

impl LineEditor {
    /// Create new instance of LineEditor with Prompt
    #[must_use]
    pub fn new() -> Self {
        LineEditor {
            editor: Editor::default(),
            painter: Painter::default(),
        }
    }

    /// Wait for input and provide the user
    ///
    /// Returns a [`std::io::Result`] in which the `Err` type is [`std::io::Result`]
    /// and the `Ok` variant wraps a [`LineEditorResult`] which handles user inputs.
    pub fn read_line(&mut self) -> Result<LineEditorResult> {
        terminal::enable_raw_mode()?;
        let result = self.read_line_helper();
        terminal::disable_raw_mode()?;
        result
    }

    /// Helper implementing the logic for [`LineEditor::read_line()`] to be wrapped
    /// in a `raw_mode` context.
    fn read_line_helper(&mut self) -> Result<LineEditorResult> {
        let mut lineeditor_events: Vec<LineEditorEvent> = vec![];

        loop {
            loop {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char(ch) => {
                            if (key_event.modifiers == KeyModifiers::NONE
                                || key_event.modifiers == KeyModifiers::SHIFT)
                                && key_event.kind == KeyEventKind::Press
                            {
                                let edit_command =
                                    LineEditorEvent::Edit(vec![EditCommand::InsertChar(ch)]);
                                lineeditor_events.push(edit_command);
                                break;
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }

            // Apply the list of events
            for event in lineeditor_events.drain(..) {
                match self.handle_editor_event(&event)? {
                    EventStatus::Handled => {}
                    EventStatus::Inapplicable => {}
                    EventStatus::Exits(result) => return Ok(result),
                }
            }

            // Render the current buffer with style
            self.painter.repaint_buffer(&self.editor.styled_buffer())?;
        }
    }

    /// Apply LineEditorEvent and return handling status
    fn handle_editor_event(&mut self, event: &LineEditorEvent) -> Result<EventStatus> {
        match event {
            LineEditorEvent::Edit(commands) => {
                for command in commands {
                    self.editor.run_edit_commands(command);
                }
                Ok(EventStatus::Handled)
            }
            LineEditorEvent::Left => {
                self.editor.run_edit_commands(&EditCommand::MoveLeftChar);
                Ok(EventStatus::Handled)
            }
            LineEditorEvent::Right => {
                self.editor.run_edit_commands(&EditCommand::MoveRightChar);
                Ok(EventStatus::Handled)
            }
            LineEditorEvent::Enter => {
                let buffer = self.editor.styled_buffer().buffer().iter().collect();
                Ok(EventStatus::Exits(LineEditorResult::Success(buffer)))
            }
            _ => Ok(EventStatus::Inapplicable),
        }
    }

    /// Get the current Editor
    pub fn editor(&mut self) -> &mut Editor {
        &mut self.editor
    }
}
