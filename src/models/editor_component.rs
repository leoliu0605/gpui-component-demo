use gpui::*;
use gpui_component::button::Button;
use gpui_component::input::{Input, InputEvent, InputState, Position, TabSize};
use gpui_component::*;

use crate::models::{ComponentMeta, subtitle};

pub struct EditorComponentView;

impl ComponentMeta for EditorComponentView {
    const DESCRIPTION: &'static str = "A powerful multi-line text input component that extends the basic input functionality with support for multiple lines, \nauto-resizing, syntax highlighting, line numbers, and code editing features. \nPerfect for forms, code editors, and content editing.";
    const LINK: &'static str = "https://longbridge.github.io/gpui-component/docs/components/editor";
}

impl Render for EditorComponentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Textarea"))
            .child(self.textarea(_window, _cx))
            .child(subtitle("AutoGrow"))
            .child(self.autogrow(_window, _cx))
            .child(subtitle("CodeEditor"))
            .child(self.code_editor(_window, _cx))
            .child(subtitle("Single Line Mode"))
            .child(self.single_line_mode(_window, _cx))
            .child(subtitle("TabSize"))
            .child(self.tab_size(_window, _cx))
            .child(subtitle("Searchable"))
            .child(self.searchable(_window, _cx))
            .child(subtitle("SoftWrap"))
            .child(self.soft_wrap(_window, _cx))
            .child(subtitle("Text Manipulation"))
            .child(self.text_manipulation(_window, _cx))
            .child(subtitle("Validation"))
            .child(self.validation(_window, _cx))
            .child(subtitle("Handle Events"))
            .child(self.handle_events(_window, _cx))
            .child(subtitle("Disabled State"))
            .child(self.disabled_state(_window, _cx))
            .child(subtitle("Custom Styling"))
            .child(self.custom_styling(_window, _cx))
            .child(subtitle("Comment Box"))
            .child("Not implemented yet")
            .child(subtitle("Code Editor with Language Selection"))
            .child("Not implemented yet")
            .child(subtitle("Text Editor with Toolbar"))
            .child("Not implemented yet")
    }
}

impl EditorComponentView {
    /// Example code for the Editor component

    fn textarea(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let state = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .rows(10) // Set number of rows
                .placeholder("Enter text here...")
        });

        Input::new(&state)
            .h(px(320.)) // Set explicit height
            .into_any_element()
    }

    fn autogrow(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let state = cx.new(|cx| {
            InputState::new(window, cx)
                .auto_grow(1, 5) // min_rows: 1, max_rows: 5
                .placeholder("Type here and watch it grow...")
        });

        Input::new(&state).into_any_element()
    }

    fn code_editor(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let state = cx.new(|cx| {
            InputState::new(window, cx)
                .code_editor("rust") // Language for syntax highlighting
                .line_number(true) // Show line numbers
                .searchable(true) // Enable search functionality
                .default_value("fn main() {\n    println!(\"Hello, world!\");\n}")
        });

        Input::new(&state)
            .h_full() // Full height
            .into_any_element()
    }

    fn single_line_mode(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let state = cx.new(|cx| {
            InputState::new(window, cx)
                .code_editor("rust")
                .multi_line(false) // Single line
                .default_value("println!(\"Hello, world!\");")
        });

        Input::new(&state).into_any_element()
    }

    fn tab_size(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let state = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .tab_size(TabSize {
                    tab_size: 4,
                    hard_tabs: false, // Use spaces instead of tabs
                })
        });

        Input::new(&state).into_any_element()
    }

    fn searchable(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let state = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .searchable(true) // Enable Ctrl+F search
                .rows(15)
                .default_value("Search through this content...")
        });

        Input::new(&state).into_any_element()
    }

    fn soft_wrap(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                // With soft wrap (default)
                let state = cx.new(|cx|
                    InputState::new(window, cx)
                        .multi_line(true)
                        .soft_wrap(true)
                        .rows(6)
                );

                Input::new(&state)
            })
            .child({
                // Without soft wrap (horizontal scrolling)
                let state = cx.new(|cx|
                    InputState::new(window, cx)
                        .multi_line(true)
                        .soft_wrap(false)
                        .rows(6)
                        .default_value("This is a very long line that will not wrap automatically but will show horizontal scrollbar instead.")
                );

                Input::new(&state)
            })
            .into_any_element()
    }

    fn text_manipulation(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let state = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .rows(6)
                .default_value("Line 1\nLine 2\nLine 3")
                .placeholder("Try the actions below...")
        });

        let insert_state = state.clone();
        let replace_state = state.clone();
        let set_cursor_state = state.clone();
        let read_cursor_state = state.clone();

        v_flex()
            .gap_2()
            .child(Input::new(&state))
            .child(
                h_flex()
                    .gap_2()
                    .child(Button::new("editor-insert").label("Insert text").on_click(
                        move |_, window, cx| {
                            insert_state.update(cx, |state, cx| {
                                state.insert("inserted text", window, cx);
                            });
                        },
                    ))
                    .child(Button::new("editor-replace").label("Replace all").on_click(
                        move |_, window, cx| {
                            replace_state.update(cx, |state, cx| {
                                state.replace("new content", window, cx);
                            });
                        },
                    ))
                    .child(
                        Button::new("editor-set-cursor")
                            .label("Set cursor")
                            .on_click(move |_, window, cx| {
                                set_cursor_state.update(cx, |state, cx| {
                                    state.set_cursor_position(
                                        Position {
                                            line: 2,
                                            character: 5,
                                        },
                                        window,
                                        cx,
                                    );
                                });
                            }),
                    )
                    .child(
                        Button::new("editor-get-cursor")
                            .label("Get cursor")
                            .on_click(move |_, _window, cx| {
                                let position = read_cursor_state.read(cx).cursor_position();
                                println!("Line: {}, Column: {}", position.line, position.character);
                            }),
                    ),
            )
            .into_any_element()
    }

    fn validation(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let state = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .validate(|text, _| {
                    // Validate that content is not empty and under 1000 chars
                    !text.trim().is_empty() && text.len() <= 1000
                })
        });

        Input::new(&state).into_any_element()
    }

    fn handle_events(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let state = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .rows(6)
                .placeholder("Type here and try Enter or Shift+Enter...")
        });

        let _ = cx.subscribe_in(&state, window, |_, state, event, _window, cx| match event {
            InputEvent::Change => {
                let content = state.read(cx).value();
                println!("Content changed: {} characters", content.len());
            }
            InputEvent::PressEnter { secondary } => {
                if *secondary {
                    println!("Shift+Enter pressed - insert line break");
                } else {
                    println!("Enter pressed - could submit form");
                }
            }
            InputEvent::Focus => println!("Textarea focused"),
            InputEvent::Blur => println!("Textarea blurred"),
        });

        Input::new(&state).into_any_element()
    }

    fn disabled_state(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let state = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .default_value("This editor is disabled.")
        });

        Input::new(&state)
            .disabled(true)
            .h(px(200.))
            .into_any_element()
    }

    fn custom_styling(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let state = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .rows(6)
                .default_value("This editor has custom styling.")
        });

        v_flex()
            .gap_2()
            .child(
                // Without default appearance
                Input::new(&state).appearance(false).h(px(200.)),
            )
            .child(
                // Custom container styling
                div()
                    .bg(cx.theme().background)
                    .border_2()
                    .border_color(cx.theme().input)
                    .rounded_lg()
                    .p_4()
                    .child(Input::new(&state).appearance(false).h(px(150.))),
            )
            .into_any_element()
    }
}
