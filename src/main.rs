#![allow(non_snake_case)]

use dioxus::{
    desktop::{Config, WindowBuilder},
    prelude::*,
};

fn main() {
    let window = WindowBuilder::new()
        .with_title("Pigcast")
        .with_decorations(false);

    let style = include_str!("../res/style.css");
    let head = format!(
        r#"
            <title>Pigcast</title>
            <style>{style}</style>
        "#
    );

    let config = Config::new().with_window(window).with_custom_head(head);

    dioxus::LaunchBuilder::new().with_cfg(config).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        p { "Hello porld (porb world)" }
    }
}
