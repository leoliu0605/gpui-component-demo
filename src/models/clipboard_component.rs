use gpui::*;
use gpui_component::input::{Input, InputState};
use gpui_component::*;
use gpui_component::{clipboard::Clipboard, label::Label};

use crate::models::{ComponentMeta, subtitle};

pub struct ClipboardComponentView;

impl ComponentMeta for ClipboardComponentView {
    const DESCRIPTION: &'static str = "The Clipboard component provides an easy way to copy text or other data to the user's clipboard. \nIt renders as a button with a copy icon that changes to a checkmark when content is successfully copied. \nThe component supports both static values and dynamic content through callback functions.";
    const LINK: &'static str =
        "https://longbridge.github.io/gpui-component/docs/components/clipboard";
}

impl Render for ClipboardComponentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Basic Clipboard"))
            .child(self.basic_clipboard())
            .child(subtitle("Using Dynamic Values"))
            .child(self.using_dynamic_values(_cx))
            .child(subtitle("With Custom Content"))
            .child(self.with_custom_content())
            .child(subtitle("In Input Fields"))
            .child(self.in_input_fields(_window, _cx))
            .child(subtitle("Simple Text Copy"))
            .child(self.simple_text_copy())
            .child(subtitle("ClipbWith User Feedbackoard"))
            .child(self.with_user_feedback())
            .child(subtitle("Form Field Integration"))
            .child(self.form_field_integration())
            .child(subtitle("Dynamic Content Copy"))
            .child(self.dynamic_content_copy(_cx))
    }
}

impl ClipboardComponentView {
    /// Example code for the Clipboard component

    fn basic_clipboard(&self) -> AnyElement {
        Clipboard::new("my-clipboard")
            .value("Text to copy")
            .on_copied(|value, window, cx| {
                window.push_notification(format!("Copied: {}", value), cx)
            })
            .into_any_element()
    }

    fn using_dynamic_values(&self, cx: &mut Context<Self>) -> AnyElement {
        let state = cx.new(|_| DynamicState {
            value: "Dynamic content to copy".to_string(),
        });

        Clipboard::new("dynamic-clipboard")
            .value_fn({
                let state = state.clone();
                move |_, cx| SharedString::from(state.read(cx).value.clone())
            })
            .on_copied(|value, window, cx| {
                window.push_notification(format!("Copied: {}", value), cx)
            })
            .into_any_element()
    }

    fn with_custom_content(&self) -> AnyElement {
        h_flex()
            .gap_2()
            .child(Label::new("Share URL"))
            .child(Icon::new(IconName::ExternalLink))
            .child(Clipboard::new("custom-clipboard").value("https://example.com"))
            .into_any_element()
    }

    fn in_input_fields(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let url_state =
            cx.new(|cx| InputState::new(window, cx).default_value("https://github.com"));

        Input::new(&url_state)
            .suffix(
                Clipboard::new("url-clipboard")
                    .value_fn({
                        let state = url_state.clone();
                        move |_, cx| state.read(cx).value()
                    })
                    .on_copied(|value, window, cx| {
                        window.push_notification(format!("URL copied: {}", value), cx)
                    }),
            )
            .into_any_element()
    }

    fn simple_text_copy(&self) -> AnyElement {
        Clipboard::new("simple")
            .value("Hello, World!")
            .into_any_element()
    }

    fn with_user_feedback(&self) -> AnyElement {
        h_flex()
            .gap_2()
            .child(Label::new("Your API Key:"))
            .child(
                Clipboard::new("feedback")
                    .value("sk-1234567890abcdef")
                    .on_copied(|_, window, cx| {
                        window.push_notification("API key copied to clipboard", cx)
                    }),
            )
            .into_any_element()
    }

    fn form_field_integration(&self) -> AnyElement {
        // let input_state =
        //     cx.new(|cx| InputState::new(window, cx).default_value("https://github.com"));
        // let api_key = "sk-1234567890abcdef";

        // h_flex()
        //     .gap_2()
        //     .items_center()
        //     .child(Label::new("API Key:"))
        //     .child(
        //         Input::new(&input_state)
        //             .value(api_key)
        //             .readonly(true)
        //             .suffix(Clipboard::new("api-key-copy").value(api_key).on_copied(
        //                 |_, window, cx| window.push_notification("API key copied!", cx),
        //             )),
        //     )
        //     .into_any_element()

        div()
            .text_color(rgb(0x666666))
            .child("Not implemented yet")
            .into_any_element()
    }

    fn dynamic_content_copy(&self, cx: &mut Context<Self>) -> AnyElement {
        let app_state = cx.new(|_| AppState {
            current_url: "https://example.com".to_string(),
        });

        Clipboard::new("current-url")
            .value_fn({
                let state = app_state.clone();
                move |_, cx| SharedString::from(state.read(cx).current_url.clone())
            })
            .on_copied(|url, window, cx| window.push_notification(format!("Shared: {}", url), cx))
            .into_any_element()
    }
}

struct AppState {
    current_url: String,
}

struct DynamicState {
    value: String,
}
