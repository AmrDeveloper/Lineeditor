mod core;
pub use core::editor;
pub use core::event;
pub use core::style;
pub use core::styled_buffer;

mod engine;
pub use engine::LineEditor;

mod painter;
pub use painter::Painter;

// Reexport the key types to be independent from an explicit crossterm dependency.
pub use crossterm::{
    event::{KeyCode, KeyEventKind, KeyModifiers},
    style::Color,
};
