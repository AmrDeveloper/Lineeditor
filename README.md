<h1 align="center">LineEditor</h1></br>

<p align="center">
  <img alt="Crates.io" src="https://img.shields.io/crates/v/lineeditor?style=flat-square">
  <img alt="Deps" src="https://deps.rs/repo/github/amrdeveloper/lineeditor/status.svg">
  <img alt="GitHub release" src="https://img.shields.io/github/v/release/amrdeveloper/Lineeditor">
  <img alt="GitHub issues" src="https://img.shields.io/github/issues/amrdeveloper/Lineeditor">
  <img alt="GitHub" src="https://img.shields.io/github/license/amrdeveloper/lineeditor">
</p>

<p align="center">
A new cross platform Line editor implementation inspired by reedline, rustyline and other cool line editors, designed with the goal to provides the core components that allows you to build your custom line editor with as mush as customization options.
</p>

<p align="center">
  <img src="media/line_editor_demo.gif" alt="animated" width="100%"/>
</p>

<p align="center">
  <img src="media/auto_complete_demo.gif" alt="animated" width="100%"/>
</p>

> [!IMPORTANT]
> LineEditor API is still experimental and may be changed from version to version until finishing the design and implementations of the basics features

### Basic example

```rs
let prompt = StringPrompt::new("prompt> ".to_string());
let line_editor = LineEditor::new(Box::new(prompt));
match line_editor.read_line() {
    Ok(LineEditorResult::Success(buffer)) => {
        
    }
    _ => {}
}
```

### Customization examples
- [Text Prompt](/examples/text_prompt.rs)
- [Custom Prompt](/examples/custom_prompt.rs)
- [Cursor style](/examples/cursor_style.rs)
- [Input Filter](/examples/input_filter.rs)
- [Key bindings](/examples/key_bindings.rs)
- [DropDown AutoComplete](/examples/drop_down_auto_complete.rs)
- [Keywords Highlighter](/examples/keyword_highlighter.rs)
- [Matching Brackets Highlighter](/examples/matching_brackets_highlighter.rs)
- [Hex Color Highlighter](/examples/hex_color_highlighter.rs)
- [Keywords Hinter](/examples/keyword_hinter.rs)
- [Auto Pair complete](/examples/auto_pair.rs)
- [Visual Selection](/examples/visual_selection.rs)

### License
```
MIT License

Copyright (c) 2023 Amr Hesham

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```