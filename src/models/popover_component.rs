use gpui::*;
use gpui_component::button::ButtonVariants;
use gpui_component::popover::Popover;
use gpui_component::*;
use gpui_component::{button::Button, divider::Divider};

use crate::models::{ComponentMeta, subtitle};

pub struct PopoverComponentView {
    open: bool,
}

impl PopoverComponentView {
    pub fn new() -> Self {
        Self { open: false }
    }
}

impl ComponentMeta for PopoverComponentView {
    const DESCRIPTION: &'static str = "Popover component for displaying floating content that appears when interacting with a trigger element. \nSupports multiple positioning options, custom content, different trigger methods, \nand automatic dismissal behaviors. Perfect for tooltips, menus, forms, and other contextual information.";
    const LINK: &'static str =
        "https://longbridge.github.io/gpui-component/docs/components/popover";
}

impl Render for PopoverComponentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Basic Popover"))
            .child(self.basic_popover())
            .child(subtitle("Popover with Custom Positioning"))
            .child(self.popover_with_custom_positioning())
            .child(subtitle("View in Popover"))
            .child(self.view_in_popover(_cx))
            .child(subtitle("Add content by content method"))
            .child(self.add_content_by_content_method())
            .child(subtitle("Right-Click Popover"))
            .child(self.right_click_popover())
            .child(subtitle("Dismiss Popover manually"))
            .child(self.dismiss_popover_manually())
            .child(subtitle("Styling Popover"))
            .child(self.styling_popover(_cx))
            .child(subtitle("Control Open State"))
            .child(self.control_open_state(_cx))
            .child(subtitle("Default Open"))
            .child(self.default_open())
    }
}

impl PopoverComponentView {
    /// Example code for the Popover component

    fn basic_popover(&self) -> AnyElement {
        Popover::new("basic-popover")
            .trigger(Button::new("trigger").label("Click me").outline())
            .child("Hello, this is a popover!")
            .child("It appears when you click the button.")
            .into_any_element()
    }

    fn popover_with_custom_positioning(&self) -> AnyElement {
        Popover::new("positioned-popover")
            .anchor(Corner::TopRight)
            .trigger(Button::new("top-right").label("Top Right").outline())
            .child("This popover appears at the top right")
            .into_any_element()
    }

    fn view_in_popover(&self, cx: &mut Context<Self>) -> AnyElement {
        let view = cx.new(|_| MyView::new());

        Popover::new("form-popover")
            .anchor(Corner::BottomLeft)
            .trigger(Button::new("show-form").label("Open Form").outline())
            .child(view.clone())
            .into_any_element()
    }

    fn add_content_by_content_method(&self) -> AnyElement {
        Popover::new("complex-popover")
            .anchor(Corner::BottomLeft)
            .trigger(Button::new("complex").label("Complex Content").outline())
            .content(|_, _, _| {
                div()
                    .child("This popover has complex content.")
                    .child(Button::new("action-btn").label("Perform Action").outline())
            })
            .into_any_element()
    }

    fn right_click_popover(&self) -> AnyElement {
        Popover::new("context-menu")
            .anchor(Corner::BottomRight)
            .mouse_button(MouseButton::Right)
            .trigger(Button::new("right-click").label("Right Click Me").outline())
            .child("Context Menu")
            .child(Divider::horizontal())
            .child("This is a custom context menu.")
            .into_any_element()
    }

    fn dismiss_popover_manually(&self) -> AnyElement {
        Popover::new("dismiss-popover")
            .trigger(Button::new("dismiss").label("Dismiss Popover").outline())
            .content(|_, _, cx| {
                div()
                    .child("Click the button below to dismiss this popover.")
                    .child(
                        Button::new("close-btn")
                            .label("Close Popover")
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(DismissEvent);
                            })),
                    )
            })
            .into_any_element()
    }

    fn styling_popover(&self, cx: &Context<Self>) -> AnyElement {
        // For custom styled popovers or when you want full control
        Popover::new("custom-popover")
            .appearance(false)
            .trigger(Button::new("custom").label("Custom Style"))
            .bg(cx.theme().accent)
            .text_color(cx.theme().accent_foreground)
            .p_6()
            .rounded_xl()
            .shadow_2xl()
            .child("Fully custom styled popover")
            .into_any_element()
    }

    fn control_open_state(&self, cx: &mut Context<Self>) -> AnyElement {
        Popover::new("controlled-popover")
            .open(self.open)
            .on_open_change(cx.listener(|this, open: &bool, _, cx| {
                this.open = *open;
                cx.notify();
            }))
            .trigger(
                Button::new("control-btn")
                    .label("Control Popover")
                    .outline(),
            )
            .child("This popover's open state is controlled programmatically.")
            .into_any_element()
    }

    fn default_open(&self) -> AnyElement {
        Popover::new("default-open-popover")
            .default_open(true)
            .trigger(
                Button::new("default-open-btn")
                    .label("Default Open")
                    .outline(),
            )
            .child("This popover is open by default when first rendered.")
            .into_any_element()
    }
}

struct MyView {
    name: String,
}

impl MyView {
    fn new() -> Self {
        Self {
            name: "My Form".to_string(),
        }
    }
}

impl Render for MyView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .child(format!("Form: {}", self.name))
            .child(Button::new("submit-btn").label("Submit").primary())
    }
}
