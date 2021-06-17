use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), scale.chroma(), 265.0)
    }

    pub(crate) fn red(&self) -> Oklch {
        oklch(0.6, 0.12, 15.0)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(0.7, 0.1, 40.0)
    }

    pub(crate) fn brown(&self) -> Oklch {
        oklch(0.6, 0.07, 40.0)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(0.85, 0.09, 85.0)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.75, 0.07, 130.0)
    }

    pub(crate) fn cyan(&self) -> Oklch {
        oklch(0.77, 0.03, 195.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.7, 0.03, 250.0)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.7, 0.06, 335.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    Bg,
    Fg,
    BrightFg,
}

impl BaseScale {
    fn lightness(self) -> f32 {
        lerp(self.value(), 0.27..0.9)
    }

    fn chroma(self) -> f32 {
        lerp(self.value(), 0.018..0.017)
    }

    fn value(self) -> f32 {
        match self {
            Self::Bg => 0.0,
            Self::Fg => 0.8,
            Self::BrightFg => 1.0,
        }
    }
}

fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h).unwrap(),
    }
}

fn lerp(x: f32, range: Range<f32>) -> f32 {
    x * (range.end - range.start) + range.start
}
