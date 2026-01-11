use gpui::{AnyElement, App, FontWeight, IntoElement, ParentElement, Styled, Window, div};
use strum_macros::{Display, EnumIter, EnumString};

use super::AccordionComponent;
use super::AlertComponent;
use super::AvatarComponent;
use super::BadgeComponent;
use super::ButtonComponent;
use super::CalendarComponent;

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
        }
    }
}
