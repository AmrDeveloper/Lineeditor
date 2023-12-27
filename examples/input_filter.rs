use lineeditor::input_filter::InputFilter;
use lineeditor::LineEditor;
use lineeditor::LineEditorResult;
use lineeditor::StringPrompt;

fn main() {
    let prompt = StringPrompt::new("prompt> ".to_string());
    let mut line_editor = LineEditor::new(Box::new(prompt));

    line_editor.set_input_filter(InputFilter::Options(vec![
        Box::new(InputFilter::Digit),
        Box::new(InputFilter::Whitespace),
    ]));

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
