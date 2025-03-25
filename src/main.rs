use std::{fs, path::PathBuf};

use gpui::{App, Application, AssetSource, SharedString, WindowOptions, prelude::*};

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

fn main() {
    Application::new()
        .with_assets(PigResources {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res"),
        })
        .run(|cx: &mut App| {
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
