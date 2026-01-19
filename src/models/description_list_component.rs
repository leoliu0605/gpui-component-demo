use gpui::*;
use gpui_component::description_list::{DescriptionItem, DescriptionList};
use gpui_component::text::TextView;
use gpui_component::*;

use crate::models::{ComponentMeta, subtitle};

pub struct DescriptionListComponentView;

impl ComponentMeta for DescriptionListComponentView {
    const DESCRIPTION: &'static str = "A versatile component for displaying key-value pairs in a structured, organized layout. \nSupports both horizontal and vertical layouts, multiple columns, borders, and different sizes. \nPerfect for showing detailed information like metadata, specifications, or summary data.";
    const LINK: &'static str =
        "https://longbridge.github.io/gpui-component/docs/components/description-list";
}

impl Render for DescriptionListComponentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Basic Description List"))
            .child(self.basic_description_list())
            .child(subtitle("Using DescriptionItem Builder"))
            .child(self.using_description_item_builder())
            .child(subtitle("Different Layouts"))
            .child(self.different_layouts())
            .child(subtitle("Multiple Columns with Spans"))
            .child(self.multiple_columns_with_spans())
            .child(subtitle("With Dividers"))
            .child(self.with_dividers())
            .child(subtitle("Different Sizes"))
            .child(self.different_sizes())
            .child(subtitle("Without Borders"))
            .child(self.without_borders())
            .child(subtitle("Custom Label Width (Horizontal Layout)"))
            .child(self.custom_label_width())
            .child(subtitle("Rich Content with Custom Elements"))
            .child(self.rich_content_with_custom_elements(_window, _cx))
            .child(subtitle("Complex Example with Mixed Content"))
            .child(self.complex_example_with_mixed_content())
            .child(subtitle("User Profile Information"))
            .child(self.user_profile_information())
            .child(subtitle("System Information"))
            .child(self.system_information())
            .child(subtitle("Product Specifications"))
            .child(self.product_specifications())
            .child(subtitle("Configuration Settings"))
            .child(self.configuration_settings())
    }
}

impl DescriptionListComponentView {
    /// Example code for the Description List component

    fn basic_description_list(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                DescriptionList::new()
                    .item("Name", "GPUI Component", 1)
                    .item("Version", "0.1.0", 1)
                    .item("License", "Apache-2.0", 1),
            )
            .into_any_element()
    }

    fn using_description_item_builder(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                DescriptionList::new().children([
                    DescriptionItem::new("Name").value("GPUI Component"),
                    DescriptionItem::new("Description")
                        .value("UI components for building desktop applications"),
                    DescriptionItem::new("Version").value("0.1.0"),
                ]),
            )
            .into_any_element()
    }

    fn different_layouts(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                // Horizontal layout (default)
                DescriptionList::horizontal()
                    .item("Platform", "macOS, Windows, Linux", 1)
                    .item(
                        "Repository",
                        "https://github.com/longbridge/gpui-component",
                        1,
                    ),
            )
            .child(
                // Vertical layout
                DescriptionList::vertical()
                    .item("Name", "GPUI Component", 1)
                    .item("Description", "A comprehensive UI component library", 1),
            )
            .into_any_element()
    }

    fn multiple_columns_with_spans(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                DescriptionList::new()
                    .columns(3)
                    .child(DescriptionItem::new("Name").value("GPUI Component").span(1))
                    .children([
                        DescriptionItem::new("Version").value("0.1.0").span(1),
                        DescriptionItem::new("License").value("Apache-2.0").span(1),
                        DescriptionItem::new("Description")
                            .value("Full-featured UI components for desktop applications")
                            .span(3), // Spans all 3 columns
                        DescriptionItem::new("Repository")
                            .value("https://github.com/longbridge/gpui-component")
                            .span(2), // Spans 2 columns
                    ]),
            )
            .into_any_element()
    }

    fn with_dividers(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                DescriptionList::new()
                    .item("Name", "GPUI Component", 1)
                    .item("Version", "0.1.0", 1)
                    .divider() // Add a visual separator
                    .item("Author", "Longbridge", 1)
                    .item("License", "Apache-2.0", 1),
            )
            .into_any_element()
    }

    fn different_sizes(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                // Large size
                DescriptionList::new()
                    .large()
                    .item("Title", "Large Description List", 1),
            )
            .child(
                // Medium size (default)
                DescriptionList::new().item("Title", "Medium Description List", 1),
            )
            .child(
                // Small size
                DescriptionList::new()
                    .small()
                    .item("Title", "Small Description List", 1),
            )
            .into_any_element()
    }

    fn without_borders(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                DescriptionList::new()
                    .bordered(false) // Remove borders for a cleaner look
                    .item("Name", "GPUI Component", 1)
                    .item("Type", "UI Library", 1),
            )
            .into_any_element()
    }

    fn custom_label_width(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                DescriptionList::horizontal()
                    .label_width(px(200.0)) // Set custom label width
                    .item("Very Long Label Name", "Short Value", 1)
                    .item("Short", "Very long value that needs more space", 1),
            )
            .into_any_element()
    }

    fn rich_content_with_custom_elements(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                DescriptionList::new().columns(2).children([
                    DescriptionItem::new("Name").value("GPUI Component"),
                    DescriptionItem::new("Description").value(
                        TextView::markdown(
                            0,
                            "UI components for building **fantastic** desktop applications.",
                            window,
                            cx,
                        )
                        .into_any_element(),
                    ),
                ]),
            )
            .into_any_element()
    }

    fn complex_example_with_mixed_content(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                DescriptionList::new()
                    .columns(3)
                    .label_width(px(150.0))
                    .children([
                        DescriptionItem::new("Project Name").value("GPUI Component").span(1),
                        DescriptionItem::new("Version").value("0.1.0").span(1),
                        DescriptionItem::new("Status").value("Active").span(1),

                        DescriptionItem::Divider, // Full-width divider

                        DescriptionItem::new("Description").value(
                            "A comprehensive UI component library for building desktop applications with GPUI"
                        ).span(3),

                        DescriptionItem::new("Repository").value(
                            "https://github.com/longbridge/gpui-component"
                        ).span(2),
                        DescriptionItem::new("License").value("Apache-2.0").span(1),

                        DescriptionItem::new("Platforms").value("macOS, Windows, Linux").span(2),
                        DescriptionItem::new("Language").value("Rust").span(1),
                    ]),
            )
            .into_any_element()
    }

    fn user_profile_information(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                DescriptionList::new()
                    .columns(2)
                    .bordered(true)
                    .children([
                        DescriptionItem::new("Full Name").value("John Doe"),
                        DescriptionItem::new("Email").value("john@example.com"),
                        DescriptionItem::new("Phone").value("+1 (555) 123-4567"),
                        DescriptionItem::new("Department").value("Engineering"),
                        DescriptionItem::Divider,
                        DescriptionItem::new("Bio").value(
                            "Senior software engineer with 10+ years of experience in Rust and system programming."
                        ).span(2),
                    ]),
            )
            .into_any_element()
    }

    fn system_information(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                DescriptionList::vertical()
                    .small()
                    .bordered(false)
                    .children([
                        DescriptionItem::new("Operating System").value("macOS 14.0"),
                        DescriptionItem::new("Architecture").value("Apple Silicon (M2)"),
                        DescriptionItem::new("Memory").value("16 GB"),
                        DescriptionItem::new("Storage").value("512 GB SSD"),
                        DescriptionItem::new("GPU").value("Apple M2 10-core GPU"),
                    ]),
            )
            .into_any_element()
    }

    fn product_specifications(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                DescriptionList::new().columns(3).large().children([
                    DescriptionItem::new("Model").value("MacBook Pro").span(1),
                    DescriptionItem::new("Year").value("2023").span(1),
                    DescriptionItem::new("Screen Size").value("14-inch").span(1),
                    DescriptionItem::new("Processor")
                        .value("Apple M2 Pro")
                        .span(2),
                    DescriptionItem::new("Base Price").value("$1,999").span(1),
                    DescriptionItem::Divider,
                    DescriptionItem::new("Key Features")
                        .value(
                            "Liquid Retina XDR display, ProMotion technology, P3 wide color gamut",
                        )
                        .span(3),
                ]),
            )
            .into_any_element()
    }

    fn configuration_settings(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                DescriptionList::horizontal()
                    .label_width(px(180.0))
                    .bordered(false)
                    .children([
                        DescriptionItem::new("Theme").value("Dark Mode"),
                        DescriptionItem::new("Font Size").value("14px"),
                        DescriptionItem::new("Auto Save").value("Enabled"),
                        DescriptionItem::new("Backup Frequency").value("Every 30 minutes"),
                        DescriptionItem::new("Language").value("English (US)"),
                    ]),
            )
            .into_any_element()
    }
}
