use gpui::{DefiniteLength, div, prelude::*};

use crate::elements::PigControls;
use crate::theme::PigTheme;

/// The root element of Pigcast
pub struct PigRoot {
    pub theme: PigTheme,
}

impl Render for PigRoot {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut Context<'_, Self>,
    ) -> impl IntoElement {
        let controls = cx.new(|_| PigControls { theme: self.theme });

        div()
            .flex()
            .flex_col()
            .size(DefiniteLength::Fraction(1.)) // fill the whole window
            .bg(self.theme.bg())
            .m_0() // remove margins
            .p_0() // padding
            .font_family("Monaspace Xenon")
            .text_color(self.theme.fg())
            .child(div().flex_grow())
            .child(controls)
    }
}
