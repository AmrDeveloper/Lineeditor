use lineeditor::style::Style;
use lineeditor::styled_buffer::StyledBuffer;
use lineeditor::Color;
use lineeditor::Highlighter;
use lineeditor::LineEditor;
use lineeditor::LineEditorResult;
use lineeditor::StringPrompt;

const GITQL_RESERVED_KEYWORDS: [&str; 31] = [
    "set", "select", "distinct", "from", "group", "where", "having", "offset", "limit", "order",
    "by", "case", "when", "then", "else", "end", "between", "in", "is", "not", "like", "glob",
    "or", "and", "xor", "true", "false", "null", "as", "asc", "desc",
];

#[derive(Default)]
pub struct GitQLHighlighter {}

impl Highlighter for GitQLHighlighter {
    fn highlight(&self, buffer: &mut StyledBuffer) {
        let lines = buffer.buffer().clone();
        let mut i: usize = 0;

        let mut keyword_style = Style::default();
        keyword_style.set_foreground_color(Color::Magenta);

        let mut string_style = Style::default();
        string_style.set_foreground_color(Color::Yellow);

        loop {
            if i >= lines.len() {
                break;
            }

            // Highlight String literal
            if lines[i] == '"' {
                buffer.style_char(i, string_style.clone());
                i += 1;

                while i < lines.len() && lines[i] != '"' {
                    buffer.style_char(i, string_style.clone());
                    i += 1;
                }

                if i < lines.len() && lines[i] == '"' {
                    buffer.style_char(i, string_style.clone());
                    i += 1;
                }

                continue;
            }

            // Highlight reserved keyword
            if lines[i].is_alphabetic() {
                let start = i;
                let mut keyword = String::new();
                while i < lines.len() && (lines[i].is_alphanumeric() || lines[i] == '_') {
                    keyword.push(lines[i]);
                    i += 1;
                }

                keyword = keyword.to_lowercase();
                if GITQL_RESERVED_KEYWORDS.contains(&keyword.as_str()) {
                    buffer.style_range(start, i, keyword_style.clone())
                }
                continue;
            }

            i += 1;
        }
    }
}

fn main() {
    let prompt = StringPrompt::new("prompt> ".to_string());
    let mut line_editor = LineEditor::new(Box::new(prompt));

    let bindings = line_editor.keybinding();
    bindings.register_common_control_bindings();

    line_editor.add_highlighter(Box::<GitQLHighlighter>::default());

    match line_editor.read_line() {
        Ok(LineEditorResult::Success(line)) => {
            println!("Line {}", line);
        }
        _ => {}
    }
}
