use dioxus::prelude::*;
use fermi::prelude::*;

mod model;
use model::*;

mod component;
use component::*;

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

fn app(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let entries = use_read(cx.scope, ENTRIES);
    cx.render(rsx!(entries.iter().map(|e| rsx!(Entry { entry: e }))))
}

fn main() {
    dioxus_desktop::launch(app);
}
