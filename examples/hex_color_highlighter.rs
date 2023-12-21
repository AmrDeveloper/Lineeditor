use lineeditor::style::Style;
use lineeditor::styled_buffer::StyledBuffer;
use lineeditor::Color;
use lineeditor::Highlighter;
use lineeditor::LineEditor;
use lineeditor::LineEditorResult;
use lineeditor::StringPrompt;

#[derive(Default)]
pub struct HexColorHighlighter {}

impl Highlighter for HexColorHighlighter {
    fn highlight(&self, buffer: &mut StyledBuffer) {
        let lines = buffer.buffer().clone();
        let mut i: usize = 0;

        loop {
            if i >= lines.len() {
                break;
            }

            // Highlight String literal
            if lines[i] == '"' {
                i += 1;

                while i < lines.len() && lines[i] != '"' {
                    i += 1;
                }

                if i < lines.len() && lines[i] == '"' {
                    i += 1;
                }

                continue;
            }

            // Highlight hex value background with it value
            if lines[i] == '#' && i + 6 < lines.len() {
                let start = i;
                i += 1;
                let hex_value = &lines[i..i + 6];
                for ch in hex_value {
                    if !ch.is_ascii_hexdigit() {
                        return;
                    }
                }
                let hex_string = hex_value.iter().cloned().collect::<String>();
                if let Ok(hex_value) = usize::from_str_radix(&hex_string, 16) {
                    let red = (hex_value >> 16) & 0xFF;
                    let green = (hex_value >> 8) & 0xFF;
                    let blue = hex_value & 0xFF;
                    let mut style = Style::default();
                    style.set_background_color(Color::Rgb {
                        r: red as u8,
                        g: green as u8,
                        b: blue as u8,
                    });
                    buffer.style_range(start, start + 7, style);
                }
            }

            i += 1;
        }
    }
}

fn main() {
    let prompt = StringPrompt::new("gql> ".to_string());
    let mut line_editor = LineEditor::new(Box::new(prompt));

    line_editor.add_highlighter(Box::<HexColorHighlighter>::default());

    let bindings = line_editor.keybinding();
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
