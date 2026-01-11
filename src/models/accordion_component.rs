use gpui::*;
use gpui_component::accordion::Accordion;
use gpui_component::*;

use crate::models::ComponentRenderer;

pub struct AccordionComponent;

impl ComponentRenderer for AccordionComponent {
    fn show(&self, _window: &mut Window, _cx: &mut App) -> AnyElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(self.add_subtitle("Basic Accordion"))
            .child(self.basic_accordion())
            .child(self.add_subtitle("Multiple Open Items"))
            .child(self.multiple_open_items_accordion())
            .child(self.add_subtitle("With Borders"))
            .child(self.with_borders_accordion())
            .child(self.add_subtitle("Different Sizes"))
            .child(self.different_sizes_small_accordion())
            .child(self.different_sizes_large_accordion())
            .child(self.add_subtitle("Handle Toggle Events"))
            .child(self.handle_toggle_events_accordion())
            .child(self.add_subtitle("Disabled State"))
            .child(self.disabled_state_accordion())
            .child(self.add_subtitle("With Custom Icons"))
            .child(self.with_custom_icons_accordion())
            .child(self.add_subtitle("Nested Accordions"))
            .child(self.nested_accordions_accordion())
            .into_any_element()
    }

    fn description(&self) -> &'static str {
        "A collapsible content container with expandable sections"
    }

    fn link(&self) -> &'static str {
        "https://longbridge.github.io/gpui-component/docs/components/accordion"
    }
}

impl AccordionComponent {
    /// Example code for the Accordion component

    fn basic_accordion(&self) -> Accordion {
        Accordion::new("my-accordion")
            .item(|item| item.title("Section 1").open(true).child("Content for section 1"))
            .item(|item| item.title("Section 2").open(true).child("Content for section 2"))
            .item(|item| item.title("Section 3").open(true).child("Content for section 3"))
    }

    fn multiple_open_items_accordion(&self) -> Accordion {
        Accordion::new("my-accordion")
            .multiple(true)
            .item(|item| item.title("Section 1").open(true).child("Content 1"))
            .item(|item| item.title("Section 2").open(true).child("Content 2"))
    }

    fn with_borders_accordion(&self) -> Accordion {
        Accordion::new("my-accordion")
            .bordered(true)
            .item(|item| item.title("Section 1").open(true).child("Content 1"))
    }

    fn different_sizes_small_accordion(&self) -> Accordion {
        Accordion::new("my-accordion")
            .small()
            .item(|item| item.title("Small Section").open(true).child("Content"))
    }

    fn different_sizes_large_accordion(&self) -> Accordion {
        Accordion::new("my-accordion")
            .large()
            .item(|item| item.title("Large Section").open(true).child("Content"))
    }

    fn handle_toggle_events_accordion(&self) -> Accordion {
        Accordion::new("my-accordion")
            .on_toggle_click(|open_indices, _window, _cx| {
                println!("Open items: {:?}", open_indices);
            })
            .item(|item| item.title("Section 1").child("Content 1"))
    }

    fn disabled_state_accordion(&self) -> Accordion {
        Accordion::new("my-accordion")
            .disabled(true)
            .item(|item| item.title("Disabled Section").child("Content"))
    }

    fn with_custom_icons_accordion(&self) -> Accordion {
        Accordion::new("my-accordion").item(|item| {
            item.title(
                h_flex()
                    .gap_2()
                    .child(Icon::new(IconName::Settings))
                    .child("Settings"),
            )
            .child("Settings content here")
        })
    }

    fn nested_accordions_accordion(&self) -> Accordion {
        Accordion::new("outer").item(|item| {
            item.title("Parent Section").child(
                Accordion::new("inner")
                    .item(|item| item.title("Child 1").child("Content"))
                    .item(|item| item.title("Child 2").child("Content")),
            )
        })
    }
}
