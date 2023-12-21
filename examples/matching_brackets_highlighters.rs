use lineeditor::style::Style;
use lineeditor::styled_buffer::StyledBuffer;
use lineeditor::Color;
use lineeditor::Highlighter;
use lineeditor::LineEditor;
use lineeditor::LineEditorResult;
use lineeditor::StringPrompt;

#[derive(Default)]
pub struct MatchingBracketsHighlighter {}

impl Highlighter for MatchingBracketsHighlighter {
    fn highlight(&self, buffer: &mut StyledBuffer) {
        let colors = vec![Color::Red, Color::Blue, Color::Yellow, Color::Green];
        let mut brackets_stack: Vec<Color> = vec![];
        let mut current_color_index = 0;

        let lines = buffer.buffer().clone();
        let mut i: usize = 0;
        loop {
            if i >= lines.len() {
                break;
            }

            if lines[i] == '"' {
                i += 1;
                while i < lines.len() && lines[i] != '"' {
                    i += 1;
                }

                if i < lines.len() {
                    i += 1;
                }
                continue;
            }

            if lines[i] == '(' || lines[i] == '<' || lines[i] == '[' || lines[i] == '{' {
                if current_color_index >= colors.len() {
                    current_color_index = 0;
                }

                let color = colors[current_color_index];
                current_color_index += 1;

                brackets_stack.push(color);

                let mut style = Style::default();
                style.set_forground_color(color);
                buffer.style_char(i, style);
                i += 1;
                continue;
            }

            if lines[i] == ')' || lines[i] == '>' || lines[i] == ']' || lines[i] == '}' {
                let color = if brackets_stack.is_empty() {
                    colors[0]
                } else {
                    brackets_stack.pop().unwrap()
                };

                let mut style = Style::default();
                style.set_forground_color(color);
                buffer.style_char(i, style);

                i += 1;
                continue;
            }
            i += 1;
        }
    }
}

fn main() {
    let prompt = StringPrompt::new("gql> ".to_string());
    let mut line_editor = LineEditor::new(Box::new(prompt));
    line_editor.add_highlighter(Box::<MatchingBracketsHighlighter>::default());

    match line_editor.read_line() {
        Ok(LineEditorResult::Success(line)) => {
            println!("Line {}", line);
        }
        _ => {}
    }
}
