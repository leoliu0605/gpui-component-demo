use gpui::*;
use gpui_component::accordion::Accordion;
use gpui_component::*;

use crate::models::{ComponentMeta, subtitle};

/// The actual View that renders accordion examples
pub struct AccordionComponentView;

impl ComponentMeta for AccordionComponentView {
    const DESCRIPTION: &'static str = "An accordion component that allows users to show and hide sections of content. \nIt uses collapse functionality internally to create collapsible panels.";
    const LINK: &'static str =
        "https://longbridge.github.io/gpui-component/docs/components/accordion";
}

impl Render for AccordionComponentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Basic Accordion"))
            .child(self.basic_accordion())
            .child(subtitle("Multiple Open Items"))
            .child(self.multiple_open_items_accordion())
            .child(subtitle("With Borders"))
            .child(self.with_borders_accordion())
            .child(subtitle("Different Sizes"))
            .child(self.different_sizes_accordion())
            .child(subtitle("Handle Toggle Events"))
            .child(self.handle_toggle_events_accordion())
            .child(subtitle("Disabled State"))
            .child(self.disabled_state_accordion())
            .child(subtitle("With Custom Icons"))
            .child(self.with_custom_icons_accordion())
            .child(subtitle("Nested Accordions"))
            .child(self.nested_accordions_accordion())
    }
}

/// FIXME: I think there are some bugs in Accordion component, it cannot open/close items when clicked.
///        Maybe need to fix it in future versions.
impl AccordionComponentView {
    /// Example code for the Accordion component

    fn basic_accordion(&self) -> AnyElement {
        Accordion::new("my-accordion")
            .item(|item| {
                item.title("Section 1")
                    .open(true)
                    .child("Content for section 1")
            })
            .item(|item| {
                item.title("Section 2")
                    .open(true)
                    .child("Content for section 2")
            })
            .item(|item| {
                item.title("Section 3")
                    .open(true)
                    .child("Content for section 3")
            })
            .into_any_element()
    }

    fn multiple_open_items_accordion(&self) -> AnyElement {
        Accordion::new("my-accordion")
            .multiple(true)
            .item(|item| item.title("Section 1").open(true).child("Content 1"))
            .item(|item| item.title("Section 2").open(true).child("Content 2"))
            .into_any_element()
    }

    fn with_borders_accordion(&self) -> AnyElement {
        Accordion::new("my-accordion")
            .bordered(true)
            .item(|item| item.title("Section 1").open(true).child("Content 1"))
            .into_any_element()
    }

    fn different_sizes_accordion(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                Accordion::new("my-accordion")
                    .small()
                    .item(|item| item.title("Small Section").open(true).child("Content")),
            )
            .child(
                Accordion::new("my-accordion")
                    .large()
                    .item(|item| item.title("Large Section").open(true).child("Content")),
            )
            .into_any_element()
    }

    fn handle_toggle_events_accordion(&self) -> AnyElement {
        Accordion::new("my-accordion")
            .on_toggle_click(|open_indices, _window, _cx| {
                println!("Open items: {:?}", open_indices);
            })
            .item(|item| item.title("Section 1").child("Content 1"))
            .into_any_element()
    }

    fn disabled_state_accordion(&self) -> AnyElement {
        Accordion::new("my-accordion")
            .disabled(true)
            .item(|item| item.title("Disabled Section").child("Content"))
            .into_any_element()
    }

    fn with_custom_icons_accordion(&self) -> AnyElement {
        Accordion::new("my-accordion")
            .item(|item| {
                item.title(
                    h_flex()
                        .gap_2()
                        .child(Icon::new(IconName::Settings))
                        .child("Settings"),
                )
                .child("Settings content here")
            })
            .into_any_element()
    }

    fn nested_accordions_accordion(&self) -> AnyElement {
        Accordion::new("outer")
            .item(|item| {
                item.title("Parent Section").open(true).child(
                    Accordion::new("inner")
                        .item(|item| item.title("Child 1").open(true).child("Content"))
                        .item(|item| item.title("Child 2").open(true).child("Content")),
                )
            })
            .into_any_element()
    }
}
