use std::{fs, path::PathBuf};

use gpui::{
    Application, AssetSource, KeyBinding, SharedString, WindowOptions, actions, prelude::*,
};

mod elements;
mod root;
mod theme;

use crate::root::PigRoot;

struct PigResources {
    base: PathBuf,
}

impl AssetSource for PigResources {
    fn load(&self, path: &str) -> gpui::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<gpui::SharedString>> {
        fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

actions!(pig, [Quit]);

fn main() {
    env_logger::init();

    Application::new()
        .with_assets(PigResources {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res"),
        })
        .run(|cx| {
            let theme = crate::theme::default_theme();

            cx.activate(true);
            cx.on_action(|_: &Quit, cx| cx.quit());
            cx.bind_keys([
                // KeyBinding::new("q", Quit, None), // convenient for testing
                KeyBinding::new("ctrl-c", Quit, None),
            ]);

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
