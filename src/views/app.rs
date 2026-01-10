use gpui::{prelude::FluentBuilder, *};
use gpui_component::{link::Link, sidebar::*, *};
use strum::IntoEnumIterator;

use super::{AppTitleBar, MainPage};
use crate::models::Components;
use crate::sidebar::CustomSidebarToggleButton;

pub struct MyApp {
    pub gpui_component_version: &'static str,
    pub sidebar_collapsed: bool,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            gpui_component_version: "0.5.0",
            sidebar_collapsed: false,
        }
    }

    fn render_sidebar(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        Sidebar::new(Side::Left)
            .w(px(200.))
            .collapsed(self.sidebar_collapsed)
            .header(
                SidebarHeader::new()
                    .pt_0()
                    .pb_0()
                    .child(h_flex().when_else(
                        !self.sidebar_collapsed,
                        |this| {
                            this.child(
                                SidebarMenuItem::new("Home")
                                    .active(true)
                                    .on_click(|_, _, _| println!("Home clicked")),
                            )
                        },
                        |this| this.hidden(),
                    ))
                    .child(
                        CustomSidebarToggleButton::left()
                            .collapsed(self.sidebar_collapsed)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.sidebar_collapsed = !this.sidebar_collapsed;
                                cx.notify();
                            })),
                    ),
            )
            .child(
                SidebarMenu::new().child(
                    SidebarMenuItem::new("Dashboard")
                        .icon(IconName::LayoutDashboard)
                        .active(true)
                        .children(Components::iter().map(|component| {
                            SidebarMenuItem::new(component.to_string())
                                .active(true)
                                .on_click(cx.listener(move |_, _, _, _| {
                                    component.on_click();
                                }))
                        }))
                        .default_open(true),
                ),
            )
            .footer(
                SidebarFooter::new().pt_0().pb_0().child(
                    h_flex()
                        .w_full()
                        .items_center()
                        .justify_center()
                        .text_xs()
                        .when(!self.sidebar_collapsed, |this| {
                            this.child(
                                Link::new("gpui-component")
                                    .href("https://longbridge.github.io/gpui-component/")
                                    .child(format!(
                                        "GPUI Component v{}",
                                        self.gpui_component_version
                                    )),
                            )
                        }),
                ),
            )
    }
}

impl Render for MyApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(AppTitleBar::new())
            .child(
                div()
                    .h_flex()
                    .w_full()
                    .h(window.window_bounds().get_bounds().size.height - TITLE_BAR_HEIGHT)
                    .child(self.render_sidebar(cx))
                    .child(MainPage::new()),
            )
            .children(Root::render_dialog_layer(window, cx))
            .children(Root::render_sheet_layer(window, cx))
            .children(Root::render_notification_layer(window, cx))
    }
}
