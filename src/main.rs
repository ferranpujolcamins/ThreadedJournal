use dioxus::prelude::*;
use dioxus_desktop::Config;
use fermi::prelude::*;
use chrono::Duration;

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
                duration: Duration::hours(1) + Duration::minutes(15)
            }],
        },
        Entry {
            items: vec![Item {
                text: "Also today".to_string(),
                duration: Duration::minutes(30)
            }],
        },
    ]
};

fn app(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let entries = use_read(cx.scope, ENTRIES);
    cx.render(rsx!(
        div {
            class: "flex items-center flex-col",
            entries.iter().map(|e| rsx!(Entry { entry: e }))
        }
    ))
}

fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::new()
            .with_custom_head("<script src=\"https://cdn.tailwindcss.com\"></script>".to_string()),
    );
}
