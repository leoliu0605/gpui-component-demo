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
            .child(
                Accordion::new("my-accordion")
                    .item(|item| item.title("Section 1").child("Content for section 1"))
                    .item(|item| item.title("Section 2").child("Content for section 2"))
                    .item(|item| item.title("Section 3").child("Content for section 3")),
            )
            .into_any_element()
    }

    fn description(&self) -> &'static str {
        "A collapsible content container with expandable sections"
    }

    fn link(&self) -> &'static str {
        "https://longbridge.github.io/gpui-component/docs/components/accordion"
    }
}
