use gpui::{prelude::FluentBuilder, *};
use gpui_component::{link::Link, sidebar::*, *};
use strum::IntoEnumIterator;

use super::AppTitleBar;
use crate::models::Components;
use crate::sidebar::CustomSidebarToggleButton;
use crate::views::MainPage;

pub struct MyApp {
    pub gpui_component_version: &'static str,
    pub sidebar_collapsed: bool,
    pub main_page: Entity<MainPage>,
}

impl MyApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            gpui_component_version: "0.5.0",
            sidebar_collapsed: false,
            main_page: cx.new(|cx| MainPage::new(cx)),
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
                            let main_page = self.main_page.clone();
                            this.child(SidebarMenuItem::new("Home").active(true).on_click(
                                cx.listener(move |_this, _, _, cx| {
                                    main_page.update(cx, |page, cx| {
                                        page.show_welcome(cx);
                                    });
                                }),
                            ))
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
                            let main_page = self.main_page.clone();
                            SidebarMenuItem::new(component.to_string())
                                .active(true)
                                .on_click(cx.listener(move |_this, _, _, cx| {
                                    main_page.update(cx, |page, cx| {
                                        page.show_component(component, cx);
                                    });
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
                    .child(self.main_page.clone()),
            )
            .children(Root::render_dialog_layer(window, cx))
            .children(Root::render_sheet_layer(window, cx))
            .children(Root::render_notification_layer(window, cx))
    }
}
