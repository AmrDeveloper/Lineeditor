use std::io::Result;

use crate::style::Style;

pub trait ListView<T> {
    fn render(&mut self) -> Result<()>;
    fn clear(&self) -> Result<()>;
    fn set_visibility(&mut self, visible: bool);
    fn is_visible(&self) -> bool;

    fn set_focus_position(&mut self, position: i64);
    fn set_focus_style(&mut self, style: Style);
    fn focus_next(&mut self);
    fn focus_previous(&mut self);
    fn clear_focus(&mut self);
    fn reset(&mut self);

    fn set_elements(&mut self, elements: &mut Vec<T>);
    fn clear_elements(&mut self);
    fn selected_element(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
}
