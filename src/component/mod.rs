#![allow(non_snake_case)]

use crate::model;
use dioxus::prelude::*;
use hhmmss::Hhmmss;

impl model::Item {
    fn format(&self) -> String {
        self.duration.hhmmss()
    }
}

#[inline_props]
pub fn Item<'a>(cx: Scope, item: &'a model::Item) -> Element {
    cx.render(rsx! {
        div {
            div { "{item.text}"}
            div { "{item.format()}"}
        }
    })
}

#[inline_props]
pub fn Entry<'a>(cx: Scope, entry: &'a model::Entry) -> Element {
    cx.render(rsx! {
        div {
            class: "shadow-md rounded dark:bg-slate-700",
            entry.items.iter().map(|i| rsx!(
                div {
                    class: "p-2",
                    Item { item: i }
                }
            ))
        }
    })
}
