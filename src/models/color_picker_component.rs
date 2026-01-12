use gpui::*;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::color_picker::{ColorPicker, ColorPickerEvent, ColorPickerState};
use gpui_component::*;

use crate::models::{ComponentMeta, subtitle};

pub struct ColorPickerComponentView {
    primary_color: Entity<ColorPickerState>,
    secondary_color: Entity<ColorPickerState>,
    accent_color: Entity<ColorPickerState>,

    colors: Vec<Entity<ColorPickerState>>,
}

impl ColorPickerComponentView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let primary_color =
            cx.new(|cx| ColorPickerState::new(window, cx).default_value(cx.theme().primary));

        let secondary_color =
            cx.new(|cx| ColorPickerState::new(window, cx).default_value(cx.theme().secondary));

        let accent_color =
            cx.new(|cx| ColorPickerState::new(window, cx).default_value(cx.theme().accent));

        Self {
            primary_color,
            secondary_color,
            accent_color,
            colors: Vec::new(),
        }
    }

    fn add_color(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let color_picker = cx.new(|cx| ColorPickerState::new(window, cx));

        // Subscribe to color changes
        cx.subscribe(&color_picker, |_this, _, ev, _| match ev {
            ColorPickerEvent::Change(color) => {
                if let Some(_color) = color {
                    // Handle palette preview update
                }
            }
        })
        .detach();

        self.colors.push(color_picker);
        cx.notify();
    }

    fn validate_contrast(&self, color: &Hsla) -> bool {
        let luminance = color.l;
        luminance > 0.2 && luminance < 0.8
    }

    fn apply_color(&self, color: &Hsla) {
        println!("Color applied: {}", color.to_hex());
    }

    fn show_contrast_warning(&self) {
        println!("Warning: Color has low contrast, consider choosing a different color");
    }
}

impl ComponentMeta for ColorPickerComponentView {
    const DESCRIPTION: &'static str = "A versatile color picker component that provides an intuitive interface for color selection. \nFeatures include color palettes, hex input, featured colors, and support for various color formats including RGB, HSL, and hex values with alpha channel support.";
    const LINK: &'static str =
        "https://longbridge.github.io/gpui-component/docs/components/color-picker";
}

impl Render for ColorPickerComponentView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Basic Color Picker"))
            .child(self.basic_color_picker(window, cx))
            .child(subtitle("With Event Handling"))
            .child(self.with_event_handling(window, cx))
            .child(subtitle("Setting Default Color"))
            .child(self.setting_default_color(window, cx))
            .child(subtitle("Different Sizes"))
            .child(self.different_sizes(window, cx))
            .child(subtitle("With Custom Featured Colors"))
            .child(self.with_custom_featured_colors(window, cx))
            .child(subtitle("With Icon Instead of Color Square"))
            .child(self.with_icon_instead_of_color_square(window, cx))
            .child(subtitle("With Label"))
            .child(self.with_label(window, cx))
            .child(subtitle("Custom Anchor Position"))
            .child(self.custom_anchor_position(window, cx))
            .child(subtitle("Color Theme Editor"))
            .child(self.color_theme_editor(window, cx))
            .child(subtitle("Brand Color Selector"))
            .child(self.brand_color_selector(window, cx))
            .child(subtitle("Toolbar Color Picker"))
            .child(self.toolbar_color_picker(window, cx))
            .child(subtitle("Color Palette Builder"))
            .child(self.color_palette_builder(window, cx))
            .child(subtitle("With Color Validation"))
            .child(self.with_color_validation(window, cx))
    }
}

impl ColorPickerComponentView {
    /// Example code for the ColorPicker component

    fn basic_color_picker(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        // Create color picker state
        let color_picker =
            cx.new(|cx| ColorPickerState::new(window, cx).default_value(cx.theme().primary));

        // Create the color picker component
        ColorPicker::new(&color_picker).into_any_element()
    }

    fn with_event_handling(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let color_picker = cx.new(|cx| ColorPickerState::new(window, cx));

        let _subscription = cx.subscribe(&color_picker, |_this, _, ev, _| match ev {
            ColorPickerEvent::Change(color) => {
                if let Some(color) = color {
                    println!("Selected color: {}", color.to_hex());
                    // Handle color change
                }
            }
        });

        ColorPicker::new(&color_picker).into_any_element()
    }

    fn setting_default_color(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        cx.new(
            |cx| ColorPickerState::new(window, cx).default_value(cx.theme().blue), // Set default color
        )
        .into_any_element()
    }

    fn different_sizes(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let color_picker = cx.new(|cx| ColorPickerState::new(window, cx));

        v_flex()
            .gap_4()
            .child(
                // Small color picker
                ColorPicker::new(&color_picker).small(),
            )
            .child(
                // Medium color picker (default)
                ColorPicker::new(&color_picker),
            )
            .child(
                // Large color picker
                ColorPicker::new(&color_picker).large(),
            )
            .child(
                // Extra small color picker
                ColorPicker::new(&color_picker).xsmall(),
            )
            .into_any_element()
    }

    fn with_custom_featured_colors(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let color_picker = cx.new(|cx| ColorPickerState::new(window, cx));

        let featured_colors = vec![
            cx.theme().red,
            cx.theme().green,
            cx.theme().blue,
            cx.theme().yellow,
            // Add your custom colors
        ];

        ColorPicker::new(&color_picker)
            .featured_colors(featured_colors)
            .into_any_element()
    }

    fn with_icon_instead_of_color_square(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let color_picker = cx.new(|cx| ColorPickerState::new(window, cx));

        ColorPicker::new(&color_picker)
            .icon(IconName::Palette)
            .into_any_element()
    }

    fn with_label(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let color_picker = cx.new(|cx| ColorPickerState::new(window, cx));

        ColorPicker::new(&color_picker)
            .label("Background Color")
            .into_any_element()
    }

    fn custom_anchor_position(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let color_picker = cx.new(|cx| ColorPickerState::new(window, cx));

        ColorPicker::new(&color_picker)
            .anchor(Corner::TopRight) // Dropdown opens to top-right
            .into_any_element()
    }

    fn color_theme_editor(&self, _window: &mut Window, _cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child("Primary Color:")
                    .child(ColorPicker::new(&self.primary_color)),
            )
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child("Secondary Color:")
                    .child(ColorPicker::new(&self.secondary_color)),
            )
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child("Accent Color:")
                    .child(ColorPicker::new(&self.accent_color)),
            )
            .into_any_element()
    }

    fn brand_color_selector(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let color_picker = cx.new(|cx| ColorPickerState::new(window, cx));

        let brand_colors = vec![
            Hsla::parse_hex("#FF6B6B").unwrap(), // Brand Red
            Hsla::parse_hex("#4ECDC4").unwrap(), // Brand Teal
            Hsla::parse_hex("#45B7D1").unwrap(), // Brand Blue
            Hsla::parse_hex("#96CEB4").unwrap(), // Brand Green
            Hsla::parse_hex("#FFEAA7").unwrap(), // Brand Yellow
        ];

        ColorPicker::new(&color_picker)
            .featured_colors(brand_colors)
            .label("Brand Color")
            .large()
            .into_any_element()
    }

    fn toolbar_color_picker(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let color_picker = cx.new(|cx| ColorPickerState::new(window, cx));

        ColorPicker::new(&color_picker)
            .icon(IconName::Palette)
            .small()
            .anchor(Corner::BottomLeft)
            .into_any_element()
    }

    fn color_palette_builder(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        h_flex()
            .gap_2()
            .children(
                self.colors
                    .iter()
                    .map(|color_picker| ColorPicker::new(color_picker).small()),
            )
            .child(
                Button::new("add-color")
                    .icon(IconName::Plus)
                    .ghost()
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.add_color(window, cx);
                    })),
            )
            .into_any_element()
    }

    fn with_color_validation(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let color_picker = cx.new(|cx| ColorPickerState::new(window, cx));

        let _subscription = cx.subscribe(&color_picker, |this, _, ev, _| match ev {
            ColorPickerEvent::Change(color) => {
                if let Some(color) = color {
                    // Validate color accessibility
                    if this.validate_contrast(color) {
                        this.apply_color(color);
                    } else {
                        this.show_contrast_warning();
                    }
                }
            }
        });

        ColorPicker::new(&color_picker)
            .label("Pick a color to validate")
            .into_any_element()
    }
}
