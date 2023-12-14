mod core;
pub use core::editor;
pub use core::event;
pub use core::keybindings;
pub use core::style;
pub use core::styled_buffer;

mod engine;
pub use engine::LineEditor;

mod prompt;
pub use prompt::Prompt;
pub use prompt::StringPrompt;

mod painter;
pub use painter::Painter;

mod autopair;
pub use autopair::AutoPair;
pub use autopair::DefaultAutoPair;
pub use autopair::DEFAULT_PAIRS;

mod hinter;
pub use hinter::Hinter;

// Reexport the key types to be independent from an explicit crossterm dependency.
pub use crossterm::event::KeyCode;
pub use crossterm::event::KeyEventKind;
pub use crossterm::event::KeyModifiers;
pub use crossterm::style::Color;
