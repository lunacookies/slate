use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), scale.chroma(), 265.0)
    }

    pub(crate) fn red(&self) -> Oklch {
        oklch(0.6061125, 0.120573945, 15.32223)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(0.69291884, 0.09627467, 38.219067)
    }

    pub(crate) fn brown(&self) -> Oklch {
        oklch(0.62193996, 0.06850407, 40.603855)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(0.8548697, 0.08916313, 84.09701)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.7682698, 0.07485832, 131.1036)
    }

    pub(crate) fn cyan(&self) -> Oklch {
        oklch(0.74973196, 0.033805843, 194.1465)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.70091945, 0.033435494, 248.33371)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.6920739, 0.06252046, 332.61578)
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
