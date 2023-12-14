use std::collections::HashMap;

use crate::styled_buffer::StyledBuffer;

/// List of common pairs
pub const DEFAULT_PAIRS: &[(char, char)] = &[
    ('(', ')'),
    ('{', '}'),
    ('[', ']'),
    ('\'', '\''),
    ('"', '"'),
    ('`', '`'),
];

/// The Auto pair trait. Implementers of this trait will take the current styled buffer and then
/// modify it, which represents the contents of the original line
pub trait AutoPair {
    /// The action that will handle the current styled buffer as a line
    fn complete_pair(&self, buffer: &mut StyledBuffer);
}

/// Auto pair complete that depend on a map of pairs
pub struct DefaultAutoPair {
    pairs: HashMap<char, char>,
}

/// Create instance of DefaultAutoPair with default pairs [`DEFAULT_PAIRS`]
impl Default for DefaultAutoPair {
    fn default() -> Self {
        let mut pairs = HashMap::with_capacity(DEFAULT_PAIRS.len());
        for pair in DEFAULT_PAIRS {
            pairs.insert(pair.0, pair.1);
        }
        Self { pairs }
    }
}

impl DefaultAutoPair {
    /// Create instance of DefaultAutoPair with custom pairs
    pub fn with_pairs(pairs: HashMap<char, char>) -> Self {
        Self { pairs }
    }
}

impl AutoPair for DefaultAutoPair {
    /// Complete the pair if it exists on the pairs map and cursor is at the end
    fn complete_pair(&self, buffer: &mut StyledBuffer) {
        if buffer.position() == buffer.len() {
            if let Some(last_char) = buffer.buffer().last() {
                if let Some(pair) = self.pairs.get(last_char) {
                    buffer.insert_char(*pair);
                    buffer.move_char_left();
                }
            }
        }
    }
}
