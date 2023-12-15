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
use crate::keybindings::KeyCombination;
use crate::keybindings::Keybindings;
use crate::style::Style;
use crate::AutoPair;
use crate::Highlighter;
use crate::Hinter;
use crate::Painter;
use crate::Prompt;

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
    prompt: Box<dyn Prompt>,
    editor: Editor,
    painter: Painter,
    keybindings: Keybindings,
    autopair: Option<Box<dyn AutoPair>>,
    highlighters: Vec<Box<dyn Highlighter>>,
    hinters: Vec<Box<dyn Hinter>>,

    selection_style: Option<Style>,
    selected_start: u16,
    selected_end: u16,
}

impl LineEditor {
    /// Create new instance of LineEditor with Prompt
    #[must_use]
    pub fn new(prompt: Box<dyn Prompt>) -> Self {
        LineEditor {
            prompt,
            editor: Editor::default(),
            painter: Painter::default(),
            keybindings: Keybindings::default(),
            autopair: None,
            highlighters: vec![],
            hinters: vec![],

            selection_style: None,
            selected_start: 0,
            selected_end: 0,
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

        let prompt_buffer = self.prompt.prompt();
        let promot_len = prompt_buffer.len() as u16;

        self.painter.set_buffer_column_start(promot_len);
        self.painter.render_promot_buffer(prompt_buffer)?;

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

                            let key_combination = KeyCombination::from(key_event);
                            if let Some(command) = self.keybindings.find_binding(key_combination) {
                                lineeditor_events.push(command);
                                break;
                            }
                        }
                        _ => {
                            let key_combination = KeyCombination::from(key_event);
                            if let Some(command) = self.keybindings.find_binding(key_combination) {
                                lineeditor_events.push(command);
                                break;
                            }
                        }
                    }
                }
            }

            // Track the buffer size at the start
            let buffer_len_bofore = self.editor.styled_buffer().len();

            // Apply the list of events
            for event in lineeditor_events.drain(..) {
                match self.handle_editor_event(&event)? {
                    EventStatus::Handled => {}
                    EventStatus::Inapplicable => {}
                    EventStatus::Exits(result) => return Ok(result),
                }
            }

            // Run the auto pair complete if one char is inserted
            if buffer_len_bofore < self.editor.styled_buffer().len() {
                // Auto pair complete
                if let Some(auto_pair) = &self.autopair {
                    auto_pair.complete_pair(self.editor.styled_buffer());
                }
            }

            // Reset styled buffer styles
            self.editor.styled_buffer().reset_styles();

            // Apply all registerd syntax highlighter in insertion order
            for highlighter in &self.highlighters {
                highlighter.highlight(self.editor.styled_buffer());
            }

            // Apply visual selection
            self.apply_visual_selection();

            // Render the current buffer with style
            self.painter
                .render_line_buffer(self.editor.styled_buffer())?;

            // If cursor is at the end of the buffer, check if hint is avaible
            if self.editor.styled_buffer().position() == self.editor.styled_buffer().len() {
                for hinter in &self.hinters {
                    if let Some(hint) = hinter.hint(self.editor.styled_buffer()) {
                        self.painter.render_hint(&hint)?;
                        break;
                    }
                }
            }
        }
    }

    /// Apply LineEditorEvent and return handling status
    fn handle_editor_event(&mut self, event: &LineEditorEvent) -> Result<EventStatus> {
        match event {
            LineEditorEvent::Edit(commands) => {
                for command in commands {
                    self.editor.run_edit_commands(command);
                }
                self.reset_selection_range();
                Ok(EventStatus::Handled)
            }
            LineEditorEvent::Enter => {
                let buffer = self.editor.styled_buffer().buffer().iter().collect();
                self.reset_selection_range();
                Ok(EventStatus::Exits(LineEditorResult::Success(buffer)))
            }
            LineEditorEvent::Left => {
                self.editor.run_edit_commands(&EditCommand::MoveLeftChar);
                self.reset_selection_range();
                Ok(EventStatus::Handled)
            }
            LineEditorEvent::Right => {
                self.editor.run_edit_commands(&EditCommand::MoveRightChar);
                self.reset_selection_range();
                Ok(EventStatus::Handled)
            }
            LineEditorEvent::Delete => {
                if self.selected_start != self.selected_end {
                    let from = usize::min(self.selected_start.into(), self.selected_end.into());
                    let to = usize::max(self.selected_start.into(), self.selected_end.into());
                    let delete_selection = EditCommand::DeleteSelected(from, to);
                    self.editor.run_edit_commands(&delete_selection);
                    self.editor.styled_buffer().set_position(from);
                    self.reset_selection_range();
                } else {
                    self.editor.run_edit_commands(&EditCommand::Delete)
                }
                Ok(EventStatus::Handled)
            }
            LineEditorEvent::Backspace => {
                if self.selected_start != self.selected_end {
                    let from = usize::min(self.selected_start.into(), self.selected_end.into());
                    let to = usize::max(self.selected_start.into(), self.selected_end.into());
                    let delete_selection = EditCommand::DeleteSelected(from, to);
                    self.editor.run_edit_commands(&delete_selection);
                    self.editor.styled_buffer().set_position(from);
                    self.reset_selection_range();
                } else {
                    self.editor.run_edit_commands(&EditCommand::Backspace)
                }
                Ok(EventStatus::Handled)
            }
            LineEditorEvent::SelectLeft => {
                if self.selected_end < 1 {
                    Ok(EventStatus::Inapplicable)
                } else {
                    self.selected_end -= 1;
                    Ok(EventStatus::Handled)
                }
            }
            LineEditorEvent::SelectRight => {
                if self.selected_end as usize > self.editor.styled_buffer().len() {
                    Ok(EventStatus::Inapplicable)
                } else {
                    self.selected_end += 1;
                    Ok(EventStatus::Handled)
                }
            }
            LineEditorEvent::SelectAll => {
                self.selected_start = 0;
                self.selected_end = self.editor.styled_buffer().len() as u16;
                Ok(EventStatus::Handled)
            }
            _ => Ok(EventStatus::Inapplicable),
        }
    }

    /// Apply visual selection on the current styled buffer
    fn apply_visual_selection(&mut self) {
        if self.selected_start == self.selected_end {
            return;
        }

        // Apply visual selection style if it not None
        if let Some(style) = &self.selection_style {
            let styled_buffer = self.editor.styled_buffer();
            // Handle From and To, so we allow select from any direction
            let from = usize::min(self.selected_start.into(), self.selected_end.into());
            let to = usize::max(self.selected_start.into(), self.selected_end.into());
            styled_buffer.style_range(from, to, style.clone());
        }
    }

    /// Reset selection start and end to be the current cursor position
    fn reset_selection_range(&mut self) {
        let position = self.editor.styled_buffer().position() as u16;
        self.selected_start = position;
        self.selected_end = position;
    }

    /// Set style for visual selection or NONE to clear it
    pub fn set_visual_selection_style(&mut self, style: Option<Style>) {
        self.selection_style = style;
    }

    /// Get the current Editor
    pub fn editor(&mut self) -> &mut Editor {
        &mut self.editor
    }

    /// Get the current Keybindings
    pub fn keybinding(&mut self) -> &mut Keybindings {
        &mut self.keybindings
    }

    /// Add Auto pair, or clear it by passing None
    pub fn set_autopair(&mut self, autopair: Option<Box<dyn AutoPair>>) {
        self.autopair = autopair
    }

    /// Get the current list of highlighters
    pub fn highlighters(&mut self) -> &mut Vec<Box<dyn Highlighter>> {
        &mut self.highlighters
    }

    /// Add new Syntax highlighter
    pub fn add_highlighter(&mut self, highlighter: Box<dyn Highlighter>) {
        self.highlighters.push(highlighter);
    }

    /// Clear current syntax highlighter
    pub fn clear_highlighters(&mut self) {
        self.highlighters.clear();
    }

    /// Get current hinters
    pub fn hinters(&mut self) -> &mut Vec<Box<dyn Hinter>> {
        &mut self.hinters
    }

    /// Add new Hinter
    pub fn add_hinter(&mut self, hinter: Box<dyn Hinter>) {
        self.hinters.push(hinter);
    }

    /// Clear current hinters
    pub fn clear_hinters(&mut self) {
        self.hinters.clear();
    }
}
