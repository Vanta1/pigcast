// TODO: remove
#![allow(dead_code)]

use gpui::{Rems, Rgba, rems, rgb};

#[derive(Copy, Clone)]
pub struct PigTheme {
    pub bg: Rgba, // bg0
    pub bg1: Rgba,
    pub bg2: Rgba,
    pub fg: Rgba,
    pub gray: Rgba,
    pub border_thickness: Rems,
}

impl PigTheme {
    pub fn bg(&self) -> Rgba {
        self.bg
    }

    pub fn bg1(&self) -> Rgba {
        self.bg1
    }

    pub fn bg2(&self) -> Rgba {
        self.bg2
    }

    pub fn fg(&self) -> Rgba {
        self.fg
    }

    pub fn gray(&self) -> Rgba {
        self.gray
    }

    pub fn border_thickness(&self) -> Rems {
        self.border_thickness
    }
}

pub fn default_theme() -> PigTheme {
    everforest_dark_hard()
}

fn everforest_dark_hard() -> PigTheme {
    PigTheme {
        bg: rgb(0x272e33),
        bg1: rgb(0x2e383c),
        bg2: rgb(0x374145),
        fg: rgb(0xd3c6aa),
        gray: rgb(0x859289),
        border_thickness: rems(0.2),
    }
}
