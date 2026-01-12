use gpui::{
    AnyElement, AnyView, AppContext, Context, FontWeight, IntoElement, ParentElement, Styled,
    Window, div,
};
use strum_macros::{Display, EnumIter, EnumString};

use super::AccordionComponentView;
use super::AlertComponentView;
use super::AvatarComponentView;
use super::BadgeComponentView;
use super::ButtonComponentView;
use super::CalendarComponentView;
use super::ChartComponentView;
use super::CheckboxComponentView;
use super::ClipboardComponentView;
use super::CollapsibleComponentView;
// use super::ColorPickerComponentView;
// use super::DatePickerComponentView;
// use super::DescriptionListComponentView;
// use super::DialogComponentView;
use super::DropdownButtonComponentView;
// use super::EditorComponentView;
// use super::FormComponentView;
// use super::GroupBoxComponentView;
// use super::IconComponentView;
// use super::ImageComponentView;
// use super::InputComponentView;
// use super::KbdComponentView;
// use super::LabelComponentView;
// use super::ListComponentView;
// use super::MenuComponentView;
// use super::NotificationComponentView;
// use super::NumberInputComponentView;
// use super::OptInputComponentView;
// use super::PlotComponentView;
use super::PopoverComponentView;
// use super::ProgressComponentView;
// use super::RadioComponentView;
// use super::ResizableComponentView;
// use super::SelectComponentView;
// use super::SettingsComponentView;
// use super::SheetComponentView;
// use super::SideComponentView;
use super::SkeletonComponentView;
// use super::SliderComponentView;
// use super::SpinnerComponentView;
// use super::StepperComponentView;
// use super::SwitchComponentView;
// use super::TableComponentView;
// use super::TabsComponentView;
// use super::TagComponentView;
// use super::TitleBarComponentView;
// use super::ToggleComponentView;
// use super::TooltipComponentView;
// use super::TreeComponentView;
// use super::VirtualListComponentView;

/// Trait for component metadata - each component view should implement this
pub trait ComponentMeta {
    const DESCRIPTION: &'static str;
    const LINK: &'static str;
}

/// Helper function to create a subtitle element
pub fn subtitle(text: &str) -> AnyElement {
    div()
        .text_2xl()
        .font_weight(FontWeight::BOLD)
        .child(text.to_string())
        .into_any_element()
}

#[derive(Debug, Clone, Copy, Display, EnumString, EnumIter, PartialEq)]
pub enum Components {
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
    // ColorPicker,
    // DatePicker,
    // DescriptionList,
    // Dialog,
    DropdownButton,
    // Editor,
    // Form,
    // GroupBox,
    // Icon,
    // Image,
    // Input,
    // Kbd,
    // Label,
    // List,
    // Menu,
    // Notification,
    // NumberInput,
    // OptInput,
    // Plot,
    Popover,
    // Progress,
    // Radio,
    // Resizable,
    // Select,
    // Settings,
    // Sheet,
    // Side,
    Skeleton,
    // Slider,
    // Spinner,
    // Stepper,
    // Switch,
    // Table,
    // Tabs,
    // Tag,
    // TitleBar,
    // Toggle,
    // Tooltip,
    // Tree,
    // VirtualList,
}

impl Components {
    /// Create a new View for this component
    /// Takes a generic context that can be dereferenced to App
    pub fn create_view<T>(&self, _: &mut Window, cx: &mut Context<T>) -> AnyView {
        match self {
            Components::Accordion => cx.new(|_cx| AccordionComponentView::new()).into(),
            Components::Alert => cx.new(|_cx| AlertComponentView).into(),
            Components::Avatar => cx.new(|_cx| AvatarComponentView).into(),
            Components::Badge => cx.new(|_cx| BadgeComponentView).into(),
            Components::Button => cx.new(|_cx| ButtonComponentView).into(),
            Components::Calendar => cx.new(|_cx| CalendarComponentView).into(),
            Components::Chart => cx.new(|_cx| ChartComponentView::new()).into(),
            Components::Checkbox => cx.new(|_cx| CheckboxComponentView::new()).into(),
            Components::Clipboard => cx.new(|_cx| ClipboardComponentView).into(),
            Components::Collapsible => cx.new(|_cx| CollapsibleComponentView::new()).into(),
            // Components::ColorPicker => cx.new(|_cx| ColorPickerComponentView).into(),
            // Components::DatePicker => cx.new(|_cx| DatePickerComponentView).into(),
            // Components::DescriptionList => cx.new(|_cx| DescriptionListComponentView).into(),
            // Components::Dialog => cx.new(|_cx| DialogComponentView).into(),
            Components::DropdownButton => cx.new(|_cx| DropdownButtonComponentView).into(),
            // Components::Editor => cx.new(|_cx| EditorComponentView).into(),
            // Components::Form => cx.new(|_cx| FormComponentView).into(),
            // Components::GroupBox => cx.new(|_cx| GroupBoxComponentView).into(),
            // Components::Icon => cx.new(|_cx| IconComponentView).into(),
            // Components::Image => cx.new(|_cx| ImageComponentView).into(),
            // Components::Input => cx.new(|_cx| InputComponentView).into(),
            // Components::Kbd => cx.new(|_cx| KbdComponentView).into(),
            // Components::Label => cx.new(|_cx| LabelComponentView).into(),
            // Components::List => cx.new(|_cx| ListComponentView).into(),
            // Components::Menu => cx.new(|_cx| MenuComponentView).into(),
            // Components::Notification => cx.new(|_cx| NotificationComponentView).into(),
            // Components::NumberInput => cx.new(|_cx| NumberInputComponentView).into(),
            // Components::OptInput => cx.new(|_cx| OptInputComponentView).into(),
            // Components::Plot => cx.new(|_cx| PlotComponentView).into(),
            Components::Popover => cx.new(|_cx| PopoverComponentView::new()).into(),
            // Components::Progress => cx.new(|_cx| ProgressComponentView).into(),
            // Components::Radio => cx.new(|_cx| RadioComponentView).into(),
            // Components::Resizable => cx.new(|_cx| ResizableComponentView).into(),
            // Components::Select => cx.new(|_cx| SelectComponentView).into(),
            // Components::Settings => cx.new(|_cx| SettingsComponentView).into(),
            // Components::Sheet => cx.new(|_cx| SheetComponentView).into(),
            // Components::Side => cx.new(|_cx| SideComponentView).into(),
            Components::Skeleton => cx.new(|_cx| SkeletonComponentView::new()).into(),
            // Components::Slider => cx.new(|_cx| SliderComponentView).into(),
            // Components::Spinner => cx.new(|_cx| SpinnerComponentView).into(),
            // Components::Stepper => cx.new(|_cx| StepperComponentView).into(),
            // Components::Switch => cx.new(|_cx| SwitchComponentView).into(),
            // Components::Table => cx.new(|_cx| TableComponentView).into(),
            // Components::Tabs => cx.new(|_cx| TabsComponentView).into(),
            // Components::Tag => cx.new(|_cx| TagComponentView).into(),
            // Components::TitleBar => cx.new(|_cx| TitleBarComponentView).into(),
            // Components::Toggle => cx.new(|_cx| ToggleComponentView).into(),
            // Components::Tooltip => cx.new(|_cx| TooltipComponentView).into(),
            // Components::Tree => cx.new(|_cx| TreeComponentView).into(),
            // Components::VirtualList => cx.new(|_cx| VirtualListComponentView).into(),
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Components::Accordion => AccordionComponentView::DESCRIPTION,
            Components::Alert => AlertComponentView::DESCRIPTION,
            Components::Avatar => AvatarComponentView::DESCRIPTION,
            Components::Badge => BadgeComponentView::DESCRIPTION,
            Components::Button => ButtonComponentView::DESCRIPTION,
            Components::Calendar => CalendarComponentView::DESCRIPTION,
            Components::Chart => ChartComponentView::DESCRIPTION,
            Components::Checkbox => CheckboxComponentView::DESCRIPTION,
            Components::Clipboard => ClipboardComponentView::DESCRIPTION,
            Components::Collapsible => CollapsibleComponentView::DESCRIPTION,
            // Components::ColorPicker => ColorPickerComponentView::DESCRIPTION,
            // Components::DatePicker => DatePickerComponentView::DESCRIPTION,
            // Components::DescriptionList => DescriptionListComponentView::DESCRIPTION,
            // Components::Dialog => DialogComponentView::DESCRIPTION,
            Components::DropdownButton => DropdownButtonComponentView::DESCRIPTION,
            // Components::Editor => EditorComponentView::DESCRIPTION,
            // Components::Form => FormComponentView::DESCRIPTION,
            // Components::GroupBox => GroupBoxComponentView::DESCRIPTION,
            // Components::Icon => IconComponentView::DESCRIPTION,
            // Components::Image => ImageComponentView::DESCRIPTION,
            // Components::Input => InputComponentView::DESCRIPTION,
            // Components::Kbd => KbdComponentView::DESCRIPTION,
            // Components::Label => LabelComponentView::DESCRIPTION,
            // Components::List => ListComponentView::DESCRIPTION,
            // Components::Menu => MenuComponentView::DESCRIPTION,
            // Components::Notification => NotificationComponentView::DESCRIPTION,
            // Components::NumberInput => NumberInputComponentView::DESCRIPTION,
            // Components::OptInput => OptInputComponentView::DESCRIPTION,
            // Components::Plot => PlotComponentView::DESCRIPTION,
            Components::Popover => PopoverComponentView::DESCRIPTION,
            // Components::Progress => ProgressComponentView::DESCRIPTION,
            // Components::Radio => RadioComponentView::DESCRIPTION,
            // Components::Resizable => ResizableComponentView::DESCRIPTION,
            // Components::Select => SelectComponentView::DESCRIPTION,
            // Components::Settings => SettingsComponentView::DESCRIPTION,
            // Components::Sheet => SheetComponentView::DESCRIPTION,
            // Components::Side => SideComponentView::DESCRIPTION,
            Components::Skeleton => SkeletonComponentView::DESCRIPTION,
            // Components::Slider => SliderComponentView::DESCRIPTION,
            // Components::Spinner => SpinnerComponentView::DESCRIPTION,
            // Components::Stepper => StepperComponentView::DESCRIPTION,
            // Components::Switch => SwitchComponentView::DESCRIPTION,
            // Components::Table => TableComponentView::DESCRIPTION,
            // Components::Tabs => TabsComponentView::DESCRIPTION,
            // Components::Tag => TagComponentView::DESCRIPTION,
            // Components::TitleBar => TitleBarComponentView::DESCRIPTION,
            // Components::Toggle => ToggleComponentView::DESCRIPTION,
            // Components::Tooltip => TooltipComponentView::DESCRIPTION,
            // Components::Tree => TreeComponentView::DESCRIPTION,
            // Components::VirtualList => VirtualListComponentView::DESCRIPTION,
        }
    }

    pub fn link(&self) -> &'static str {
        match self {
            Components::Accordion => AccordionComponentView::LINK,
            Components::Alert => AlertComponentView::LINK,
            Components::Avatar => AvatarComponentView::LINK,
            Components::Badge => BadgeComponentView::LINK,
            Components::Button => ButtonComponentView::LINK,
            Components::Calendar => CalendarComponentView::LINK,
            Components::Chart => ChartComponentView::LINK,
            Components::Checkbox => CheckboxComponentView::LINK,
            Components::Clipboard => ClipboardComponentView::LINK,
            Components::Collapsible => CollapsibleComponentView::LINK,
            // Components::ColorPicker => ColorPickerComponentView::LINK,
            // Components::DatePicker => DatePickerComponentView::LINK,
            // Components::DescriptionList => DescriptionListComponentView::LINK,
            // Components::Dialog => DialogComponentView::LINK,
            Components::DropdownButton => DropdownButtonComponentView::LINK,
            // Components::Editor => EditorComponentView::LINK,
            // Components::Form => FormComponentView::LINK,
            // Components::GroupBox => GroupBoxComponentView::LINK,
            // Components::Icon => IconComponentView::LINK,
            // Components::Image => ImageComponentView::LINK,
            // Components::Input => InputComponentView::LINK,
            // Components::Kbd => KbdComponentView::LINK,
            // Components::Label => LabelComponentView::LINK,
            // Components::List => ListComponentView::LINK,
            // Components::Menu => MenuComponentView::LINK,
            // Components::Notification => NotificationComponentView::LINK,
            // Components::NumberInput => NumberInputComponentView::LINK,
            // Components::OptInput => OptInputComponentView::LINK,
            // Components::Plot => PlotComponentView::LINK,
            Components::Popover => PopoverComponentView::LINK,
            // Components::Progress => ProgressComponentView::LINK,
            // Components::Radio => RadioComponentView::LINK,
            // Components::Resizable => ResizableComponentView::LINK,
            // Components::Select => SelectComponentView::LINK,
            // Components::Settings => SettingsComponentView::LINK,
            // Components::Sheet => SheetComponentView::LINK,
            // Components::Side => SideComponentView::LINK,
            Components::Skeleton => SkeletonComponentView::LINK,
            // Components::Slider => SliderComponentView::LINK,
            // Components::Spinner => SpinnerComponentView::LINK,
            // Components::Stepper => StepperComponentView::LINK,
            // Components::Switch => SwitchComponentView::LINK,
            // Components::Table => TableComponentView::LINK,
            // Components::Tabs => TabsComponentView::LINK,
            // Components::Tag => TagComponentView::LINK,
            // Components::TitleBar => TitleBarComponentView::LINK,
            // Components::Toggle => ToggleComponentView::LINK,
            // Components::Tooltip => TooltipComponentView::LINK,
            // Components::Tree => TreeComponentView::LINK,
            // Components::VirtualList => VirtualListComponentView::LINK,
        }
    }
}
