use dioxus::{desktop::WindowBuilder, prelude::*};

mod components;
mod config;
mod error;

use components::sidebar::Sidebar;
use config::Config;

fn main() {
    let config = {
        let window = WindowBuilder::new()
            .with_title("Pigcast")
            .with_decorations(false);

        // TODO: Investigate asset! macro not working
        let style = include_str!("../res/style.css");
        let head = format!(
            r#"
                <title>Pigcast</title>
                <style>{style}</style>
            "#
        );

        dioxus::desktop::Config::new()
            .with_window(window)
            .with_custom_head(head)
    };

    dioxus::LaunchBuilder::new().with_cfg(config).launch(App);
}

#[component]
fn App() -> Element {
    let config =
        use_signal(|| Config::from_file("./res/conig.toml".to_string()).unwrap_or_default());

    rsx! {
        Sidebar { config: config }
    }
}
