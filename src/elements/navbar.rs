use gpui::{div, prelude::*, relative, rems, svg};

use crate::theme::PigTheme;

pub struct PigNavbar {
    pub theme: PigTheme,
}

impl Render for PigNavbar {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut Context<'_, Self>,
    ) -> impl IntoElement {
        div()
            .w(relative(1.))
            .h(rems(4.))
            .pl(rems(0.8))
            .flex()
            .flex_row()
            .justify_start()
            .items_center()
            .child(
                svg()
                    .path("icons/home.svg")
                    .size(rems(2.2))
                    .text_color(self.theme.gray()),
            )
    }
}
