use gpui::*;
use gpui_component::checkbox::Checkbox;
use gpui_component::skeleton::Skeleton;
use gpui_component::*;

use crate::models::{ComponentMeta, subtitle};

pub struct SkeletonComponentView {
    is_loading: bool,
}

impl SkeletonComponentView {
    pub fn new() -> Self {
        Self { is_loading: true }
    }
}

impl ComponentMeta for SkeletonComponentView {
    const DESCRIPTION: &'static str = "The Skeleton component displays animated placeholder content while actual content is loading. \nIt provides visual feedback to users that content is being loaded and helps maintain layout structure during loading states.";
    const LINK: &'static str =
        "https://longbridge.github.io/gpui-component/docs/components/skeleton";
}

impl Render for SkeletonComponentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Basic Skeleton"))
            .child(self.basic_skeleton())
            .child(subtitle("Text Line Skeleton"))
            .child(self.text_line_skeleton())
            .child(subtitle("Circle Skeleton"))
            .child(self.circle_skeleton())
            .child(subtitle("Rectangle Skeleton"))
            .child(self.rectangle_skeleton())
            .child(subtitle("Different Shapes"))
            .child(self.different_shapes())
            .child(subtitle("Secondary Variant"))
            .child(self.secondary_variant())
            .child(subtitle("Sizes"))
            .child(self.sizes())
            .child(subtitle("Loading Profile Card"))
            .child(self.loading_profile_card(_cx))
            .child(subtitle("Loading Article List"))
            .child(self.loading_article_list())
            .child(subtitle("Loading Table Rows"))
            .child(self.loading_table_rows(_cx))
            .child(subtitle("Loading Button States"))
            .child(self.loading_button_states())
            .child(subtitle("Loading Form Fields"))
            .child(self.loading_form_fields())
            .child(subtitle("Conditional Loading"))
            .child(self.conditional_loading(_cx))
    }
}

impl SkeletonComponentView {
    /// Example code for the Skeleton component

    fn basic_skeleton(&self) -> AnyElement {
        Skeleton::new().into_any_element()
    }

    fn text_line_skeleton(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                // Single line of text
                Skeleton::new().w(px(250.)).h_4().rounded_md(),
            )
            .child("...")
            .child(
                // Multiple text lines
                v_flex()
                    .gap_2()
                    .child(Skeleton::new().w(px(250.)).h_4().rounded_md())
                    .child(Skeleton::new().w(px(200.)).h_4().rounded_md())
                    .child(Skeleton::new().w(px(180.)).h_4().rounded_md()),
            )
            .into_any_element()
    }

    fn circle_skeleton(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                // Avatar placeholder
                Skeleton::new().size_12().rounded_full(),
            )
            .child(
                // Profile picture placeholder
                Skeleton::new().w(px(64.)).h(px(64.)).rounded_full(),
            )
            .into_any_element()
    }

    fn rectangle_skeleton(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                // Card image placeholder
                Skeleton::new().w(px(250.)).h(px(125.)).rounded_md(),
            )
            .child(
                // Button placeholder
                Skeleton::new().w(px(120.)).h(px(40.)).rounded_md(),
            )
            .into_any_element()
    }

    fn different_shapes(&self) -> AnyElement {
        h_flex()
            .gap_4()
            .child(
                // Text content
                Skeleton::new().w(px(200.)).h_4().rounded_sm(),
            )
            .child(
                // Square image
                Skeleton::new().size_20().rounded_md(),
            )
            .child(
                // Wide banner
                Skeleton::new().w_full().h(px(200.)).rounded_lg(),
            )
            .child(
                // Small icon
                Skeleton::new().size_6().rounded_md(),
            )
            .into_any_element()
    }

    fn secondary_variant(&self) -> AnyElement {
        // Use secondary color (more subtle)
        Skeleton::new()
            .secondary()
            .w(px(200.))
            .h_4()
            .rounded_md()
            .into_any_element()
    }

    fn sizes(&self) -> AnyElement {
        h_flex()
            .gap_4()
            // Height utilities
            .child(
                Skeleton::new().h_3(), // 12px height
            )
            .child(
                Skeleton::new().h_4(), // 16px height
            )
            .child(
                Skeleton::new().h_5(), // 20px height
            )
            .child(
                Skeleton::new().h_6(), // 24px height
            )
            // Width utilities
            .child(
                Skeleton::new().w(px(100.)), // 100px width
            )
            .child(
                Skeleton::new().w(px(200.)), // 200px width
            )
            .child(
                Skeleton::new().w_full(), // Full width
            )
            .child(
                Skeleton::new().w_1_2(), // 50% width
            )
            // Square sizes
            .child(
                Skeleton::new().size_4(), // 16x16px
            )
            .child(
                Skeleton::new().size_8(), // 32x32px
            )
            .child(
                Skeleton::new().size_12(), // 48x48px
            )
            .child(
                Skeleton::new().size_16(), // 64x64px
            )
            .into_any_element()
    }

    fn loading_profile_card(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .gap_4()
            .p_4()
            .border_1()
            .border_color(cx.theme().border)
            .rounded_lg()
            .child(
                h_flex()
                    .gap_3()
                    .items_center()
                    .child(Skeleton::new().size_12().rounded_full()) // Avatar
                    .child(
                        v_flex()
                            .gap_2()
                            .child(Skeleton::new().w(px(120.)).h_4().rounded_md()) // Name
                            .child(Skeleton::new().w(px(100.)).h_3().rounded_md()), // Email
                    ),
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(Skeleton::new().w_full().h_4().rounded_md()) // Bio line 1
                    .child(Skeleton::new().w(px(200.)).h_4().rounded_md()), // Bio line 2
            )
            .into_any_element()
    }

    fn loading_article_list(&self) -> AnyElement {
        v_flex()
            .gap_6()
            .children((0..3).map(|_| {
                h_flex()
                    .gap_4()
                    .child(Skeleton::new().w(px(120.)).h(px(80.)).rounded_md()) // Thumbnail
                    .child(
                        v_flex()
                            .gap_2()
                            .flex_1()
                            .child(Skeleton::new().w_full().h_5().rounded_md()) // Title
                            .child(Skeleton::new().w(px(300.)).h_4().rounded_md()) // Excerpt line 1
                            .child(Skeleton::new().w(px(250.)).h_4().rounded_md()) // Excerpt line 2
                            .child(Skeleton::new().w(px(100.)).h_3().rounded_md()), // Date
                    )
            }))
            .into_any_element()
    }

    fn loading_table_rows(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .gap_2()
            .children((0..5).map(|_| {
                h_flex()
                    .gap_4()
                    .p_3()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(Skeleton::new().size_8().rounded_full()) // Status indicator
                    .child(Skeleton::new().w(px(150.)).h_4().rounded_md()) // Name
                    .child(Skeleton::new().w(px(200.)).h_4().rounded_md()) // Email
                    .child(Skeleton::new().w(px(80.)).h_4().rounded_md()) // Role
                    .child(Skeleton::new().w(px(60.)).h_4().rounded_md()) // Actions
            }))
            .into_any_element()
    }

    fn loading_button_states(&self) -> AnyElement {
        h_flex()
            .gap_3()
            .child(Skeleton::new().w(px(80.)).h(px(36.)).rounded_md()) // Primary button
            .child(Skeleton::new().w(px(70.)).h(px(36.)).rounded_md()) // Secondary button
            .child(Skeleton::new().size_9().rounded_md()) // Icon button
            .into_any_element()
    }

    fn loading_form_fields(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                v_flex()
                    .gap_1()
                    .child(Skeleton::new().w(px(60.)).h_4().rounded_md()) // Label
                    .child(Skeleton::new().w_full().h(px(40.)).rounded_md()), // Input
            )
            .child(
                v_flex()
                    .gap_1()
                    .child(Skeleton::new().w(px(80.)).h_4().rounded_md()) // Label
                    .child(Skeleton::new().w_full().h(px(120.)).rounded_md()), // Textarea
            )
            .into_any_element()
    }

    fn conditional_loading(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                Checkbox::new("loading-toggle")
                    .label("Is Loading")
                    .checked(self.is_loading)
                    .on_click(cx.listener(|this, checked, _, cx| {
                        this.is_loading = *checked;
                        cx.notify();
                    })),
            )
            .child(if self.is_loading {
                Skeleton::new()
                    .w(px(200.))
                    .h_4()
                    .rounded_md()
                    .into_any_element()
            } else {
                div().child("Actual content here").into_any_element()
            })
            .into_any_element()
    }
}
