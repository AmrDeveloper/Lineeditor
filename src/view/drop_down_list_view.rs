use std::io::stdout;
use std::io::Result;
use std::io::Write;

use crossterm::cursor::position;
use crossterm::cursor::MoveTo;
use crossterm::cursor::MoveToColumn;
use crossterm::cursor::MoveToNextLine;
use crossterm::cursor::MoveToPreviousLine;
use crossterm::terminal;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::terminal::ScrollUp;
use crossterm::QueueableCommand;

use crate::completion::Suggestion;
use crate::style::Style;
use crate::ListView;

#[derive(Default)]
pub struct DropDownListView {
    elements: Vec<Suggestion>,
    focus_style: Style,
    focus_position: i64,
    is_visible: bool,
}

impl ListView<Suggestion> for DropDownListView {
    fn render(&mut self) -> Result<()> {
        let mut stdout = std::io::BufWriter::new(std::io::stderr());

        let (_, rows) = terminal::size()?;
        let (start_column, start_row) = position()?;

        let mut number_of_scrolls = 0;
        if (start_row + 1 + self.elements.len() as u16) > rows {
            number_of_scrolls = (start_row + 1 + self.elements.len() as u16) - rows + 1;
            stdout.queue(ScrollUp(number_of_scrolls))?;
            stdout.queue(MoveToPreviousLine(number_of_scrolls))?;
        }

        for (index, suggestion) in self.elements.iter_mut().enumerate() {
            let content = &mut suggestion.content;
            stdout.queue(MoveToNextLine(1))?;
            stdout.queue(MoveToColumn(start_column))?;

            if index as i64 == self.focus_position {
                let mut current_styles = content.styles().clone();
                content.style_all(self.focus_style.clone());
                super::base::render_styled_buffer(&mut stdout, content)?;
                content.set_styles(&mut current_styles);
            } else {
                super::base::render_styled_buffer(&mut stdout, content)?;
            }
        }

        stdout.queue(MoveTo(start_column, start_row - number_of_scrolls))?;
        stdout.flush()?;
        Ok(())
    }

    fn clear(&self) -> Result<()> {
        let mut stdout = stdout();
        stdout.queue(Clear(ClearType::FromCursorDown))?;
        stdout.flush()?;
        Ok(())
    }

    fn set_visibility(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn set_focus_position(&mut self, position: i64) {
        self.focus_position = position;
    }

    fn set_focus_style(&mut self, style: Style) {
        self.focus_style = style;
    }

    fn focus_next(&mut self) {
        if self.focus_position < self.elements.len() as i64 - 1 {
            self.focus_position += 1;
        }
    }

    fn focus_previous(&mut self) {
        if self.focus_position > 0 {
            self.focus_position -= 1;
        }
    }

    fn clear_focus(&mut self) {
        self.focus_position = 0;
    }

    fn reset(&mut self) {
        self.clear_elements();
        self.clear_focus();
    }

    fn set_elements(&mut self, elements: &mut Vec<Suggestion>) {
        self.elements.append(elements);
    }

    fn clear_elements(&mut self) {
        self.elements.clear();
    }

    fn selected_element(&self) -> Option<&Suggestion> {
        self.elements.get(self.focus_position as usize)
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn len(&self) -> usize {
        self.elements.len()
    }
}
