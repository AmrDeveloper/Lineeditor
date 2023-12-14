use std::collections::HashMap;
use std::hash::Hash;

use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyModifiers;

use crate::event::EditCommand;

use super::event::LineEditorEvent;

/// Represent the key combination
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct KeyCombination {
    pub key_kind: KeyEventKind,
    pub modifier: KeyModifiers,
    pub key_code: KeyCode,
}

/// Create KeyCombination from crossterm KeyEvent
impl From<KeyEvent> for KeyCombination {
    fn from(key_event: KeyEvent) -> Self {
        KeyCombination {
            key_kind: key_event.kind,
            modifier: key_event.modifiers,
            key_code: key_event.code,
        }
    }
}

/// Map of keybindings and [`LineEditorEvent`]
pub struct Keybindings {
    /// Defines a keybinding for a reedline event
    pub bindings: HashMap<KeyCombination, LineEditorEvent>,
}

/// Create a new instance of [`Keybindings`]
impl Default for Keybindings {
    fn default() -> Self {
        Keybindings {
            bindings: HashMap::new(),
        }
    }
}

impl Keybindings {
    /// Register an [`LineEditorEvent`] for specific key combination
    pub fn register_binding(&mut self, key_combination: KeyCombination, event: LineEditorEvent) {
        self.bindings.insert(key_combination, event);
    }

    /// Find a keybinding based on the modifier and keycode
    pub fn find_binding(&self, key_combination: KeyCombination) -> Option<LineEditorEvent> {
        self.bindings.get(&key_combination).cloned()
    }

    /// Get assigned keybindings
    pub fn keybindings(&self) -> &HashMap<KeyCombination, LineEditorEvent> {
        &self.bindings
    }

    /// Register basic functionality to Control
    ///
    /// `Enter`, `Esc`
    pub fn register_common_control_bindings(&mut self) {
        self.register_binding(
            KeyCombination {
                key_kind: KeyEventKind::Press,
                modifier: KeyModifiers::NONE,
                key_code: KeyCode::Enter,
            },
            LineEditorEvent::Enter,
        );

        self.register_binding(
            KeyCombination {
                key_kind: KeyEventKind::Press,
                modifier: KeyModifiers::NONE,
                key_code: KeyCode::Esc,
            },
            LineEditorEvent::Esc,
        );
    }

    /// Register basic functionality to Navigation
    ///
    /// `Up`, `Down`, `Right`, `Left` Arrow keys
    /// `CTRL + Right`, `CTRL + Left`
    pub fn register_common_navigation_bindings(&mut self) {
        self.register_binding(
            KeyCombination {
                key_kind: KeyEventKind::Press,
                modifier: KeyModifiers::NONE,
                key_code: KeyCode::Up,
            },
            LineEditorEvent::Up,
        );

        self.register_binding(
            KeyCombination {
                key_kind: KeyEventKind::Press,
                modifier: KeyModifiers::NONE,
                key_code: KeyCode::Down,
            },
            LineEditorEvent::Down,
        );

        self.register_binding(
            KeyCombination {
                key_kind: KeyEventKind::Press,
                modifier: KeyModifiers::NONE,
                key_code: KeyCode::Left,
            },
            LineEditorEvent::Left,
        );

        self.register_binding(
            KeyCombination {
                key_kind: KeyEventKind::Press,
                modifier: KeyModifiers::NONE,
                key_code: KeyCode::Right,
            },
            LineEditorEvent::Right,
        );

        self.register_binding(
            KeyCombination {
                key_kind: KeyEventKind::Press,
                modifier: KeyModifiers::CONTROL,
                key_code: KeyCode::Left,
            },
            LineEditorEvent::Edit(vec![EditCommand::MoveLeftWord]),
        );

        self.register_binding(
            KeyCombination {
                key_kind: KeyEventKind::Press,
                modifier: KeyModifiers::CONTROL,
                key_code: KeyCode::Right,
            },
            LineEditorEvent::Edit(vec![EditCommand::MoveRightWord]),
        );
    }

    /// Register basic functionality to edit
    ///
    /// `Delete`, `Backspace` and the basic variants do delete words
    pub fn register_common_edit_bindings(&mut self) {
        self.register_binding(
            KeyCombination {
                key_kind: KeyEventKind::Press,
                modifier: KeyModifiers::NONE,
                key_code: KeyCode::Backspace,
            },
            LineEditorEvent::Backspace,
        );

        self.register_binding(
            KeyCombination {
                key_kind: KeyEventKind::Press,
                modifier: KeyModifiers::NONE,
                key_code: KeyCode::Delete,
            },
            LineEditorEvent::Delete,
        );
    }

    /// Register basic functionality to selection
    ///
    /// Select right and left and select all
    pub fn register_common_selection_bindings(&mut self) {
        self.register_binding(
            KeyCombination {
                key_kind: KeyEventKind::Press,
                modifier: KeyModifiers::SHIFT,
                key_code: KeyCode::Left,
            },
            LineEditorEvent::SelectLeft,
        );

        self.register_binding(
            KeyCombination {
                key_kind: KeyEventKind::Press,
                modifier: KeyModifiers::SHIFT,
                key_code: KeyCode::Right,
            },
            LineEditorEvent::SelectRight,
        );

        self.register_binding(
            KeyCombination {
                key_kind: KeyEventKind::Press,
                modifier: KeyModifiers::CONTROL,
                key_code: KeyCode::Char('a'),
            },
            LineEditorEvent::SelectAll,
        );
    }
}
