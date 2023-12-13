use super::style::Style;

/// Memory representation of the lines and styles
pub struct StyledBuffer {
    /// The buffer as list of characters
    buffer: Vec<char>,
    /// The styles for each character on the buffer
    styles: Vec<Style>,
    /// The current insertion point in the buffer
    cursor_position: usize,
}

/// Create default instance of StyledBuffer
impl Default for StyledBuffer {
    fn default() -> Self {
        Self {
            buffer: vec![],
            styles: vec![],
            cursor_position: 0,
        }
    }
}

impl StyledBuffer {
    /// Insert character at the current position with default style
    pub fn insert_char(&mut self, ch: char) {
        self.buffer.insert(self.cursor_position, ch);
        self.styles.insert(self.cursor_position, Style::default());
        self.move_right_char();
    }

    /// Insert character at the current position with style
    pub fn insert_styled_char(&mut self, ch: char, style: Style) {
        self.buffer.insert(self.cursor_position, ch);
        self.styles.insert(self.cursor_position, style);
        self.move_right_char();
    }

    /// Insert string at the current position with defualt style
    pub fn insert_string(&mut self, str: &str) {
        for ch in str.chars() {
            self.insert_char(ch);
        }
    }

    /// Insert string at the current position with style
    pub fn insert_styled_string(&mut self, str: &str, style: Style) {
        for ch in str.chars() {
            self.insert_styled_char(ch, style.clone());
        }
    }

    /// Safe Move the cursor position to the right
    pub fn move_right_char(&mut self) {
        if self.cursor_position < self.len() {
            self.cursor_position += 1;
        }
    }

    /// Safe Move the cursor position to the left
    pub fn move_left_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    /// Move cursor to the start of the buffer
    pub fn move_to_start(&mut self) {
        self.cursor_position = 0;
    }

    /// Move cursor to the end of the buffer
    pub fn move_to_end(&mut self) {
        self.cursor_position = self.buffer.len();
    }

    /// Deletes one character to the right
    pub fn delete_right_char(&mut self) {
        if self.cursor_position + 1 < self.buffer.len() {
            self.buffer.remove(self.cursor_position + 1);
            self.styles.remove(self.cursor_position + 1);
        }
    }

    /// Deletes one character to the left
    pub fn delete_left_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.buffer.remove(self.cursor_position);
            self.styles.remove(self.cursor_position);
        }
    }

    /// Get current Buffer
    pub fn buffer(&mut self) -> &Vec<char> {
        &self.buffer
    }
    
    /// Get char at position
    pub fn char_at(&self, position: usize) -> Option<char> {
        Some(self.buffer[position])
    }

    /// Get current Styles
    pub fn styles(&self) -> &Vec<Style> {
        &self.styles
    }

    /// Reset all styles to the default one
    pub fn reset_styles(&mut self) {
        for i in 0..self.styles.len() {
            self.styles[i] = Style::default();
        }
    }

    /// Empty buffer, styles and reset cursor position
    pub fn clear(&mut self) {
        self.buffer = vec![];
        self.styles = vec![];
        self.cursor_position = 0;
    }

    // Set cursor position
    pub fn set_position(&mut self, pos: usize) {
        self.cursor_position = pos;
    }

    /// Get cursor position
    pub fn position(&self) -> usize {
        self.cursor_position
    }

    /// Returns true if the cursor position is at the end of the buffer
    pub fn is_cursor_at_the_end(&self) -> bool {
        self.cursor_position == self.len()
    }

    /// Length of the buffer
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns `true` if the buffer contains no elements.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
