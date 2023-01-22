use chrono::Duration;
use dioxus::prelude::*;
use dioxus_desktop::Config;
use fermi::prelude::*;

mod model;
use model::*;

mod component;
use component::*;

// TODO: AtomFamily in the future
static ENTRIES: Atom<Vec<Entry>> = |_| {
    vec![
        Entry {
            date: chrono::NaiveDate::from_ymd_opt(2023, 01, 15).unwrap(),
            items: vec![Item {
                text: "I did something".to_string(),
                duration: Duration::hours(1) + Duration::minutes(15),
            }],
        },
        Entry {
            date: chrono::NaiveDate::from_ymd_opt(2023, 01, 12).unwrap(),
            items: vec![Item {
                text: "Also today".to_string(),
                duration: Duration::minutes(30),
            }],
        },
    ]
};

fn app(cx: Scope) -> Element {
    use_init_atom_root(cx);
    let entries = use_read(cx.scope, ENTRIES);
    let dark_mode = true;
    let stylesheet = if dark_mode {
        include_str!("../themes/default-dark.css")
    } else {
        include_str!("../themes/default.css")
    };
    cx.render(rsx!(
        style { [rsx!{stylesheet}].into_iter() }
        div {
            class: "min-h-screen flex items-center flex-col gap-2",
            entries.iter().map(|e| {
                let entry_ref = use_ref(cx, || e.clone());
                rsx!(Entry { entry: entry_ref })
            })
        }
    ))
}

fn main() {
    dioxus_desktop::launch_cfg(
        app,
        Config::new()
            // TODO: use local file
            .with_custom_head("<script src=\"https://cdn.tailwindcss.com\"></script>".to_string())
            .with_disable_context_menu(!cfg!(debug_assertions)),
    );
}
