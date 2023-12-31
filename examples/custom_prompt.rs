use lineeditor::styled_buffer::StyledBuffer;
use lineeditor::LineEditor;
use lineeditor::LineEditorResult;
use lineeditor::Prompt;

pub struct CurrentPathPrompt {}

impl Prompt for CurrentPathPrompt {
    fn prompt(&self) -> StyledBuffer {
        let path = if let Ok(current_dir) = std::env::current_dir() {
            current_dir.to_string_lossy().to_string()
        } else {
            "".to_owned()
        };
        let mut styled_buffer = StyledBuffer::default();
        styled_buffer.insert_string(&format!("📁 {}> ", path));
        styled_buffer
    }
}

fn main() {
    let prompt = CurrentPathPrompt {};
    let mut line_editor = LineEditor::new(Box::new(prompt));

    let bindings = line_editor.keybinding();
    bindings.register_common_control_bindings();

    match line_editor.read_line() {
        Ok(LineEditorResult::Success(line)) => {
            println!("Line {}", line);
        }
        _ => {}
    }
}
