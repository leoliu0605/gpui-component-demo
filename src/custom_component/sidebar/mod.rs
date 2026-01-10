use gpui::{prelude::FluentBuilder, *};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::{Icon, IconName, Side, Sizable};
use std::rc::Rc;

/// A custom SidebarToggleButton with configurable icon size
#[derive(IntoElement)]
pub struct CustomSidebarToggleButton {
    btn: Button,
    collapsed: bool,
    side: Side,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl CustomSidebarToggleButton {
    pub fn new(side: Side) -> Self {
        Self {
            btn: Button::new("collapse").ghost().large(),
            collapsed: false,
            side,
            on_click: None,
        }
    }

    pub fn left() -> Self {
        Self::new(Side::Left)
    }

    #[allow(dead_code)]
    pub fn right() -> Self {
        Self::new(Side::Right)
    }

    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    pub fn on_click(
        mut self,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(on_click));
        self
    }
}

impl RenderOnce for CustomSidebarToggleButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let on_click = self.on_click.clone();

        let icon = if self.collapsed {
            if self.side.is_left() {
                IconName::PanelLeftOpen
            } else {
                IconName::PanelRightOpen
            }
        } else {
            if self.side.is_left() {
                IconName::PanelLeftClose
            } else {
                IconName::PanelRightClose
            }
        };

        self.btn
            .when_some(on_click, |this, on_click| {
                this.on_click(move |ev, window, cx| {
                    on_click(ev, window, cx);
                })
            })
            .icon(Icon::new(icon))
            .when(self.collapsed, |this| this.w_full().justify_center())
    }
}
