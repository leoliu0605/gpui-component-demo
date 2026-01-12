use gpui::*;
use gpui_component::avatar::Avatar;
use gpui_component::badge::Badge;
use gpui_component::link::Link;
use gpui_component::*;

use crate::models::{ComponentMeta, subtitle};

pub struct BadgeComponentView;

impl ComponentMeta for BadgeComponentView {
    const DESCRIPTION: &'static str = "A versatile badge component that can display counts, dots, or icons on elements. \nPerfect for indicating notifications, status, or other contextual information.";
    const LINK: &'static str = "https://longbridge.github.io/gpui-component/docs/components/badge";
}

impl Render for BadgeComponentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Badge with Count"))
            .child(self.badge_with_count())
            .child(subtitle("Variants"))
            .child(self.variants())
            .child(subtitle("Badge Sizes"))
            .child(self.badge_sizes())
            .child(subtitle("Badge Colors"))
            .child(self.badge_colors(_cx))
            .child(subtitle("Badge on Icons"))
            .child(self.badge_on_icons())
            .child(subtitle("Badge on Avatars"))
            .child(self.badge_on_avatars(_cx))
            .child(subtitle("Complex Nested Badges"))
            .child(self.complex_nested_badges(_cx))
            .child(subtitle("Notification Indicators"))
            .child(self.notification_indicators(_cx))
            .child(subtitle("Status Indicators"))
            .child(self.status_indicators(_cx))
            .child(subtitle("Different Badge Positions"))
            .child(self.different_badge_positions())
            .child(subtitle("Count Formatting"))
            .child(self.count_formatting())
    }
}

impl BadgeComponentView {
    /// Example code for the Badge component

    fn badge_with_count(&self) -> AnyElement {
        h_flex()
            .gap_4()
            .child(
                Badge::new()
                    .count(3)
                    .child(Icon::new(IconName::Bell))
                    .into_any_element(),
            )
            .into_any_element()
    }

    fn variants(&self) -> AnyElement {
        h_flex()
            .gap_4()
            .child(
                // Number badge (default)
                Badge::new()
                    .count(5)
                    .child(Avatar::new().src("https://example.com/avatar.jpg")),
            )
            .child(
                // Dot badge
                Badge::new().dot().child(Icon::new(IconName::Inbox)),
            )
            .child(
                // Icon badge
                Badge::new()
                    .icon(IconName::Check)
                    .child(Avatar::new().src("https://example.com/avatar.jpg")),
            )
            .into_any_element()
    }

    fn badge_sizes(&self) -> AnyElement {
        h_flex()
            .gap_4()
            .child(
                // Small badge
                Badge::new().small().count(1).child(Avatar::new().small()),
            )
            .child(
                // Medium badge (default)
                Badge::new().count(5).child(Avatar::new()),
            )
            .child(
                // Large badge
                Badge::new().large().count(10).child(Avatar::new().large()),
            )
            .into_any_element()
    }

    fn badge_colors(&self, cx: &Context<Self>) -> AnyElement {
        h_flex()
            .gap_4()
            .child(
                // Custom colors
                Badge::new()
                    .count(3)
                    .color(cx.theme().blue)
                    .child(Avatar::new()),
            )
            .child(
                Badge::new()
                    .icon(IconName::Star)
                    .color(cx.theme().yellow)
                    .child(Avatar::new()),
            )
            .child(
                Badge::new()
                    .dot()
                    .color(cx.theme().green)
                    .child(Icon::new(IconName::Bell)),
            )
            .into_any_element()
    }

    fn badge_on_icons(&self) -> AnyElement {
        h_flex()
            .gap_4()
            .child(
                // Badge with count on icon
                Badge::new()
                    .count(3)
                    .child(Icon::new(IconName::Bell).large()),
            )
            .child(
                // Badge with high count (shows max)
                Badge::new()
                    .count(103)
                    .child(Icon::new(IconName::Inbox).large()),
            )
            .child(
                // Custom max count
                Badge::new()
                    .count(150)
                    .max(999)
                    .child(Icon::new(IconName::Map)),
            )
            .into_any_element()
    }

    fn badge_on_avatars(&self, cx: &Context<Self>) -> AnyElement {
        h_flex()
            .gap_4()
            .child(
                // Basic count badge
                Badge::new()
                    .count(5)
                    .child(Avatar::new().src("https://example.com/avatar.jpg")),
            )
            .child(
                // Status badge with icon
                Badge::new()
                    .icon(IconName::Check)
                    .color(cx.theme().green)
                    .child(Avatar::new().src("https://example.com/avatar.jpg")),
            )
            .child(
                // Online indicator with dot
                Badge::new()
                    .dot()
                    .color(cx.theme().green)
                    .child(Avatar::new().src("https://example.com/avatar.jpg")),
            )
            .into_any_element()
    }

    fn complex_nested_badges(&self, cx: &Context<Self>) -> AnyElement {
        h_flex()
            .gap_4()
            .child(
                // Badge on badge for complex status
                Badge::new().count(212).large().child(
                    Badge::new()
                        .icon(IconName::Check)
                        .large()
                        .color(cx.theme().cyan)
                        .child(Avatar::new().large().src("https://example.com/avatar.jpg")),
                ),
            )
            .child(
                // Multiple status indicators
                Badge::new().count(2).color(cx.theme().green).large().child(
                    Badge::new()
                        .icon(IconName::Star)
                        .large()
                        .color(cx.theme().yellow)
                        .child(Avatar::new().large().src("https://example.com/avatar.jpg")),
                ),
            )
            .into_any_element()
    }

    fn notification_indicators(&self, cx: &Context<Self>) -> AnyElement {
        h_flex()
            .gap_4()
            .child(
                // Unread messages
                Badge::new()
                    .count(12)
                    .child(Icon::new(IconName::Map).large()),
            )
            .child(
                // New notifications
                Badge::new()
                    .count(3)
                    .color(cx.theme().red)
                    .child(Icon::new(IconName::Bell).large()),
            )
            .child(
                // High priority with custom max
                Badge::new()
                    .count(1234)
                    .max(999)
                    .color(cx.theme().red_light)
                    .child(Icon::new(IconName::Heart)),
            )
            .into_any_element()
    }

    fn status_indicators(&self, cx: &Context<Self>) -> AnyElement {
        h_flex()
            .gap_4()
            .child(
                // Online status
                Badge::new()
                    .dot()
                    .color(cx.theme().green)
                    .child(Avatar::new().src("https://example.com/user.jpg")),
            )
            .child(
                // Verified status
                Badge::new()
                    .icon(IconName::CircleCheck)
                    .color(cx.theme().blue)
                    .child(Avatar::new().src("https://example.com/verified-user.jpg")),
            )
            .child(
                // Warning status
                Badge::new()
                    .icon(IconName::Heart)
                    .color(cx.theme().yellow)
                    .child(Avatar::new().src("https://example.com/user.jpg")),
            )
            .into_any_element()
    }

    fn different_badge_positions(&self) -> AnyElement {
        Link::new("different_badge_positions")
        .href("https://longbridge.github.io/gpui-component/docs/components/badge#different-badge-positions")
        .child("Click here to learn more")
        .into_any_element()
    }

    fn count_formatting(&self) -> AnyElement {
        h_flex()
            .gap_4()
            // Numbers 1-99 show as-is
            .child(
                Badge::new().count(5), // Shows "5"
            )
            .child(
                Badge::new().count(99), // Shows "99"
            )
            // Numbers above max show with "+"
            .child(
                Badge::new().count(100), // Shows "99+" (default max)
            )
            .child(
                Badge::new().count(1000).max(999), // Shows "999+"
            )
            // Zero count hides the badge
            .child(
                Badge::new().count(0), // Badge not visible
            )
            .into_any_element()
    }
}
