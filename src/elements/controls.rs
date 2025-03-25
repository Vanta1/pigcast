use gpui::{DefiniteLength, div, prelude::*, rems, svg};

use crate::theme::PigTheme;

/// bottom controls
pub struct PigControls {
    pub theme: PigTheme,
}

impl Render for PigControls {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<'_, Self>,
    ) -> impl IntoElement {
        div()
            .h(rems(3.))
            .w(DefiniteLength::Fraction(1.))
            .flex()
            .justify_center()
            .border_t_1()
            .border_color(self.theme.bg1())
            .child(svg().path("icons/play.svg").size_10())
    }
}
