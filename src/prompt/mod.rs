use crate::styled_buffer::StyledBuffer;

/// The Prompt trait, Implementers of this trait will return a prompt as styled buffer
pub trait Prompt {
    /// The action that will return prompt with styles as StyledBuffer
    fn prompt(&self) -> &StyledBuffer;
}

pub struct StringPrompt {
    text: StyledBuffer,
}

impl StringPrompt {
    #[must_use]
    pub fn new(text: String) -> Self {
        let mut buffer = StyledBuffer::default();
        buffer.insert_string(&text);

        StringPrompt { text: buffer }
    }
}

impl Prompt for StringPrompt {
    fn prompt(&self) -> &StyledBuffer {
        &self.text
    }
}
