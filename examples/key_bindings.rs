use lineeditor::LineEditor;
use lineeditor::LineEditorResult;
use lineeditor::StringPrompt;

fn main() {
    let prompt = StringPrompt::new("gql> ".to_string());
    let mut line_editor = LineEditor::new(Box::new(prompt));

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
