use crate::models::Components;
use crate::views::ComponentShowcase;
use gpui::*;

pub struct MainPage {
    showcase: Option<Entity<ComponentShowcase>>,
}

impl MainPage {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self { showcase: None }
    }

    pub fn show_component(&mut self, component: Components, cx: &mut Context<Self>) {
        if let Some(showcase) = &self.showcase {
            showcase.update(cx, |showcase, cx| {
                showcase.set_component(component, cx);
            });
        } else {
            self.showcase = Some(cx.new(|_cx| ComponentShowcase::new(component)));
        }
        cx.notify();
    }

    pub fn show_welcome(&mut self, cx: &mut Context<Self>) {
        self.showcase = None;
        cx.notify();
    }
}

impl Render for MainPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        match &self.showcase {
            Some(showcase) => div().size_full().child(showcase.clone()),
            None => div()
                .flex()
                .flex_col()
                .size_full()
                .items_center()
                .justify_center()
                .gap_4()
                .child(
                    div()
                        .text_3xl()
                        .font_weight(FontWeight::BOLD)
                        .child("Welcome to GPUI Component Demo"),
                )
                .child(
                    div()
                        .text_base()
                        .text_color(rgb(0x666666))
                        .child("Select a component from the sidebar to view its demo"),
                ),
        }
    }
}
