use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), scale.chroma(), 265.0)
    }

    pub(crate) fn red(&self) -> Oklch {
        oklch(0.65, 0.1, 15.0)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(0.7, 0.07, 40.0)
    }

    pub(crate) fn brown(&self) -> Oklch {
        oklch(0.6, 0.04, 40.0)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(0.85, 0.07, 85.0)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.7, 0.07, 130.0)
    }

    pub(crate) fn cyan(&self) -> Oklch {
        oklch(0.7, 0.04, 195.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.7, 0.04, 250.0)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.7, 0.07, 335.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    DarkerBg,
    DarkBg,
    Bg,
    LightBg,
    DarkerFg,
    DarkFg,
    Fg,
    BrightFg,
}

impl BaseScale {
    fn lightness(self) -> f32 {
        lerp(self.value(), 0.24..0.9)
    }

    fn chroma(self) -> f32 {
        match self {
            Self::DarkFg | Self::Fg | Self::BrightFg => 0.017,
            _ => lerp(self.value(), 0.018..0.03),
        }
    }

    fn value(self) -> f32 {
        match self {
            Self::DarkerBg => 0.0,
            Self::DarkBg => 0.03,
            Self::Bg => 0.06,
            Self::LightBg => 0.09,
            Self::DarkerFg => 0.5,
            Self::DarkFg => 0.6,
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
