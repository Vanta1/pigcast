#![allow(non_snake_case)]

use dioxus::{
    desktop::{use_window, Config, WindowBuilder},
    prelude::*,
};
use document::Stylesheet;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    let window = WindowBuilder::new()
        .with_title("Pigcast")
        .with_decorations(false);
    let config = Config::new().with_window(window);
    dioxus::LaunchBuilder::new().with_cfg(config).launch(App);
}

#[component]
fn App() -> Element {
    use_window().set_decorations(false);
    rsx! {
        Stylesheet { href: CSS }
        p { "Hello porld (porb world)" }
    }
}
