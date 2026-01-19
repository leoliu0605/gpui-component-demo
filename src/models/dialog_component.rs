use gpui::*;
use gpui_component::button::{ButtonVariant, ButtonVariants};
use gpui_component::dialog::DialogButtonProps;
use gpui_component::input::{Input, InputState};
use gpui_component::text::TextView;
use gpui_component::*;
use gpui_component::{WindowExt, button::Button};

use crate::models::{ComponentMeta, subtitle};

pub struct DialogComponentView;

impl ComponentMeta for DialogComponentView {
    const DESCRIPTION: &'static str = "Dialog component for creating dialogs, confirmations, and alerts. \nSupports overlay, keyboard shortcuts, and various customizations.";
    const LINK: &'static str = "https://longbridge.github.io/gpui-component/docs/components/dialog";
}

impl Render for DialogComponentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Basic Dialog"))
            .child(self.basic_dialog())
            .child(subtitle("Form Dialog"))
            .child(self.form_dialog())
            .child(subtitle("Confirm Dialog"))
            .child(self.confirm_dialog())
            .child(subtitle("Alert Dialog"))
            .child(self.alert_dialog())
            .child(subtitle("Custom Button Labels"))
            .child(self.custom_button_labels())
            .child(subtitle("Dialog with Icon"))
            .child(self.dialog_with_icon())
            .child(subtitle("Scrollable Dialog"))
            .child(self.scrollable_dialog())
            .child(subtitle("Dialog Options"))
            .child(self.dialog_options())
            .child(subtitle("Nested Dialogs"))
            .child(self.nested_dialogs())
            .child(subtitle("Custom Styling"))
            .child(self.custom_styling())
            .child(subtitle("Custom Padding"))
            .child(self.custom_padding())
            .child(subtitle("Close Dialog Programmatically"))
            .child(self.close_dialog_programmatically())
            .child(subtitle("Delete Confirmation"))
            .child(self.delete_confirmation())
            .child(subtitle("Success Alert"))
            .child(self.success_alert())
    }
}

impl DialogComponentView {
    /// Example code for the Dialog component

    fn basic_dialog(&self) -> AnyElement {
        Button::new("open-dialog-button")
            .label("Open Dialog")
            .on_click(move |_, window, cx| {
                window.open_dialog(cx, |dialog, _, _| {
                    dialog.title("Welcome").child("This is a dialog dialog.")
                });
            })
            .into_any_element()
    }

    fn form_dialog(&self) -> AnyElement {
        Button::new("open-form-dialog-button")
            .label("Open Form Dialog")
            .on_click(move |_, window, cx| {
                let input = cx.new(|cx| InputState::new(window, cx));
                let input = input.clone();

                window.open_dialog(cx, move |dialog, _, _| {
                    dialog
                        .title("User Information")
                        .child(
                            v_flex()
                                .gap_3()
                                .child("Please enter your details:")
                                .child(Input::new(&input)),
                        )
                        .footer(|_, _, _, _| {
                            vec![
                                Button::new("ok").primary().label("Submit").on_click(
                                    |_, window, cx| {
                                        window.close_dialog(cx);
                                    },
                                ),
                                Button::new("cancel")
                                    .label("Cancel")
                                    .on_click(|_, window, cx| {
                                        window.close_dialog(cx);
                                    }),
                            ]
                        })
                });
            })
            .into_any_element()
    }

    fn confirm_dialog(&self) -> AnyElement {
        Button::new("open-confirm-dialog-button")
            .label("Open Confirm Dialog")
            .on_click(move |_, window, cx| {
                window.open_dialog(cx, |dialog, _, _| {
                    dialog
                        .confirm()
                        .child("Are you sure you want to delete this item?")
                        .on_ok(|_, window, cx| {
                            window.push_notification("Item deleted", cx);
                            true // Return true to close dialog
                        })
                        .on_cancel(|_, window, cx| {
                            window.push_notification("Cancelled", cx);
                            true
                        })
                })
            })
            .into_any_element()
    }

    fn alert_dialog(&self) -> AnyElement {
        Button::new("open-alert-dialog-button")
            .label("Open Alert Dialog")
            .on_click(move |_, window, cx| {
                window.open_dialog(cx, |dialog, _, _| {
                    dialog
                        .alert()
                        .child("Operation completed successfully!")
                        .on_close(|_, window, cx| {
                            window.push_notification("Alert closed", cx);
                        })
                })
            })
            .into_any_element()
    }

    fn custom_button_labels(&self) -> AnyElement {
        Button::new("open-custom-button-labels-dialog-button")
            .label("Open Dialog with Custom Button Labels")
            .on_click(move |_, window, cx| {
                window.open_dialog(cx, |dialog, _, _| {
                    dialog
                        .confirm()
                        .child("Update available. Restart now?")
                        .button_props(
                            DialogButtonProps::default()
                                .cancel_text("Later")
                                .cancel_variant(ButtonVariant::Secondary)
                                .ok_text("Restart Now")
                                .ok_variant(ButtonVariant::Danger),
                        )
                        .on_ok(|_, window, cx| {
                            window.push_notification("Restarting...", cx);
                            true
                        })
                })
            })
            .into_any_element()
    }

    fn dialog_with_icon(&self) -> AnyElement {
        Button::new("open-dialog-with-icon-button")
            .label("Open Dialog with Icon")
            .on_click(move |_, window, cx| {
                window.open_dialog(cx, |dialog, _, cx| {
                    dialog.confirm().child(
                        h_flex()
                            .gap_3()
                            .child(
                                Icon::new(IconName::TriangleAlert)
                                    .size_6()
                                    .text_color(cx.theme().warning),
                            )
                            .child("This action cannot be undone."),
                    )
                })
            })
            .into_any_element()
    }

    fn scrollable_dialog(&self) -> AnyElement {
        Button::new("open-scrollable-dialog-button")
            .label("Open Scrollable Dialog")
            .on_click(move |_, window, cx| {
                let long_markdown_text = r#"
# Lorem Ipsum
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
# Section 1
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
# Section 2
Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
# Section 3
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
# Section 4
Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
"#;
                window.open_dialog(cx, move |dialog, window, cx| {
                    dialog
                        .h(px(450.))
                        .title("Long Content")
                        .child(TextView::markdown(
                            "content",
                            long_markdown_text,
                            window,
                            cx,
                        ))
                })
            })
            .into_any_element()
    }

    fn dialog_options(&self) -> AnyElement {
        Button::new("open-dialog-options-button")
            .label("Open Dialog with Options")
            .on_click(move |_, window, cx| {
                window.open_dialog(cx, |dialog, _, _| {
                    dialog
                        .title("Custom Dialog")
                        .overlay(true) // Show overlay (default: true)
                        .overlay_closable(true) // Click overlay to close (default: true)
                        .keyboard(true) // ESC to close (default: true)
                        .close_button(false) // Show close button (default: true)
                        .child("Dialog content")
                })
            })
            .into_any_element()
    }

    fn nested_dialogs(&self) -> AnyElement {
        Button::new("open-nested-dialogs-button")
            .label("Open Nested Dialogs")
            .on_click(move |_, window, cx| {
                window.open_dialog(cx, |dialog, _, _| {
                    dialog
                        .title("First Dialog")
                        .child("This is the first dialog")
                        .footer(|_, _, _, _| {
                            vec![
                                Button::new("open-another")
                                    .label("Open Another Dialog")
                                    .on_click(|_, window, cx| {
                                        window.open_dialog(cx, |dialog, _, _| {
                                            dialog.title("Second Dialog").child("This is nested")
                                        });
                                    }),
                            ]
                        })
                })
            })
            .into_any_element()
    }

    fn custom_styling(&self) -> AnyElement {
        Button::new("open-custom-styling-dialog-button")
            .label("Open Dialog with Custom Styling")
            .on_click(move |_, window, cx| {
                window.open_dialog(cx, |dialog, _, cx| {
                    dialog
                        .rounded_lg()
                        .bg(cx.theme().cyan)
                        .text_color(cx.theme().info_foreground)
                        .title("Custom Style")
                        .child("Styled dialog content")
                })
            })
            .into_any_element()
    }

    fn custom_padding(&self) -> AnyElement {
        Button::new("open-custom-padding-dialog-button")
            .label("Open Dialog with Custom Padding")
            .on_click(move |_, window, cx| {
                window.open_dialog(cx, |dialog, _, _| {
                    dialog
                        .p_3() // Custom padding
                        .title("Custom Padding")
                        .child("Dialog with custom spacing")
                })
            })
            .into_any_element()
    }

    fn close_dialog_programmatically(&self) -> AnyElement {
        Button::new("open-close-programmatically-dialog-button")
            .label("Open Dialog and Close Programmatically")
            .on_click(move |_, window, cx| {
                // Close top level active dialog.
                window.close_dialog(cx);

                // Close and perform action
                Button::new("submit")
                    .primary()
                    .label("Submit")
                    .on_click(|_, window, cx| {
                        // Do something
                        window.close_dialog(cx);
                    });
            })
            .into_any_element()
    }

    fn delete_confirmation(&self) -> AnyElement {
        Button::new("delete")
            .danger()
            .label("Delete")
            .on_click(|_, window, cx| {
                window.open_dialog(cx, |dialog, _, _| {
                    dialog
                        .confirm()
                        .child("Are you sure you want to delete this item?")
                        .on_ok(|_, window, cx| {
                            // Perform delete
                            window.push_notification("Deleted", cx);
                            true
                        })
                });
            })
            .into_any_element()
    }

    fn success_alert(&self) -> AnyElement {
        Button::new("show-success-alert-button")
            .label("Show Success Alert")
            .on_click(|_, window, cx| {
                window.open_dialog(cx, |dialog, _, _| {
                    dialog
                        .confirm()
                        .alert()
                        .child("Your changes have been saved successfully!")
                        .on_close(|_, _, _| {
                            // Optional close handler
                        })
                })
            })
            .into_any_element()
    }
}
