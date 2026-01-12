use gpui::*;
use gpui_component::button::{Button, ButtonCustomVariant, ButtonGroup, ButtonVariants};
use gpui_component::*;

use crate::models::{ComponentMeta, subtitle};

pub struct ButtonComponentView;

impl ComponentMeta for ButtonComponentView {
    const DESCRIPTION: &'static str = "The Button element with multiple variants, sizes, and states. \nSupports icons, loading states, and can be grouped together.";
    const LINK: &'static str = "https://longbridge.github.io/gpui-component/docs/components/button";
}

impl Render for ButtonComponentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Basic Button"))
            .child(self.basic_button())
            .child(subtitle("Variants"))
            .child(self.variants())
            .child(subtitle("Outline Buttons"))
            .child(self.outline_buttons())
            .child(subtitle("Compact Button"))
            .child(self.compact_button())
            .child(subtitle("Sizeable"))
            .child(self.sizeable())
            .child(subtitle("With Icons"))
            .child(self.with_icons())
            .child(subtitle("With a dropdown caret icon"))
            .child(self.with_a_dropdown_caret_icon())
            .child(subtitle("Button States"))
            .child(self.button_states())
            .child(subtitle("Button Group"))
            .child(self.button_group())
            .child(subtitle("Toggle Button Group"))
            .child(self.toggle_button_group())
            .child(subtitle("Custom Variant"))
            .child(self.custom_variant(_cx))
            .child(subtitle("With Tooltip"))
            .child(self.with_tooltip())
            .child(subtitle("Custom Children"))
            .child(self.custom_children())
    }
}

impl ButtonComponentView {
    /// Example code for the Button component

    fn basic_button(&self) -> AnyElement {
        Button::new("my-button")
            .label("Click me")
            .on_click(|_, _, _| {
                println!("Button clicked!");
            })
            .into_any_element()
    }

    fn variants(&self) -> AnyElement {
        h_flex()
            .gap_4()
            .child(
                // Primary button
                Button::new("btn-primary").primary().label("Primary"),
            )
            .child(
                // Secondary button (default)
                Button::new("btn-secondary").label("Secondary"),
            )
            .child(
                // Danger button
                Button::new("btn-danger").danger().label("Delete"),
            )
            .child(
                // Warning button
                Button::new("btn-warning").warning().label("Warning"),
            )
            .child(
                // Success button
                Button::new("btn-success").success().label("Success"),
            )
            .child(
                // Info button
                Button::new("btn-info").info().label("Info"),
            )
            .child(
                // Ghost button
                Button::new("btn-ghost").ghost().label("Ghost"),
            )
            .child(
                // Link button
                Button::new("btn-link").link().label("Link"),
            )
            .child(
                // Text button
                Button::new("btn-text").text().label("Text"),
            )
            .into_any_element()
    }

    fn outline_buttons(&self) -> AnyElement {
        h_flex()
            .gap_4()
            .child(
                Button::new("btn-primary-outline")
                    .primary()
                    .outline()
                    .label("Primary Outline"),
            )
            .child(
                Button::new("btn-danger-outline")
                    .danger()
                    .outline()
                    .label("Danger Outline"),
            )
            .into_any_element()
    }

    fn compact_button(&self) -> AnyElement {
        // Compact (reduced padding)
        Button::new("btn-compact")
            .label("Compact")
            .compact()
            .into_any_element()
    }

    fn sizeable(&self) -> AnyElement {
        h_flex()
            .gap_4()
            .child(Button::new("btn-xsmall").xsmall().label("Extra Small"))
            .child(Button::new("btn-small").small().label("Small"))
            .child(
                Button::new("btn-medium").label("Medium"), // default
            )
            .child(Button::new("btn-large").large().label("Large"))
            .into_any_element()
    }

    fn with_icons(&self) -> AnyElement {
        h_flex()
            .gap_4()
            .child(
                // Icon before label
                Button::new("btn-icon-check")
                    .icon(IconName::Check)
                    .label("Confirm"),
            )
            .child(
                // Icon only
                Button::new("btn-icon-search").icon(IconName::Search),
            )
            .child(
                // Custom icon size
                Button::new("btn-icon-heart")
                    .icon(Icon::new(IconName::Heart))
                    .label("Like"),
            )
            .into_any_element()
    }

    fn with_a_dropdown_caret_icon(&self) -> AnyElement {
        Button::new("btn-dropdown")
            .label("Options")
            .dropdown_caret(true)
            .into_any_element()
    }

    fn button_states(&self) -> AnyElement {
        h_flex()
            .gap_4()
            .child(
                // Disabled
                Button::new("btn-disabled").label("Disabled").disabled(true),
            )
            .child(
                // Loading
                Button::new("btn-loading").label("Loading").loading(true),
            )
            .child(
                // Selected
                Button::new("btn-selected").label("Selected").selected(true),
            )
            .into_any_element()
    }

    fn button_group(&self) -> AnyElement {
        ButtonGroup::new("btn-group")
            .child(Button::new("btn1").label("One"))
            .child(Button::new("btn2").label("Two"))
            .child(Button::new("btn3").label("Three"))
            .into_any_element()
    }

    fn toggle_button_group(&self) -> AnyElement {
        ButtonGroup::new("toggle-group")
            .multiple(true) // Allow multiple selections
            .child(Button::new("btn1").label("Option 1").selected(true))
            .child(Button::new("btn2").label("Option 2"))
            .child(Button::new("btn3").label("Option 3"))
            .on_click(|selected_indices, _, _| {
                println!("Selected: {:?}", selected_indices);
            })
            .into_any_element()
    }

    fn custom_variant(&self, cx: &Context<Self>) -> AnyElement {
        let custom = ButtonCustomVariant::new(cx)
            .color(cx.theme().magenta)
            .foreground(cx.theme().primary_foreground)
            .border(cx.theme().magenta)
            .hover(cx.theme().magenta.opacity(0.1))
            .active(cx.theme().magenta);

        Button::new("custom-btn")
            .custom(custom)
            .label("Custom Button")
            .into_any_element()
    }

    fn with_tooltip(&self) -> AnyElement {
        Button::new("btn")
            .label("Hover me")
            .tooltip("This is a helpful tooltip")
            .into_any_element()
    }

    fn custom_children(&self) -> AnyElement {
        Button::new("btn")
            .child(
                h_flex()
                    .items_center()
                    .gap_2()
                    .child("Custom Content")
                    .child(IconName::ChevronDown)
                    .child(IconName::Eye),
            )
            .into_any_element()
    }
}
