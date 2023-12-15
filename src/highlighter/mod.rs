use crate::styled_buffer::StyledBuffer;

/// The syntax highlighting trait. Implementers of this trait will take the current styled buffer and then
/// modify it, which represents the contents of the original line
pub trait Highlighter {
    /// The action that will handle the current styled buffer as a line
    fn highlight(&self, buffer: &mut StyledBuffer);
}
