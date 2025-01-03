use dioxus::prelude::*;

use crate::config::Config;

#[component]
pub fn Sidebar(config: Signal<Config>) -> Element {
    let config = config();

    // for future reference no touch, this could be really useful for dynamically changing colors
    //document::eval(r#"document.documentElement.style.setProperty("--bg-color", "green");"#);

    rsx! {
        div {
            class: "sidebar",
            div {
                class: "sidebar-feeds",
                for feed in config.feeds() {
                    div {
                        key: "{feed.title}",
                        "{feed.title}"
                    }
                }
            }
        }
    }
}
