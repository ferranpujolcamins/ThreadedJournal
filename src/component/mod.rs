#![allow(non_snake_case)]

use crate::model;
use chrono::Duration;
use dioxus::prelude::*;
use hhmmss::Hhmmss;

impl model::Item {
    fn format(&self) -> String {
        self.duration.hhmmss()
    }
}

#[inline_props]
pub fn Item<'a>(cx: Scope, item: &'a UseRef<model::Item>) -> Element {
    cx.render(rsx! {
        div {
            div {
                textarea {
                    class: "border-black dark:border-slate-600 bg-white dark:bg-slate-700 text-black dark:text-white", // TODO: we are repeating dark mode and theme info here
                    rows: "10",
                    cols: "80",
                    value: "{item.read().text}",
                    oninput: move |event| {
                        item.write().text = event.value.clone();
                    },
                }
            }
            div { "{item.read().format()}"}
        }
    })
}

#[inline_props]
pub fn Entry<'a>(cx: Scope, entry: &'a UseRef<model::Entry>) -> Element {
    cx.render(rsx! {
        div {
            class: "shadow-md rounded dark:bg-slate-700",
            entry.read().items.iter().map(|i| {
                let itemRef = use_ref(cx, || i.clone());
                rsx!(
                    div {
                        class: "p-2",
                        Item { item: itemRef }
                    }
                )
            })
        }
    })
}
