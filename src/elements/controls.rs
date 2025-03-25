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
        let icon_size = rems(2.);
        let icon_color = self.theme.gray();

        div()
            .h(rems(4.))
            .w(DefiniteLength::Fraction(1.))
            .flex()
            .flex_row()
            .justify_center()
            .items_center()
            .border_t(rems(0.2))
            .border_color(self.theme.bg1())
            .child(
                svg()
                    .path("icons/backward.svg")
                    .size(icon_size - rems(0.0))
                    .text_color(icon_color),
            )
            .child(
                svg()
                    .ml(rems(0.6))
                    .mr(rems(0.5))
                    .path("icons/play.svg")
                    .size(icon_size + rems(0.0))
                    .text_color(icon_color),
            )
            .child(
                svg()
                    .path("icons/forward.svg")
                    .size(icon_size - rems(0.0))
                    .text_color(icon_color),
            )
    }
}
