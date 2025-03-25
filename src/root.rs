use gpui::{div, prelude::*, relative};

use crate::elements::{PigControls, PigNavbar};
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
        let navbar = cx.new(|_| PigNavbar { theme: self.theme });

        div()
            .flex()
            .flex_col()
            .size(relative(1.)) // fill the whole window
            .bg(self.theme.bg())
            .m_0() // remove margins
            .p_0() // padding
            .font_family("Monaspace Xenon")
            .text_color(self.theme.fg())
            .child(navbar)
            .child(
                div() // placeholder for main view
                    .flex()
                    .flex_grow()
                    .border_y(self.theme.border_thickness())
                    .border_color(self.theme.bg1()),
            )
            .child(controls)
    }
}
