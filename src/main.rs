use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::sidebar::*;
use gpui_component::*;
use gpui_component_assets::Assets;

actions!(system_monitor, [Quit]);

pub struct MyApp {
    sidebar_collapsed: bool,
}

impl MyApp {
    fn new() -> Self {
        Self {
            sidebar_collapsed: false,
        }
    }
}

impl Render for MyApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(
                // Render custom title bar on top of Root view.
                TitleBar::new().child(
                    h_flex()
                        .w_full()
                        .justify_center()
                        .pr_20() // Add right padding to center the title correctly. See https://longbridge.github.io/gpui-component/docs/components/title-bar#macos
                        .child("GPUI Component Demo"),
                ),
            )
            .child(
                div()
                    .h_flex()
                    .size_full()
                    .child(
                        Sidebar::new(Side::Left)
                            .w(px(200.))
                            .collapsed(self.sidebar_collapsed)
                            .collapsible(true)
                            .header(
                                SidebarHeader::new()
                                    .child(
                                        h_flex().when(!self.sidebar_collapsed, |this| {
                                            this.child("Home")
                                        }),
                                    )
                                    .child(
                                        SidebarToggleButton::left()
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
                                        .children([
                                            SidebarMenuItem::new("Accordion")
                                                .active(true)
                                                .on_click(|_, _, _| {
                                                    println!("Open Accordion Page")
                                                }),
                                            SidebarMenuItem::new("Alert")
                                                .active(true)
                                                .on_click(|_, _, _| println!("Open Alert Page")),
                                            SidebarMenuItem::new("Avatar")
                                                .active(true)
                                                .on_click(|_, _, _| println!("Open Avatar Page")),
                                        ])
                                        .default_open(true),
                                ),
                            )
                            .footer(SidebarFooter::new().child(h_flex().child("child"))),
                    )
                    .child(
                        div()
                            .v_flex()
                            .size_full()
                            .items_center()
                            .justify_center()
                            .child("Main Content Area"),
                    ),
            )
            .children(Root::render_dialog_layer(window, cx))
            .children(Root::render_sheet_layer(window, cx))
            .children(Root::render_notification_layer(window, cx))
    }
}

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        cx.bind_keys([
            #[cfg(target_os = "macos")]
            KeyBinding::new("cmd-q", Quit, None),
        ]);

        // Handle the Quit action
        cx.on_action(|_: &Quit, cx: &mut App| {
            cx.quit();
        });

        let window_options = WindowOptions {
            titlebar: Some(TitleBar::title_bar_options()),
            window_bounds: Some(WindowBounds::centered(size(px(800.), px(600.)), cx)),
            ..Default::default()
        };

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                window.activate_window();

                Theme::change(ThemeMode::Dark, Some(window), cx);

                let view = cx.new(|_| MyApp::new());
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
