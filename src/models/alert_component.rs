use gpui::*;
use gpui_component::alert::Alert;
// use gpui_component::text::markdown;
use gpui_component::*;

use crate::models::{ComponentMeta, subtitle};

pub struct AlertComponentView;

impl ComponentMeta for AlertComponentView {
    const DESCRIPTION: &'static str = "A versatile alert component for displaying important messages to users. \nSupports multiple variants (info, success, warning, error), custom icons, optional titles, \nclosable functionality, and banner mode.";
    const LINK: &'static str = "https://longbridge.github.io/gpui-component/docs/components/alert";
}

impl Render for AlertComponentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Basic Alert"))
            .child(self.basic_alert())
            .child(subtitle("Alert with Title"))
            .child(self.alert_with_title())
            .child(subtitle("Alert Variants"))
            .child(self.alert_variants())
            .child(subtitle("Alert Sizes"))
            .child(self.alert_sizes())
            .child(subtitle("Closable Alerts"))
            .child(self.closable_alerts())
            .child(subtitle("Banner Mode"))
            .child(self.banner_mode())
            .child(subtitle("Custom Icons"))
            .child(self.custom_icons())
            .child(subtitle("With Markdown Content"))
            .child(self.with_markdown_content())
            .child(subtitle("Conditional Visibility"))
            .child(self.conditional_visibility())
            .child(subtitle("Form Validation Errors"))
            .child(self.form_validation_errors())
            .child(subtitle("Success Notification"))
            .child(self.success_notification())
            .child(subtitle("System Status Banner"))
            .child(self.system_status_banner())
            .child(subtitle("Interactive Alert with Custom Action"))
            .child(self.interactive_alert_with_custom_action())
            .child(subtitle("Multi-line Content with Formatting"))
            .child(self.multi_line_content_with_formatting())
    }
}

impl AlertComponentView {
    /// Example code for the Alert component

    fn basic_alert(&self) -> AnyElement {
        Alert::new("alert-id", "This is a basic alert message.").into_any_element()
    }

    fn alert_with_title(&self) -> AnyElement {
        Alert::new(
            "alert-with-title",
            "Your changes have been saved successfully.",
        )
        .title("Success!")
        .into_any_element()
    }

    fn alert_variants(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                h_flex()
                    .gap_4()
                    .child(
                        Alert::info("info-alert", "This is an informational message.")
                            .title("Information"),
                    )
                    .child(
                        Alert::success("success-alert", "Your operation completed successfully.")
                            .title("Success!"),
                    ),
            )
            .child(
                h_flex()
                    .gap_4()
                    .child(
                        Alert::warning(
                            "warning-alert",
                            "Please review your settings before proceeding.",
                        )
                        .title("Warning"),
                    )
                    .child(
                        Alert::error(
                            "error-alert",
                            "An error occurred while processing your request.",
                        )
                        .title("Error"),
                    ),
            )
            .into_any_element()
    }

    fn alert_sizes(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                h_flex()
                    .gap_4()
                    .child(
                        Alert::info("alert-xsmall", "Message content")
                            .xsmall()
                            .title("XSmall Alert"),
                    )
                    .child(
                        Alert::info("alert-small", "Message content")
                            .small()
                            .title("Small Alert"),
                    ),
            )
            .child(
                h_flex()
                    .gap_4()
                    .child(Alert::info("alert-medium", "Message content").title("Medium Alert"))
                    .child(
                        Alert::info("alert-large", "Message content")
                            .large()
                            .title("Large Alert"),
                    ),
            )
            .into_any_element()
    }

    fn closable_alerts(&self) -> AnyElement {
        Alert::info("closable-alert", "This alert can be dismissed.")
            .title("Dismissible")
            .on_close(|_event, _window, _cx| {
                println!("Alert was closed");
                // Handle alert dismissal
            })
            .into_any_element()
    }

    fn banner_mode(&self) -> AnyElement {
        v_flex()
            .gap_4()
            .child(
                h_flex()
                    .gap_4()
                    .child(
                        Alert::info(
                            "banner-alert",
                            "This is a banner alert that spans the full width.",
                        )
                        .banner(),
                    )
                    .child(
                        Alert::success("banner-success", "Operation completed successfully!")
                            .banner(),
                    ),
            )
            .child(
                h_flex()
                    .gap_4()
                    .child(
                        Alert::warning(
                            "banner-warning",
                            "System maintenance scheduled for tonight.",
                        )
                        .banner(),
                    )
                    .child(
                        Alert::error("banner-error", "Service temporarily unavailable.").banner(),
                    ),
            )
            .into_any_element()
    }

    fn custom_icons(&self) -> AnyElement {
        Alert::new("custom-icon", "Meeting scheduled for tomorrow at 3 PM.")
            .title("Calendar Reminder")
            .icon(IconName::Calendar)
            .into_any_element()
    }

    fn with_markdown_content(&self) -> AnyElement {
        Alert::error(
            "error-with-markdown",
            "Please verify your billing information and try again.\n\
        - Check your card details\n\
        - Ensure sufficient funds\n\
        - Verify billing address",
        )
        .title("Payment Failed")
        .into_any_element()
    }

    fn conditional_visibility(&self) -> AnyElement {
        let should_show_alert = true; // This would be based on some application logic

        Alert::info("conditional-alert", "This alert may be hidden.")
            .title("Conditional")
            .visible(should_show_alert) // boolean condition
            .into_any_element()
    }

    fn form_validation_errors(&self) -> AnyElement {
        Alert::error(
            "validation-error",
            "Please correct the following errors before submitting:\n\
    - Email address is required\n\
    - Password must be at least 8 characters\n\
    - Terms of service must be accepted",
        )
        .title("Validation Failed")
        .into_any_element()
    }

    fn success_notification(&self) -> AnyElement {
        Alert::success(
            "save-success",
            "Your profile has been updated successfully.",
        )
        .title("Changes Saved")
        .on_close(|_, _, _| {
            // Auto-dismiss after showing
        })
        .into_any_element()
    }

    fn system_status_banner(&self) -> AnyElement {
        Alert::warning(
            "maintenance-banner",
            "Scheduled maintenance will occur tonight from 2:00 AM to 4:00 AM EST. \
    Some services may be temporarily unavailable.",
        )
        .banner()
        .large()
        .into_any_element()
    }

    fn interactive_alert_with_custom_action(&self) -> AnyElement {
        Alert::info(
            "update-available",
            "A new version of the application is available.",
        )
        .title("Update Available")
        .icon(IconName::Settings)
        .on_close(|_, _, _| {
            // Handle update or dismiss
            println!("Update notification closed");
        })
        .into_any_element()
    }

    fn multi_line_content_with_formatting(&self) -> AnyElement {
        // Alert::warning(
        //     "security-alert",
        //     markdown(
        //         "**Security Notice**: Unusual activity detected on your account.\n\n\
        // Recent activity:\n\
        // - Login from new device (Chrome on Windows)\n\
        // - Location: San Francisco, CA\n\
        // - Time: Today at 2:30 PM\n\n\
        // If this wasn't you, please [change your password](/) immediately.",
        //     ),
        // )
        // .title("Security Alert")
        // .icon(IconName::Shield)
        // .into_any_element()

        div()
            .text_color(rgb(0x666666))
            .child("Not implemented yet")
            .into_any_element()
    }
}
