use dioxus::prelude::*;
use fermi::prelude::*;

#[derive(PartialEq)]
struct Item {
    text: String,
}

#[derive(PartialEq)]
struct Entry {
    items: Vec<Item>,
}

// TODO: AtomFamily instead?
static ENTRIES: Atom<Vec<Entry>> = |_| {
    vec![
        Entry {
            items: vec![Item {
                text: "I did something".to_string(),
            }],
        },
        Entry {
            items: vec![Item {
                text: "Also today".to_string(),
            }],
        },
    ]
};

#[inline_props]
fn item_element<'a>(cx: Scope, item: &'a Item) -> Element {
    cx.render(rsx! {
        div {
            div { "{item.text}"}
        }
    })
}

#[inline_props]
fn entry_element<'a>(cx: Scope, entry: &'a Entry) -> Element {
    cx.render(rsx! {
        div {
            entry.items.iter().map(|item| rsx!(
                div {
                    item_element { item: item }
                }
            ))
        }
    })
}

fn app(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let entries = use_read(cx.scope, ENTRIES);
    cx.render(rsx!(entries
        .iter()
        .map(|entry| rsx!(entry_element { entry: entry }))))
}

fn main() {
    dioxus_desktop::launch(app);
}
