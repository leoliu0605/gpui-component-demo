use gpui::*;
use gpui_component::checkbox::Checkbox;
use gpui_component::*;

use crate::models::{ComponentMeta, subtitle};

pub struct CheckboxComponentView {
    is_checked: bool,
    agree_terms: bool,
    subscribe: bool,
}

impl ComponentMeta for CheckboxComponentView {
    const DESCRIPTION: &'static str = "A checkbox component for binary choices. \nSupports labels, disabled state, and different sizes.";
    const LINK: &'static str =
        "https://longbridge.github.io/gpui-component/docs/components/checkbox";
}

impl CheckboxComponentView {
    pub fn new() -> Self {
        Self {
            is_checked: false,
            agree_terms: false,
            subscribe: false,
        }
    }
}

impl Render for CheckboxComponentView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Basic Checkbox"))
            .child(self.basic_checkbox())
            .child(subtitle("Controlled Checkbox"))
            .child(self.controlled_checkbox(cx))
            .child(subtitle("Different Sizes"))
            .child(self.different_sizes())
            .child(subtitle("Disabled State"))
            .child(self.disabled_state())
            .child(subtitle("Without Label"))
            .child(self.without_label())
            .child(subtitle("Custom Tab Order"))
            .child(self.custom_tab_order())
            .child(subtitle("Checkbox List"))
            .child(self.checkbox_list())
            .child(subtitle("Form Integration"))
            .child(self.form_integration(cx))
    }
}

impl CheckboxComponentView {
    /// Example code for the Checkbox component

    fn basic_checkbox(&self) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                Checkbox::new("my-checkbox")
                    .label("Accept terms and conditions")
                    .checked(false)
                    .on_click(|checked, _, _| {
                        println!("Checkbox is now: {}", checked);
                    })
            })
            .into_any_element()
    }

    fn controlled_checkbox(&self, cx: &mut Context<Self>) -> AnyElement {
        h_flex()
            .gap_4()
            .child({
                Checkbox::new("checkbox")
                    .label("Option")
                    .checked(self.is_checked)
                    .on_click(cx.listener(|view, checked, _, cx| {
                        view.is_checked = *checked;
                        cx.notify();
                    }))
            })
            .into_any_element()
    }

    fn different_sizes(&self) -> AnyElement {
        h_flex()
            .gap_4()
            .child(Checkbox::new("cb-xs").text_xs().label("Extra Small"))
            .child(Checkbox::new("cb-sm").text_sm().label("Small"))
            .child(
                Checkbox::new("cb-md").label("Medium"), // default
            )
            .child(Checkbox::new("cb-lg").text_lg().label("Large"))
            .into_any_element()
    }

    fn disabled_state(&self) -> AnyElement {
        Checkbox::new("checkbox")
            .label("Disabled checkbox")
            .disabled(true)
            .checked(false)
            .into_any_element()
    }

    fn without_label(&self) -> AnyElement {
        Checkbox::new("checkbox-no-label")
            .checked(true)
            .into_any_element()
    }

    fn custom_tab_order(&self) -> AnyElement {
        Checkbox::new("checkbox-tab-order")
            .label("Custom tab order")
            .tab_index(2)
            .tab_stop(true)
            .into_any_element()
    }

    fn checkbox_list(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(Checkbox::new("cb1").label("Option 1").checked(true))
            .child(Checkbox::new("cb2").label("Option 2").checked(false))
            .child(Checkbox::new("cb3").label("Option 3").checked(false))
            .into_any_element()
    }

    fn form_integration(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                Checkbox::new("terms")
                    .label("I agree to the terms and conditions")
                    .checked(self.agree_terms)
                    .on_click(cx.listener(|view, checked, _, cx| {
                        view.agree_terms = *checked;
                        cx.notify();
                    })),
            )
            .child(
                Checkbox::new("subscribe")
                    .label("Subscribe to newsletter")
                    .checked(self.subscribe)
                    .on_click(cx.listener(|view, checked, _, cx| {
                        view.subscribe = *checked;
                        cx.notify();
                    })),
            )
            .into_any_element()
    }
}
