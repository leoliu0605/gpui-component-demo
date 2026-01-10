use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::link::Link;
use gpui_component::sidebar::*;
use gpui_component::*;
use gpui_component_assets::Assets;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

use gpui_component_demo::sidebar::CustomSidebarToggleButton;

actions!(my_app, [Quit]);

#[derive(Debug, Clone, Copy, Display, EnumString, EnumIter)]
enum Components {
    Accordion,
    Alert,
    Avatar,
    Badge,
    Button,
    Calendar,
    Chart,
    Checkbox,
    Clipboard,
    Collapsible,
    ColorPicker,
    DatePicker,
    DescriptionList,
    Dialog,
    DropdownButton,
    Editor,
    Form,
    GroupBox,
    Icon,
    Image,
    Input,
    Kbd,
    Label,
    List,
    Menu,
    Notification,
    NumberInput,
    OptInput,
    Plot,
    Popover,
    Progress,
    Radio,
    Resizable,
    Select,
    Settings,
    Sheet,
    Side,
    Skeleton,
    Slider,
    Spinner,
    Stepper,
    Switch,
    Table,
    Tabs,
    Tag,
    TitleBar,
    Toggle,
    Tooltip,
    Tree,
    VirtualList,
}

impl Components {
    fn on_click(&self) {
        println!("Open {:?} Page", self);
    }
}

pub struct MyApp {
    gpui_component_version: &'static str,
    sidebar_collapsed: bool,
}

impl MyApp {
    fn new() -> Self {
        Self {
            gpui_component_version: "0.5.0",
            sidebar_collapsed: false,
        }
    }
}

impl Render for MyApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
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
                    .w_full()
                    .h(window.window_bounds().get_bounds().size.height - TITLE_BAR_HEIGHT)
                    .child(
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
                            ),
                    )
                    .child(
                        div()
                            .v_flex()
                            .size_full()
                            .items_center()
                            .justify_center()
                            .child("Welcome to GPUI Component Demo!"),
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
            window_bounds: Some(WindowBounds::centered(size(px(1100.), px(700.)), cx)),
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
