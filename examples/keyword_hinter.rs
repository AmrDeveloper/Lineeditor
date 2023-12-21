use lineeditor::style::Style;
use lineeditor::styled_buffer::StyledBuffer;
use lineeditor::Color;
use lineeditor::Hinter;
use lineeditor::LineEditor;
use lineeditor::LineEditorResult;
use lineeditor::StringPrompt;

const GITQL_RESERVED_KEYWORDS: [&str; 31] = [
    "set", "select", "distinct", "from", "group", "where", "having", "offset", "limit", "order",
    "by", "case", "when", "then", "else", "end", "between", "in", "is", "not", "like", "glob",
    "or", "and", "xor", "true", "false", "null", "as", "asc", "desc",
];

#[derive(Default)]
pub struct GitQLHinter {}

impl Hinter for GitQLHinter {
    fn hint(&self, buffer: &mut StyledBuffer) -> Option<StyledBuffer> {
        if let Some(keyword) = buffer.last_alphabetic_keyword() {
            let keyword_lower = keyword.to_lowercase();
            for word in GITQL_RESERVED_KEYWORDS {
                if word.starts_with(&keyword_lower) {
                    let hint = &word[keyword.len()..];
                    let mut styled_buffer = StyledBuffer::default();
                    let mut style = Style::default();
                    style.set_forground_color(Color::DarkGrey);
                    styled_buffer.insert_styled_string(hint, style);
                    return Some(styled_buffer);
                }
            }
        }
        None
    }
}

fn main() {
    let prompt = StringPrompt::new("gql> ".to_string());
    let mut line_editor = LineEditor::new(Box::new(prompt));
    line_editor.add_hinter(Box::<GitQLHinter>::default());

    match line_editor.read_line() {
        Ok(LineEditorResult::Success(line)) => {
            println!("Line {}", line);
        }
        _ => {}
    }
}
