use gpui::{AnyElement, App, FontWeight, IntoElement, ParentElement, Styled, Window, div};
use strum_macros::{Display, EnumIter, EnumString};

use super::AccordionComponent;
use super::AlertComponent;
use super::AvatarComponent;
use super::BadgeComponent;
use super::ButtonComponent;
use super::CalendarComponent;
// use super::ChartComponent;
// use super::CheckboxComponent;
// use super::ClipboardComponent;
// use super::CollapsibleComponent;
// use super::ColorPickerComponent;
// use super::DatePickerComponent;
// use super::DescriptionListComponent;
// use super::DialogComponent;
// use super::DropdownButtonComponent;
// use super::EditorComponent;
// use super::FormComponent;
// use super::GroupBoxComponent;
// use super::IconComponent;
// use super::ImageComponent;
// use super::InputComponent;
// use super::KbdComponent;
// use super::LabelComponent;
// use super::ListComponent;
// use super::MenuComponent;
// use super::NotificationComponent;
// use super::NumberInputComponent;
// use super::OptInputComponent;
// use super::PlotComponent;
// use super::PopoverComponent;
// use super::ProgressComponent;
// use super::RadioComponent;
// use super::ResizableComponent;
// use super::SelectComponent;
// use super::SettingsComponent;
// use super::SheetComponent;
// use super::SideComponent;
// use super::SkeletonComponent;
// use super::SliderComponent;
// use super::SpinnerComponent;
// use super::StepperComponent;
// use super::SwitchComponent;
// use super::TableComponent;
// use super::TabsComponent;
// use super::TagComponent;
// use super::TitleBarComponent;
// use super::ToggleComponent;
// use super::TooltipComponent;
// use super::TreeComponent;
// use super::VirtualListComponent;

#[derive(Debug, Clone, Copy, Display, EnumString, EnumIter, PartialEq)]
pub enum Components {
    Accordion,
    Alert,
    Avatar,
    Badge,
    Button,
    Calendar,
    // Chart,
    // Checkbox,
    // Clipboard,
    // Collapsible,
    // ColorPicker,
    // DatePicker,
    // DescriptionList,
    // Dialog,
    // DropdownButton,
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
    // Popover,
    // Progress,
    // Radio,
    // Resizable,
    // Select,
    // Settings,
    // Sheet,
    // Side,
    // Skeleton,
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

pub trait ComponentRenderer {
    fn show(&self, window: &mut Window, cx: &mut App) -> AnyElement;
    fn description(&self) -> &'static str;
    fn link(&self) -> &'static str;
    fn add_subtitle(&self, text: &str) -> AnyElement {
        div()
            .text_2xl()
            .font_weight(FontWeight::BOLD)
            .child(text.to_string())
            .into_any_element()
    }
}

impl Components {
    pub fn get_renderer(&self) -> Box<dyn ComponentRenderer> {
        match self {
            Components::Accordion => Box::new(AccordionComponent),
            Components::Alert => Box::new(AlertComponent),
            Components::Avatar => Box::new(AvatarComponent),
            Components::Badge => Box::new(BadgeComponent),
            Components::Button => Box::new(ButtonComponent),
            Components::Calendar => Box::new(CalendarComponent),
            // Components::Chart => Box::new(ChartComponent),
            // Components::Checkbox => Box::new(CheckboxComponent),
            // Components::Clipboard => Box::new(ClipboardComponent),
            // Components::Collapsible => Box::new(CollapsibleComponent),
            // Components::ColorPicker => Box::new(ColorPickerComponent),
            // Components::DatePicker => Box::new(DatePickerComponent),
            // Components::DescriptionList => Box::new(DescriptionListComponent),
            // Components::Dialog => Box::new(DialogComponent),
            // Components::DropdownButton => Box::new(DropdownButtonComponent),
            // Components::Editor => Box::new(EditorComponent),
            // Components::Form => Box::new(FormComponent),
            // Components::GroupBox => Box::new(GroupBoxComponent),
            // Components::Icon => Box::new(IconComponent),
            // Components::Image => Box::new(ImageComponent),
            // Components::Input => Box::new(InputComponent),
            // Components::Kbd => Box::new(KbdComponent),
            // Components::Label => Box::new(LabelComponent),
            // Components::List => Box::new(ListComponent),
            // Components::Menu => Box::new(MenuComponent),
            // Components::Notification => Box::new(NotificationComponent),
            // Components::NumberInput => Box::new(NumberInputComponent),
            // Components::OptInput => Box::new(OptInputComponent),
            // Components::Plot => Box::new(PlotComponent),
            // Components::Popover => Box::new(PopoverComponent),
            // Components::Progress => Box::new(ProgressComponent),
            // Components::Radio => Box::new(RadioComponent),
            // Components::Resizable => Box::new(ResizableComponent),
            // Components::Select => Box::new(SelectComponent),
            // Components::Settings => Box::new(SettingsComponent),
            // Components::Sheet => Box::new(SheetComponent),
            // Components::Side => Box::new(SideComponent),
            // Components::Skeleton => Box::new(SkeletonComponent),
            // Components::Slider => Box::new(SliderComponent),
            // Components::Spinner => Box::new(SpinnerComponent),
            // Components::Stepper => Box::new(StepperComponent),
            // Components::Switch => Box::new(SwitchComponent),
            // Components::Table => Box::new(TableComponent),
            // Components::Tabs => Box::new(TabsComponent),
            // Components::Tag => Box::new(TagComponent),
            // Components::TitleBar => Box::new(TitleBarComponent),
            // Components::Toggle => Box::new(ToggleComponent),
            // Components::Tooltip => Box::new(TooltipComponent),
            // Components::Tree => Box::new(TreeComponent),
            // Components::VirtualList => Box::new(VirtualListComponent),
        }
    }
}
