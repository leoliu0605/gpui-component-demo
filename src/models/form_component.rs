use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::form::{Field, Form, field, h_form, v_form};
use gpui_component::input::{Input, InputState};
use gpui_component::switch::Switch;
use gpui_component::*;

use crate::models::{ComponentMeta, subtitle};

pub struct FormComponentView;

impl ComponentMeta for FormComponentView {
    const DESCRIPTION: &'static str = "A comprehensive form component that provides structured layout for form fields with support for vertical/horizontal layouts, \nvalidation, field groups, and responsive multi-column layouts.";
    const LINK: &'static str = "https://longbridge.github.io/gpui-component/docs/components/form";
}

impl Render for FormComponentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Basic Form"))
            .child(self.basic_form(_window, _cx))
            .child(subtitle("Horizontal Form Layout"))
            .child(self.horizontal_form_layout(_window, _cx))
            .child(subtitle("Multi-Column Form"))
            .child(self.multi_column_form(_window, _cx))
            .child(subtitle("Vertical Layout (Default)"))
            .child(self.vertical_layout(_window, _cx))
            .child(subtitle("Horizontal Layout"))
            .child(self.horizontal_layout(_window, _cx))
            .child(subtitle("Custom Sizing"))
            .child(self.custom_sizing(_window, _cx))
            .child(subtitle("Required Fields"))
            .child(self.required_fields(_window, _cx))
            .child(subtitle("Field Descriptions"))
            .child(self.field_descriptions(_window, _cx))
            .child(subtitle("Dynamic Descriptions"))
            .child(self.dynamic_descriptions(_window, _cx))
            .child(subtitle("Field Visibility"))
            .child(self.field_visibility())
            .child(subtitle("Basic Submit Pattern"))
            .child("Not implemented yet")
            .child(subtitle("Form with Action Buttons"))
            .child(self.form_with_action_buttons(_window, _cx))
            .child(subtitle("Related Fields"))
            .child(self.related_fields(_window, _cx))
            .child(subtitle("Custom Field Components"))
            .child("Not implemented yet")
            .child(subtitle("Conditional Fields"))
            .child("Not implemented yet")
            .child(subtitle("Column Spanning"))
            .child("Not implemented yet")
            .child(subtitle("Column Positioning"))
            .child("Not implemented yet")
            .child(subtitle("Responsive Layout"))
            .child("Not implemented yet")
            .child(subtitle("User Registration Form"))
            .child("Not implemented yet")
            .child(subtitle("Settings Form with Sections"))
            .child("Not implemented yet")
            .child(subtitle("Contact Form"))
            .child("Not implemented yet")
    }
}

impl FormComponentView {
    /// Example code for the Form component

    fn basic_form(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let name_input = cx.new(|cx| InputState::new(window, cx).placeholder("Enter your name"));
        let email_input = cx.new(|cx| InputState::new(window, cx).placeholder("Enter your email"));

        v_form()
            .child(field().label("Name").child(Input::new(&name_input)))
            .child(
                field()
                    .label("Email")
                    .child(Input::new(&email_input))
                    .required(true),
            )
            .into_any_element()
    }

    fn horizontal_form_layout(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let first_name = cx.new(|cx| InputState::new(window, cx).placeholder("First Name"));
        let last_name = cx.new(|cx| InputState::new(window, cx).placeholder("Last Name"));

        h_form()
            .label_width(px(120.))
            .child(field().label("First Name").child(Input::new(&first_name)))
            .child(field().label("Last Name").child(Input::new(&last_name)))
            .into_any_element()
    }

    fn multi_column_form(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let first_name = cx.new(|cx| InputState::new(window, cx).placeholder("First Name"));
        let last_name = cx.new(|cx| InputState::new(window, cx).placeholder("Last Name"));
        let bio_input = cx.new(|cx| InputState::new(window, cx).placeholder("Short Bio"));

        v_form()
            .columns(2) // Two-column layout
            .child(field().label("First Name").child(Input::new(&first_name)))
            .child(field().label("Last Name").child(Input::new(&last_name)))
            .child(
                field()
                    .label("Bio")
                    .col_span(2) // Span across both columns
                    .child(Input::new(&bio_input)),
            )
            .into_any_element()
    }

    fn vertical_layout(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let input = cx.new(|cx| InputState::new(window, cx).placeholder("Your answer here"));
        let email_input = cx.new(|cx| InputState::new(window, cx).placeholder("Your email here"));

        v_form()
            .gap(px(12.))
            .child(field().label("Name").child(input))
            .child(field().label("Email").child(email_input))
            .into_any_element()
    }

    fn horizontal_layout(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let input = cx.new(|cx| InputState::new(window, cx).placeholder("Your answer here"));
        let email_input = cx.new(|cx| InputState::new(window, cx).placeholder("Your email here"));

        h_form()
            .label_width(px(100.))
            .child(field().label("Name").child(input))
            .child(field().label("Email").child(email_input))
            .into_any_element()
    }

    fn custom_sizing(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .gap_4()
            .child({
                let input = cx.new(|cx| InputState::new(window, cx).placeholder("Your title here"));
                v_form()
                    .large() // Large form size
                    .label_text_size(rems(1.2))
                    .child(field().label("Title").child(input))
            })
            .child({
                let input = cx.new(|cx| InputState::new(window, cx).placeholder("Your code here"));
                v_form()
                    .small() // Small form size
                    .child(field().label("Code").child(input))
            })
            .into_any_element()
    }

    fn required_fields(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let email_input = cx.new(|cx| InputState::new(window, cx).placeholder("Enter your email"));

        field()
            .label("Email")
            .required(true) // Shows asterisk (*) next to label
            .child(Input::new(&email_input))
            .into_any_element()
    }

    fn field_descriptions(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let password_input = cx.new(|cx| InputState::new(window, cx).placeholder("Enter password"));

        field()
            .label("Password")
            .description("Must be at least 8 characters long")
            .child(Input::new(&password_input))
            .into_any_element()
    }

    fn dynamic_descriptions(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let bio_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("Tell us about yourself"));

        field()
            .label("Bio")
            .description_fn(|_, _| div().child("Use at most 100 words to describe yourself."))
            .child(Input::new(&bio_input))
            .into_any_element()
    }

    fn field_visibility(&self) -> AnyElement {
        let is_admin = true; // This would typically come from application state

        field()
            .label("Admin Settings")
            .visible(is_admin) // Conditionally show field
            .child(Switch::new("admin-mode"))
            .into_any_element()
    }

    fn form_with_action_buttons(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let title = cx.new(|cx| InputState::new(window, cx).placeholder("Enter title"));
        let content = cx.new(|cx| InputState::new(window, cx).placeholder("Enter content"));

        v_form()
            .child(field().label("Title").child(Input::new(&title)))
            .child(field().label("Content").child(Input::new(&content)))
            .child(
                field().label_indent(false).child(
                    h_flex()
                        .gap_2()
                        .child(Button::new("save").primary().child("Save"))
                        .child(Button::new("cancel").child("Cancel"))
                        .child(Button::new("preview").outline().child("Preview")),
                ),
            )
            .into_any_element()
    }

    fn related_fields(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let first_name = cx.new(|cx| InputState::new(window, cx).placeholder("First Name"));
        let last_name = cx.new(|cx| InputState::new(window, cx).placeholder("Last Name"));
        let street = cx.new(|cx| InputState::new(window, cx).placeholder("Street Address"));
        let city = cx.new(|cx| InputState::new(window, cx).placeholder("City"));
        let zip = cx.new(|cx| InputState::new(window, cx).placeholder("ZIP Code"));

        v_form()
            .child(
                field().label("Name").child(
                    h_flex()
                        .gap_2()
                        .child(div().flex_1().child(Input::new(&first_name)))
                        .child(div().flex_1().child(Input::new(&last_name))),
                ),
            )
            .child(
                field()
                    .label("Address")
                    .items_start() // Align to start for multi-line content
                    .child(
                        v_flex().gap_2().child(Input::new(&street)).child(
                            h_flex()
                                .gap_2()
                                .child(div().flex_1().child(Input::new(&city)))
                                .child(div().w(px(100.)).child(Input::new(&zip))),
                        ),
                    ),
            )
            .into_any_element()
    }
}
