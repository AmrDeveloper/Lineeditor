/// Editing actions which can be mapped to key bindings.
///
/// Executed by `Editor::run_edit_commands()`
#[derive(Clone)]
pub enum EditCommand {
    /// Insert a character at the current insertion point
    InsertChar(char),

    /// Insert a string at the current insertion point
    InsertString(String),

    /// Backspace delete from the current insertion point
    DeleteLeftChar,

    /// Delete in-place from the current insertion point
    DeleteRightChar,

    /// Delete in-place the current selected range
    DeleteSelected(usize, usize),

    /// Clear the current buffer
    Clear,
}

/// Movements actions which can be mapped to key bindings.
#[derive(Clone)]
pub enum MovementCommand {
    /// Move to the start of the buffer
    MoveToStart,

    /// Move to the end of the buffer
    MoveToEnd,

    /// Move one character to the left
    MoveLeftChar,

    /// Move one character to the right
    MoveRightChar,

    /// Move one word to the left
    MoveLeftWord,

    /// Move one word to the right
    MoveRightWord,

    /// Move to position
    MoveToPosition(usize),
}

/// LineEditor supported actions.
#[derive(Clone)]
pub enum LineEditorEvent {
    /// No op event
    None,

    /// Handle enter event
    Enter,

    /// Esc event
    Esc,

    /// Handle unconditional submit event
    Submit,

    /// Run these commands in the editor
    Edit(Vec<EditCommand>),

    /// Run movements commands in the editor
    Movement(Vec<MovementCommand>),

    /// Move up to the previous line, if multiline, or up into the historic buffers
    Up,

    /// Move down to the next line, if multiline, or down through the historic buffers
    Down,

    /// Move right to the next column, completion entry, or complete hint
    Right,

    /// Move left to the next column, or completion entry
    Left,

    /// Select one character to the right
    SelectRight,

    /// Select one character to the left
    SelectLeft,

    /// Select all buffer
    SelectAll,

    /// Cut the selected text into clipboard
    CutSelected,

    /// Copy the selected text into clipboard
    CopySelected,

    /// Paste text from clipboard into selection or at insertion point
    Paste,

    /// Delete char from the left or delete selected range
    Backspace,

    /// Delete char from the right or delete selected range
    Delete,
}
