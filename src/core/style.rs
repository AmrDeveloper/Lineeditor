use crossterm::style::Attribute;
use crossterm::style::Color;

/// Represent the forground, background colors and attributes
#[derive(Clone)]
pub struct Style {
    /// Optional forground color
    forground: Option<Color>,
    /// Optional background color
    background: Option<Color>,
    /// Set of attributes like Bold, Italic, Undercurled...etc.
    attributes: Vec<Attribute>,
}

/// Create default instance of Style
impl Default for Style {
    fn default() -> Self {
        Style {
            forground: None,
            background: None,
            attributes: vec![],
        }
    }
}

impl Style {
    /// Get the style forground color
    pub fn set_forground_color(&mut self, color: Color) {
        self.forground = Some(color);
    }

    /// Get the style forground color
    pub fn forground_color(&self) -> &Option<Color> {
        &self.forground
    }

    /// Get the style background color
    pub fn set_background_color(&mut self, color: Color) {
        self.background = Some(color);
    }

    /// Get the style background color
    pub fn background_color(&self) -> &Option<Color> {
        &self.background
    }

    /// Get the style attributes
    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }

    /// Add attribute to this style
    pub fn add_attribute(&mut self, attribute: Attribute) {
        self.attributes.push(attribute);
    }

    /// Remove all attributes for this style
    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }
}
