use gpui::*;
use gpui_component::button::{Button, ButtonVariants, DropdownButton};
use gpui_component::*;

use crate::models::{ComponentMeta, subtitle};

pub struct DropdownButtonComponentView;

impl ComponentMeta for DropdownButtonComponentView {
    const DESCRIPTION: &'static str = "A DropdownButton is a combination of a button and a trigger button. \nIt allows us to display a dropdown menu when the trigger is clicked, \nbut the left Button can still respond to independent events. \n\nAnd more option methods of Button are also available for the DropdownButton, \nsuch as setting different variants using ButtonCustomVariant, sizes using Sizable, adding icons, loading states.";
    const LINK: &'static str =
        "https://longbridge.github.io/gpui-component/docs/components/dropdown_button";
}

impl Render for DropdownButtonComponentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Basic Dropdown Button"))
            .child(self.basic_dropdown_button())
            .child(subtitle("Variants"))
            .child(self.variants())
    }
}

impl DropdownButtonComponentView {
    /// Example code for the DropdownButton component

    fn basic_dropdown_button(&self) -> AnyElement {
        DropdownButton::new("dropdown-basic")
            .button(Button::new("btn-basic").label("Click Me"))
            .dropdown_menu(|menu, _, _| {
                menu.menu("Option 1", Box::new(MyAction))
                    .menu("Option 2", Box::new(MyAction))
                    .separator()
                    .menu("Option 3", Box::new(MyAction))
            })
            .into_any_element()
    }

    fn variants(&self) -> AnyElement {
        h_flex()
            .gap_4()
            .child(
                DropdownButton::new("dropdown-primary")
                    .primary()
                    .button(Button::new("btn-primary").label("Primary"))
                    .dropdown_menu(|menu, _, _| menu.menu("Option 1", Box::new(MyAction))),
            )
            .child(
                // With custom anchor
                DropdownButton::new("dropdown-custom-anchor")
                    .button(Button::new("btn-custom-anchor").label("Click Me"))
                    .dropdown_menu_with_anchor(Corner::BottomRight, |menu, _, _| {
                        menu.menu("Option 1", Box::new(MyAction))
                    }),
            )
            .into_any_element()
    }
}

actions!(dropdown_button, [MyAction]);
