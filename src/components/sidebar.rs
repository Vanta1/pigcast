use dioxus::prelude::*;

use crate::config::Config;

#[component]
pub fn Sidebar() -> Element {
    let config = use_context::<Config>();

    let feeds: Vec<(&String, &toml::Table)> = config.data()["feed"]
        .as_table()
        .unwrap()
        .iter()
        .map(|f| (f.0, f.1.as_table().unwrap()))
        .collect();

    rsx! {
        div {
            class: "sidebar",
            div {
                class: "sidebar-feeds",
                for feed in feeds {
                    div {
                        key: "{feed.0}",
                        "{feed.0}"
                    }
                }
            }
        }
    }
}
