use lineeditor::event::LineEditorEvent;
use lineeditor::keybindings::KeyCombination;
use lineeditor::styled_buffer::StyledBuffer;
use lineeditor::Completer;
use lineeditor::KeyModifiers;
use lineeditor::LineEditor;
use lineeditor::LineEditorResult;
use lineeditor::Span;
use lineeditor::StringPrompt;
use lineeditor::Suggestion;

const GITQL_RESERVED_KEYWORDS: [&str; 31] = [
    "set", "select", "distinct", "from", "group", "where", "having", "offset", "limit", "order",
    "by", "case", "when", "then", "else", "end", "between", "in", "is", "not", "like", "glob",
    "or", "and", "xor", "true", "false", "null", "as", "asc", "desc",
];

pub struct FixedCompleter {}

impl Completer for FixedCompleter {
    fn complete(&self, input: &StyledBuffer) -> Vec<Suggestion> {
        let mut suggestions: Vec<Suggestion> = vec![];
        if input.position() != input.len() {
            return suggestions;
        }

        if let Some(keyword) = input.last_alphabetic_keyword() {
            for reserved_keyword in GITQL_RESERVED_KEYWORDS {
                if reserved_keyword.starts_with(&keyword) {
                    let suggestion = Suggestion {
                        content: StyledBuffer::from(reserved_keyword),
                        span: Span {
                            start: input.len() - keyword.len(),
                            end: input.len(),
                        },
                    };
                    suggestions.push(suggestion);
                }
            }
        }
        suggestions
    }
}

fn main() {
    let prompt = StringPrompt::new("prompt> ".to_string());
    let mut line_editor = LineEditor::new(Box::new(prompt));

    line_editor.set_completer(Box::new(FixedCompleter {}));

    let bindings = line_editor.keybinding();

    bindings.register_binding(
        KeyCombination {
            key_kind: lineeditor::KeyEventKind::Press,
            modifier: KeyModifiers::NONE,
            key_code: lineeditor::KeyCode::Tab,
        },
        LineEditorEvent::ToggleAutoComplete,
    );
    bindings.register_common_control_bindings();
    bindings.register_common_navigation_bindings();
    bindings.register_common_edit_bindings();
    bindings.register_common_selection_bindings();

    match line_editor.read_line() {
        Ok(LineEditorResult::Success(line)) => {
            println!("Line {}", line);
        }
        _ => {}
    }
}
