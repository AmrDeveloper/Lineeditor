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

/// Create instance of StyledBuffer from String
impl From<&str> for StyledBuffer {
    fn from(value: &str) -> Self {
        let mut styled_buffer = StyledBuffer::default();
        styled_buffer.insert_string(value);
        styled_buffer
    }
}

impl StyledBuffer {
    /// Insert character at the current position with default style
    pub fn insert_char(&mut self, ch: char) {
        self.buffer.insert(self.cursor_position, ch);
        self.styles.insert(self.cursor_position, Style::default());
        self.move_char_right();
    }

    /// Insert character at the current position with style
    pub fn insert_styled_char(&mut self, ch: char, style: Style) {
        self.buffer.insert(self.cursor_position, ch);
        self.styles.insert(self.cursor_position, style);
        self.move_char_right();
    }

    /// Insert string at the current position with default style
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
    pub fn move_char_right(&mut self) {
        if self.cursor_position < self.len() {
            self.cursor_position += 1;
        }
    }

    /// Safe Move the cursor position to the left
    pub fn move_char_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    /// Move the cursor to the begin of the next right word
    pub fn move_word_right(&mut self) {
        while self.cursor_position < self.len() {
            if self.buffer[self.cursor_position].is_whitespace() {
                if self.cursor_position != self.len() {
                    self.cursor_position += 1;
                }
                return;
            }
            self.cursor_position += 1;
        }
    }

    /// Move the cursor to the begin of the next right word
    pub fn move_word_left(&mut self) {
        self.cursor_position = usize::min(self.cursor_position, self.len() - 1);
        if self.cursor_position != 0 {
            self.cursor_position -= 1;
        }

        while 0 != self.cursor_position {
            if self.buffer[self.cursor_position].is_whitespace() {
                self.cursor_position -= 1;
                continue;
            }
            break;
        }

        while 0 != self.cursor_position {
            if self.buffer[self.cursor_position].is_whitespace() {
                if self.cursor_position + 1 < self.len() {
                    self.cursor_position += 1;
                }
                return;
            }
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

    /// Deletes range of characters and styles from buffer
    pub fn delete_range(&mut self, from: usize, to: usize) {
        if to <= self.len() {
            self.buffer.drain(from..to);
            self.styles.drain(from..to);
            self.cursor_position = from;
        }
    }

    /// Get current Buffer
    pub fn buffer(&mut self) -> &Vec<char> {
        &self.buffer
    }

    /// Get the literal value of StyledBuffer without styles
    pub fn literal(&self) -> String {
        let literal: &String = &self.buffer.clone().into_iter().collect();
        literal.to_string()
    }

    /// Get char at position
    pub fn char_at(&self, position: usize) -> Option<char> {
        Some(self.buffer[position])
    }

    /// Get the sub string from the provided range, or None if range is invalid
    pub fn sub_string(&self, start: usize, end: usize) -> Option<String> {
        if start < end && end <= self.len() {
            let slice: String = self.buffer[start..end].iter().clone().collect();
            return Some(slice);
        }
        None
    }

    /// Return the last keyword that contains alphabetic characters on the buffer or None
    pub fn last_alphabetic_keyword(&self) -> Option<String> {
        let mut keyword = String::new();

        for c in self.buffer.iter().rev() {
            if c.is_alphabetic() {
                keyword.insert(0, *c);
                continue;
            }

            break;
        }

        if keyword.is_empty() {
            None
        } else {
            Some(keyword)
        }
    }

    /// Get current Styles
    pub fn styles(&self) -> &Vec<Style> {
        &self.styles
    }

    /// Update the current list of styles
    pub fn set_styles(&mut self, styles: &mut Vec<Style>) {
        if self.len() == styles.len() {
            self.styles.clear();
            self.styles.append(styles);
        }
    }

    /// Set style for one character
    pub fn style_char(&mut self, position: usize, style: Style) {
        self.styles[position] = style;
    }

    /// Set style for a range of characters
    pub fn style_range(&mut self, start: usize, end: usize, style: Style) {
        let max = std::cmp::min(end, self.styles.len());
        for i in start..max {
            self.styles[i] = style.clone();
        }
    }

    /// Set one style for all characters
    pub fn style_all(&mut self, style: Style) {
        for i in 0..self.len() {
            self.styles[i] = style.clone();
        }
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
