use gpui::*;
use gpui_component::StyledExt;

#[derive(IntoElement)]
pub struct MainPage;

impl MainPage {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for MainPage {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .child("Welcome to GPUI Component Demo!")
    }
}
