use gpui::*;
use gpui_component::avatar::{Avatar, AvatarGroup};
use gpui_component::*;

use crate::models::{ComponentMeta, subtitle};

pub struct AvatarComponentView;

impl ComponentMeta for AvatarComponentView {
    const DESCRIPTION: &'static str = "The Avatar component displays user profile images with intelligent fallbacks. \nWhen no image is provided, it shows user initials or a placeholder icon.";
    const LINK: &'static str = "https://longbridge.github.io/gpui-component/docs/components/avatar";
}

impl Render for AvatarComponentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Basic Avatar"))
            .child(self.basic_avatar())
            .child(subtitle("Avatar with Fallback Text"))
            .child(self.avatar_with_fallback_text())
            .child(subtitle("Avatar Placeholder"))
            .child(self.avatar_placeholder())
            .child(subtitle("Avatar Sizes"))
            .child(self.avatar_sizes())
            .child(subtitle("Custom Styling"))
            .child(self.custom_styling(_cx))
            .child(subtitle("Basic Group"))
            .child(self.basic_group())
            .child(subtitle("Group with Limit"))
            .child(self.group_with_limit())
            .child(subtitle("Group with Ellipsis"))
            .child(self.group_with_ellipsis())
            .child(subtitle("Group Sizes"))
            .child(self.group_sizes())
            .child(subtitle("Adding Multiple Avatars"))
            .child(self.adding_multiple_avatars())
            .child(subtitle("Team Display"))
            .child(self.team_display())
            .child(subtitle("User Profile Header"))
            .child(self.user_profile_header(_cx))
            .child(subtitle("Anonymous User"))
            .child(self.anonymous_user())
            .child(subtitle("Avatar with Custom Colors"))
            .child(self.avatar_with_custom_colors())
    }
}

impl AvatarComponentView {
    /// Example code for the Avatar component

    fn basic_avatar(&self) -> AnyElement {
        Avatar::new()
            .name("John Doe")
            .src("https://example.com/avatar.jpg")
            .into_any_element()
    }

    fn avatar_with_fallback_text(&self) -> AnyElement {
        h_flex()
            .gap_4()
            .child(Avatar::new().name("John Doe"))
            .child(Avatar::new().name("Jane Smith"))
            .into_any_element()
    }

    fn avatar_placeholder(&self) -> AnyElement {
        h_flex()
            .gap_4()
            .child(Avatar::new())
            .child(Avatar::new().placeholder(IconName::Building2))
            .into_any_element()
    }

    fn avatar_sizes(&self) -> AnyElement {
        h_flex()
            .gap_4()
            .child(Avatar::new().name("John Doe").xsmall())
            .child(Avatar::new().name("John Doe").small())
            .child(Avatar::new().name("John Doe"))
            .child(Avatar::new().name("John Doe").large())
            .child(Avatar::new().name("John Doe").with_size(px(100.)))
            .into_any_element()
    }

    fn custom_styling(&self, cx: &Context<Self>) -> AnyElement {
        Avatar::new()
            .src("https://example.com/avatar.jpg")
            .with_size(px(100.))
            .border_3()
            .border_color(cx.theme().foreground)
            .shadow_sm()
            .rounded(px(20.)) // Custom border radius
            .into_any_element()
    }

    fn basic_group(&self) -> AnyElement {
        AvatarGroup::new()
            .child(Avatar::new().src("https://example.com/user1.jpg"))
            .child(Avatar::new().src("https://example.com/user2.jpg"))
            .child(Avatar::new().src("https://example.com/user3.jpg"))
            .child(Avatar::new().name("John Doe"))
            .into_any_element()
    }

    fn group_with_limit(&self) -> AnyElement {
        AvatarGroup::new()
            .limit(3) // Show maximum 3 avatars
            .child(Avatar::new().src("https://example.com/user1.jpg"))
            .child(Avatar::new().src("https://example.com/user2.jpg"))
            .child(Avatar::new().src("https://example.com/user3.jpg"))
            .child(Avatar::new().src("https://example.com/user4.jpg")) // Hidden
            .child(Avatar::new().src("https://example.com/user5.jpg")) // Hidden
            .into_any_element()
    }

    fn group_with_ellipsis(&self) -> AnyElement {
        AvatarGroup::new()
            .limit(3)
            .ellipsis() // Shows "..." when limit is exceeded
            .child(Avatar::new().src("https://example.com/user1.jpg"))
            .child(Avatar::new().src("https://example.com/user2.jpg"))
            .child(Avatar::new().src("https://example.com/user3.jpg"))
            .child(Avatar::new().src("https://example.com/user4.jpg"))
            .child(Avatar::new().src("https://example.com/user5.jpg"))
            .into_any_element()
    }

    fn group_sizes(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                // Extra small group
                AvatarGroup::new()
                    .xsmall()
                    .child(Avatar::new().name("A"))
                    .child(Avatar::new().name("B"))
                    .child(Avatar::new().name("C")),
            )
            .child(
                // Small group
                AvatarGroup::new()
                    .small()
                    .child(Avatar::new().name("A"))
                    .child(Avatar::new().name("B")),
            )
            .child(
                // Medium group (default)
                AvatarGroup::new()
                    .child(Avatar::new().name("A"))
                    .child(Avatar::new().name("B")),
            )
            .child(
                // Large group
                AvatarGroup::new()
                    .large()
                    .child(Avatar::new().name("A"))
                    .child(Avatar::new().name("B")),
            )
            .into_any_element()
    }

    fn adding_multiple_avatars(&self) -> AnyElement {
        let avatars = vec![
            Avatar::new().src("https://example.com/user1.jpg"),
            Avatar::new().src("https://example.com/user2.jpg"),
            Avatar::new().name("John Doe"),
        ];

        AvatarGroup::new()
            .children(avatars)
            .limit(5)
            .ellipsis()
            .into_any_element()
    }

    fn team_display(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child("Development Team")
            .child(
                AvatarGroup::new()
                    .limit(4)
                    .ellipsis()
                    .child(
                        Avatar::new()
                            .name("Alice Johnson")
                            .src("https://example.com/alice.jpg"),
                    )
                    .child(
                        Avatar::new()
                            .name("Bob Smith")
                            .src("https://example.com/bob.jpg"),
                    )
                    .child(Avatar::new().name("Charlie Brown"))
                    .child(Avatar::new().name("Diana Prince"))
                    .child(Avatar::new().name("Eve Wilson")),
            )
            .into_any_element()
    }

    fn user_profile_header(&self, cx: &Context<Self>) -> AnyElement {
        h_flex()
            .items_center()
            .gap_4()
            .child(
                Avatar::new()
                    .src("https://example.com/profile.jpg")
                    .name("John Doe")
                    .large()
                    .border_2()
                    .border_color(cx.theme().primary),
            )
            .child(v_flex().child("John Doe").child("Software Engineer"))
            .into_any_element()
    }

    fn anonymous_user(&self) -> AnyElement {
        Avatar::new()
            .placeholder(IconName::CircleUser)
            .large()
            .into_any_element()
    }

    fn avatar_with_custom_colors(&self) -> AnyElement {
        h_flex()
            .gap_4()
            // The avatar automatically generates colors based on the name
            // Different names will get different colors from the color palette
            .child(
                Avatar::new().name("Alice"), // Gets one color
            )
            .child(
                Avatar::new().name("Bob"), // Gets a different color
            )
            .child(
                Avatar::new().name("Charlie"), // Gets another color
            )
            .into_any_element()
    }
}
