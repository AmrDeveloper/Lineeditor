/// Editing actions which can be mapped to key bindings.
///
/// Executed by `Editor::run_edit_commands()`
#[derive(Clone)]
pub enum EditCommand {
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

    /// Insert a character at the current insertion point
    InsertChar(char),

    /// Insert a string at the current insertion point
    InsertString(String),

    /// Backspace delete from the current insertion point
    Backspace,

    /// Delete in-place from the current insertion point
    Delete,

    /// Delete in-place the current selected range
    DeleteSelected(usize, usize),

    /// Clear the current buffer
    Clear,
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

    /// Delete char from the left or delete selected range
    Backspace,

    /// Delete char from the right or delete selected range
    Delete,
}
