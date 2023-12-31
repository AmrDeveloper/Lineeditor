use lineeditor::DefaultAutoPair;
use lineeditor::LineEditor;
use lineeditor::LineEditorResult;
use lineeditor::StringPrompt;

fn main() {
    let prompt = StringPrompt::new("prompt> ".to_string());
    let mut line_editor = LineEditor::new(Box::new(prompt));
    line_editor.set_auto_pair(Some(Box::<DefaultAutoPair>::default()));

    let bindings = line_editor.keybinding();
    bindings.register_common_control_bindings();

    match line_editor.read_line() {
        Ok(LineEditorResult::Success(line)) => {
            println!("Line {}", line);
        }
        _ => {}
    }
}
