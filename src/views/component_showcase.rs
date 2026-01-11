use crate::models::Components;
use gpui::*;
use gpui_component::{link::Link, scroll::ScrollableElement};

pub struct ComponentShowcase {
    pub component: Components,
}

impl ComponentShowcase {
    pub fn new(component: Components) -> Self {
        Self { component }
    }

    pub fn set_component(&mut self, component: Components, cx: &mut Context<Self>) {
        self.component = component;
        cx.notify();
    }
}

impl Render for ComponentShowcase {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let renderer = self.component.get_renderer();
        let title = self.component.to_string();
        let link = renderer.link();

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
                            .child(renderer.description()),
                    ),
            )
            .child(div().w_full().h(px(1.)).bg(rgb(0xe0e0e0)))
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .p_6()
                    .bg(rgb(0xf5f5f5))
                    .rounded_lg()
                    .border_1()
                    .border_color(rgb(0xe0e0e0))
                    .overflow_y_scrollbar()
                    .child(renderer.show(window, cx)),
            )
    }
}
