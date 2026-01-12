use std::collections::HashSet;

use gpui::*;
use gpui_component::accordion::Accordion;
use gpui_component::*;

use crate::models::{ComponentMeta, subtitle};

pub struct AccordionComponentView {
    /// A set to track which accordion items are open by their unique key
    open_items: HashSet<String>,
}

impl AccordionComponentView {
    pub fn new() -> Self {
        Self {
            open_items: HashSet::new(),
        }
    }

    fn is_open(&self, key: &str) -> bool {
        self.open_items.contains(key)
    }
}

impl ComponentMeta for AccordionComponentView {
    const DESCRIPTION: &'static str = "An accordion component that allows users to show and hide sections of content. \nIt uses collapse functionality internally to create collapsible panels.";
    const LINK: &'static str =
        "https://longbridge.github.io/gpui-component/docs/components/accordion";
}

impl Render for AccordionComponentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let entity = _cx.entity().clone();

        v_flex()
            .gap_2()
            .w_full()
            .max_w_96()
            .child(subtitle("Basic Accordion"))
            .child(self.basic_accordion(entity.clone()))
            .child(subtitle("Multiple Open Items"))
            .child(self.multiple_open_items_accordion(entity.clone()))
            .child(subtitle("With Borders"))
            .child(self.with_borders_accordion(entity.clone()))
            .child(subtitle("Different Sizes"))
            .child(self.different_sizes_accordion(entity.clone()))
            .child(subtitle("Handle Toggle Events"))
            .child(self.handle_toggle_events_accordion(entity.clone()))
            .child(subtitle("Disabled State"))
            .child(self.disabled_state_accordion(entity.clone()))
            .child(subtitle("With Custom Icons"))
            .child(self.with_custom_icons_accordion(entity.clone()))
            .child(subtitle("Nested Accordions"))
            .child(self.nested_accordions_accordion(entity.clone()))
    }
}

impl AccordionComponentView {
    /// Example code for the Accordion component

    fn basic_accordion(&self, entity: Entity<Self>) -> AnyElement {
        let e = entity.clone();

        Accordion::new("basic-accordion")
            .item(|item| {
                item.title("Section 1")
                    .open(self.is_open("basic-1"))
                    .child("Content for section 1")
            })
            .item(|item| {
                item.title("Section 2")
                    .open(self.is_open("basic-2"))
                    .child("Content for section 2")
            })
            .item(|item| {
                item.title("Section 3")
                    .open(self.is_open("basic-3"))
                    .child("Content for section 3")
            })
            .on_toggle_click(move |open_indices, _window, _cx| {
                e.update(_cx, |view, cx| {
                    // Toggle based on which items are now open
                    view.open_items.clear();
                    view.open_items.extend(
                        ["basic-1", "basic-2", "basic-3"]
                            .iter()
                            .enumerate()
                            .filter_map(|(i, key)| {
                                if open_indices.contains(&i) {
                                    Some(key.to_string())
                                } else {
                                    None
                                }
                            }),
                    );
                    cx.notify();
                });
            })
            .into_any_element()
    }

    fn multiple_open_items_accordion(&self, entity: Entity<Self>) -> AnyElement {
        let e = entity.clone();

        Accordion::new("multiple-accordion")
            .multiple(true)
            .item(|item| {
                item.title("Section 1")
                    .open(self.is_open("multiple-1"))
                    .child("Content 1")
            })
            .item(|item| {
                item.title("Section 2")
                    .open(self.is_open("multiple-2"))
                    .child("Content 2")
            })
            .on_toggle_click(move |open_indices, _window, _cx| {
                e.update(_cx, |view, cx| {
                    let keys = ["multiple-1", "multiple-2"];
                    for (i, key) in keys.iter().enumerate() {
                        if open_indices.contains(&i) {
                            view.open_items.insert(key.to_string());
                        } else {
                            view.open_items.remove(*key);
                        }
                    }
                    cx.notify();
                });
            })
            .into_any_element()
    }

    fn with_borders_accordion(&self, entity: Entity<Self>) -> AnyElement {
        let e = entity.clone();

        Accordion::new("bordered-accordion")
            .bordered(true)
            .item(|item| {
                item.title("Section 1")
                    .open(self.is_open("bordered-1"))
                    .child("Content 1")
            })
            .on_toggle_click(move |open_indices, _window, _cx| {
                e.update(_cx, |view, cx| {
                    if open_indices.contains(&0) {
                        view.open_items.insert("bordered-1".to_string());
                    } else {
                        view.open_items.remove("bordered-1");
                    }
                    cx.notify();
                });
            })
            .into_any_element()
    }

    fn different_sizes_accordion(&self, entity: Entity<Self>) -> AnyElement {
        let e1 = entity.clone();
        let e2 = entity.clone();

        v_flex()
            .gap_4()
            .child(
                Accordion::new("small-accordion")
                    .small()
                    .item(|item| {
                        item.title("Small Section")
                            .open(self.is_open("small-1"))
                            .child("Content")
                    })
                    .on_toggle_click(move |open_indices, _window, _cx| {
                        e1.update(_cx, |view, cx| {
                            if open_indices.contains(&0) {
                                view.open_items.insert("small-1".to_string());
                            } else {
                                view.open_items.remove("small-1");
                            }
                            cx.notify();
                        });
                    }),
            )
            .child(
                Accordion::new("large-accordion")
                    .large()
                    .item(|item| {
                        item.title("Large Section")
                            .open(self.is_open("large-1"))
                            .child("Content")
                    })
                    .on_toggle_click(move |open_indices, _window, _cx| {
                        e2.update(_cx, |view, cx| {
                            if open_indices.contains(&0) {
                                view.open_items.insert("large-1".to_string());
                            } else {
                                view.open_items.remove("large-1");
                            }
                            cx.notify();
                        });
                    }),
            )
            .into_any_element()
    }

    fn handle_toggle_events_accordion(&self, entity: Entity<Self>) -> AnyElement {
        let e = entity.clone();

        Accordion::new("toggle-events-accordion")
            .on_toggle_click(move |open_indices, _window, _cx| {
                println!("Open items: {:?}", open_indices);
                e.update(_cx, |view, cx| {
                    if open_indices.contains(&0) {
                        view.open_items.insert("toggle-1".to_string());
                    } else {
                        view.open_items.remove("toggle-1");
                    }
                    cx.notify();
                });
            })
            .item(|item| {
                item.title("Section 1")
                    .open(self.is_open("toggle-1"))
                    .child("Content 1")
            })
            .into_any_element()
    }

    fn disabled_state_accordion(&self, entity: Entity<Self>) -> AnyElement {
        let e = entity.clone();

        Accordion::new("disabled-accordion")
            .disabled(true)
            .item(|item| {
                item.title("Disabled Section")
                    .open(self.is_open("disabled-1"))
                    .child("Content")
            })
            .on_toggle_click(move |open_indices, _window, _cx| {
                e.update(_cx, |view, cx| {
                    if open_indices.contains(&0) {
                        view.open_items.insert("disabled-1".to_string());
                    } else {
                        view.open_items.remove("disabled-1");
                    }
                    cx.notify();
                });
            })
            .into_any_element()
    }

    fn with_custom_icons_accordion(&self, entity: Entity<Self>) -> AnyElement {
        let e = entity.clone();

        Accordion::new("custom-icons-accordion")
            .item(|item| {
                item.title(
                    h_flex()
                        .gap_2()
                        .child(Icon::new(IconName::Settings))
                        .child("Settings"),
                )
                .open(self.is_open("custom-1"))
                .child("Settings content here")
            })
            .on_toggle_click(move |open_indices, _window, _cx| {
                e.update(_cx, |view, cx| {
                    if open_indices.contains(&0) {
                        view.open_items.insert("custom-1".to_string());
                    } else {
                        view.open_items.remove("custom-1");
                    }
                    cx.notify();
                });
            })
            .into_any_element()
    }

    fn nested_accordions_accordion(&self, entity: Entity<Self>) -> AnyElement {
        let e_outer = entity.clone();
        let e_inner = entity.clone();

        Accordion::new("nested-outer")
            .item(|item| {
                item.title("Parent Section")
                    .open(self.is_open("nested-outer-1"))
                    .child(
                        Accordion::new("nested-inner")
                            .item(|item| {
                                item.title("Child 1")
                                    .open(self.is_open("nested-inner-1"))
                                    .child("Content")
                            })
                            .item(|item| {
                                item.title("Child 2")
                                    .open(self.is_open("nested-inner-2"))
                                    .child("Content")
                            })
                            .on_toggle_click(move |open_indices, _window, _cx| {
                                e_inner.update(_cx, |view, cx| {
                                    let keys = ["nested-inner-1", "nested-inner-2"];
                                    for (i, key) in keys.iter().enumerate() {
                                        if open_indices.contains(&i) {
                                            view.open_items.insert(key.to_string());
                                        } else {
                                            view.open_items.remove(*key);
                                        }
                                    }
                                    cx.notify();
                                });
                            }),
                    )
            })
            .on_toggle_click(move |open_indices, _window, _cx| {
                e_outer.update(_cx, |view, cx| {
                    if open_indices.contains(&0) {
                        view.open_items.insert("nested-outer-1".to_string());
                    } else {
                        view.open_items.remove("nested-outer-1");
                    }
                    cx.notify();
                });
            })
            .into_any_element()
    }
}
