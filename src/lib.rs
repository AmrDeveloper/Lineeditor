mod core;
pub use core::editor;
pub use core::event;
pub use core::input_filter;
pub use core::keybindings;
pub use core::style;
pub use core::styled_buffer;

mod engine;
pub use engine::LineEditor;
pub use engine::LineEditorResult;

mod prompt;
pub use prompt::Prompt;
pub use prompt::StringPrompt;

mod autopair;
pub use autopair::AutoPair;
pub use autopair::DefaultAutoPair;
pub use autopair::DEFAULT_PAIRS;

mod hinter;
pub use hinter::Hinter;

mod highlighter;
pub use highlighter::Highlighter;

mod completion;
pub use completion::Completer;
pub use completion::Span;
pub use completion::Suggestion;

mod view;
pub use view::drop_down_list_view::DropDownListView;
pub use view::list_view::ListView;
pub use view::styled_editor_view;

// Reexport the key types to be independent from an explicit crossterm dependency.
pub use crossterm::cursor::SetCursorStyle;
pub use crossterm::event::KeyCode;
pub use crossterm::event::KeyEventKind;
pub use crossterm::event::KeyModifiers;
pub use crossterm::style::Color;
