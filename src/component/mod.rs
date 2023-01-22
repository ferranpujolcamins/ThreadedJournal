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

pub fn NewEntryButton(cx: Scope) -> Element {
    cx.render(rsx! {
        button {
            div {
                "+"
            }
        }
    })
}

#[inline_props]
pub fn Item<'a>(cx: Scope, item: &'a UseRef<model::Item>) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-row gap-2 divide-x",
            div { "{item.read().format()}"}
            div {
                textarea {
                    rows: "10",
                    cols: "80",
                    value: "{item.read().text}",
                    oninput: move |event| {
                        item.write().text = event.value.clone();
                    },
                }
            }
        }
    })
}

#[inline_props]
pub fn Entry<'a>(cx: Scope, entry: &'a UseRef<model::Entry>) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-col gap-2",
            oncontextmenu: |_| {
                panic!() // TODO: 
            },
            entry.read().date.format("%d-%m-%Y").to_string()
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
