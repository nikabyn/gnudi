use color::{AlphaColor, Srgb};

use crate::graphics::Area;

pub struct Shadow {
    pub(crate) color: AlphaColor<Srgb>,
    pub(crate) blur: f32,

    pub(crate) x: f32,
    pub(crate) y: f32,

    pub(crate) kind: Kind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Kind {
    // cast behind the rectangle, onto whatever is under it
    #[default]
    Drop,

    // along the inside edge, as if the rectangle were pressed in
    Inner,
}

impl Shadow {
    pub fn drop(color: impl Into<AlphaColor<Srgb>>) -> Self {
        Self::new(color.into(), Kind::Drop)
    }

    pub fn inner(color: impl Into<AlphaColor<Srgb>>) -> Self {
        Self::new(color.into(), Kind::Inner)
    }

    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.color = self.color.with_alpha(alpha);

        self
    }

    pub fn blur(mut self, amount: f32) -> Self {
        self.blur = amount;

        self
    }

    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;

        self
    }

    fn new(color: impl Into<AlphaColor<Srgb>>, kind: Kind) -> Self {
        Self {
            color: color.into(),
            blur: 0.0,
            x: 0.0,
            y: 0.0,
            kind,
        }
    }

    pub(crate) fn place(&self, area: Area) -> Area {
        Area::new(area.x + self.x, area.y + self.y, area.width, area.height)
    }
}
