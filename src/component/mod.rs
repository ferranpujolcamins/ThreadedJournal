#![allow(non_snake_case)]

use crate::model;
use dioxus::prelude::*;

#[inline_props]
pub fn Item<'a>(cx: Scope, item: &'a model::Item) -> Element {
    cx.render(rsx! {
        div {
            div { "{item.text}"}
        }
    })
}


#[inline_props]
pub fn Entry<'a>(cx: Scope, entry: &'a model::Entry) -> Element {
    cx.render(rsx! {
        div {
            entry.items.iter().map(|i| rsx!(
                div {
                    Item { item: i }
                }
            ))
        }
    })
}
