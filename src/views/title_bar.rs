use gpui::*;
use gpui_component::*;

#[derive(IntoElement)]
pub struct AppTitleBar;

impl AppTitleBar {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for AppTitleBar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        TitleBar::new().child(
            h_flex()
                .w_full()
                .justify_center()
                .pr_20()
                .child("GPUI Component Demo"),
        )
    }
}
