mod core;
pub use core::editor;
pub use core::event;
pub use core::style;
pub use core::styled_buffer;

mod engine;
pub use engine::LineEditor;

mod prompt;
pub use prompt::Prompt;
pub use prompt::StringPrompt;

mod painter;
pub use painter::Painter;

// Reexport the key types to be independent from an explicit crossterm dependency.
pub use crossterm::event::KeyCode;
pub use crossterm::event::KeyEventKind;
pub use crossterm::event::KeyModifiers;
pub use crossterm::style::Color;
