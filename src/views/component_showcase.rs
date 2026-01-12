use crate::models::Components;
use gpui::*;
use gpui_component::{divider::Divider, link::Link, scroll::ScrollableElement};

pub struct ComponentShowcase {
    pub component: Components,
    current_view: Option<AnyView>,
}

impl ComponentShowcase {
    pub fn new(component: Components) -> Self {
        Self {
            component,
            current_view: None,
        }
    }

    pub fn set_component(
        &mut self,
        component: Components,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.component = component;
        // Create new view for the component
        self.current_view = Some(self.component.create_view(window, cx));
        cx.notify();
    }
}

impl Render for ComponentShowcase {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title = self.component.to_string();
        let description = self.component.description();
        let link = self.component.link();

        // Create view if not exists
        let component_view = if let Some(view) = &self.current_view {
            view.clone()
        } else {
            let view = self.component.create_view(window, cx);
            self.current_view = Some(view.clone());
            view
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .p_8()
            .gap_6()
            .overflow_hidden()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        Link::new("component-title")
                            .href(link)
                            .child(div().text_3xl().font_weight(FontWeight::BOLD).child(title)),
                    )
                    .child(
                        div()
                            .text_base()
                            .text_color(rgb(0x666666))
                            .child(description),
                    ),
            )
            .child(Divider::horizontal())
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scrollbar()
                    .child(component_view),
            )
    }
}
