use crate::styled_buffer::StyledBuffer;

/// The Hinter trait, Implementers of this trait will take the current styled buffer and then
/// Return a new StyledBuffer for the hint if exists or None if not hint
pub trait Hinter {
    /// The action that will handle the current styled buffer as a line
    fn hint(&self, buffer: &mut StyledBuffer) -> Option<StyledBuffer>;
}
