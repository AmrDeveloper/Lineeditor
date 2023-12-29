use crate::styled_buffer::StyledBuffer;

/// A span of source code, with positions
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
}

/// Suggestion returned by the Completer
pub struct Suggestion {
    /// Suggestion content and styles
    pub content: StyledBuffer,
    /// Replacement span
    pub span: Span,
}

/// The Completer trait, Implementers of this trait will return a list of suggestions as styled buffers
pub trait Completer {
    /// The action that will return a list of suggestions
    fn complete(&self, input: &StyledBuffer) -> Vec<Suggestion>;
}
