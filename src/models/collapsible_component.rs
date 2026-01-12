use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::button::ButtonVariants;
use gpui_component::*;
use gpui_component::{button::Button, collapsible::Collapsible};

use crate::models::{ComponentMeta, subtitle};

pub struct CollapsibleComponentView {
    open: bool,
}

impl CollapsibleComponentView {
    pub fn new() -> Self {
        Self { open: false }
    }
}

impl ComponentMeta for CollapsibleComponentView {
    const DESCRIPTION: &'static str = "An interactive element which expands/collapses.";
    const LINK: &'static str =
        "https://longbridge.github.io/gpui-component/docs/components/collapsible";
}

impl Render for CollapsibleComponentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Basic Use"))
            .child(self.basic_use(_cx))
    }
}

impl CollapsibleComponentView {
    /// Example code for the Collapsible component

    fn basic_use(&self, cx: &mut Context<Self>) -> AnyElement {
        Collapsible::new()
            .max_w_128()
            .gap_1()
            .open(self.open)
            .child(
                "This is a collapsible component. \
        Click the header to expand or collapse the content.",
            )
            .content(
                "This is the full content of the Collapsible component. \
        It is only visible when the component is expanded. \n\
        You can put any content you like here, including text, images, \
        or other UI elements.",
            )
            .child(
                h_flex().justify_center().child(
                    Button::new("toggle1")
                        .icon(IconName::ChevronDown)
                        .label("Show more")
                        .when(self.open, |this| {
                            this.icon(IconName::ChevronUp).label("Show less")
                        })
                        .xsmall()
                        .link()
                        .on_click({
                            cx.listener(move |this, _, _, cx| {
                                this.open = !this.open;
                                cx.notify();
                            })
                        }),
                ),
            )
            .into_any_element()
    }
}
