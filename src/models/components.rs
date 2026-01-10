use gpui::{AnyElement, App, Window};
use strum_macros::{Display, EnumIter, EnumString};

use super::AccordionComponent;

#[derive(Debug, Clone, Copy, Display, EnumString, EnumIter, PartialEq)]
pub enum Components {
    Accordion,
    // Alert,
    // Avatar,
    // Badge,
    // Button,
    // Calendar,
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
}

impl Components {
    pub fn get_renderer(&self) -> Box<dyn ComponentRenderer> {
        match self {
            Components::Accordion => Box::new(AccordionComponent),
        }
    }
}
