use gpui::{App, Application, WindowOptions, prelude::*};

mod elements;
mod root;
mod theme;

use crate::root::PigRoot;

fn main() {
    Application::new().run(|cx: &mut App| {
        let theme = crate::theme::default_theme();

        cx.open_window(
            WindowOptions {
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("test".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| PigRoot { theme }),
        )
        .unwrap();
    });
}
