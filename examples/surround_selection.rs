use lineeditor::style::Style;
use lineeditor::LineEditor;
use lineeditor::LineEditorResult;
use lineeditor::StringPrompt;

fn main() {
    let prompt = StringPrompt::new("prompt> ".to_string());
    let mut line_editor = LineEditor::new(Box::new(prompt));

    let mut style = Style::default();
    style.set_background_color(lineeditor::Color::Cyan);
    line_editor.set_visual_selection_style(Some(style));
    line_editor.enable_surround_selection(true);

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
