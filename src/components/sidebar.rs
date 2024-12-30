use dioxus::prelude::*;

use crate::config::Config;

#[component]
pub fn Sidebar(config: Signal<Config>) -> Element {
    let config = config();

    let feeds = config.feeds();

    rsx! {
        div {
            class: "sidebar",
            div {
                class: "sidebar-feeds",
                for feed in feeds {
                    div {
                        key: "{feed.title}",
                        "{feed.title}"
                    }
                }
            }
        }
    }
}
