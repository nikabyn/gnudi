use color::{AlphaColor, OpaqueColor, Srgb};

use crate::{Gradient, Image};

pub enum Fill {
    Color(AlphaColor<Srgb>),

    Gradient(Gradient),

    Image(Image),

    /// paints nothing and cuts its shape out of the rectangle holding it
    Mask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mask;

impl From<OpaqueColor<Srgb>> for Fill {
    fn from(color: OpaqueColor<Srgb>) -> Self {
        Self::Color(color.with_alpha(1.0))
    }
}

impl From<AlphaColor<Srgb>> for Fill {
    fn from(color: AlphaColor<Srgb>) -> Self {
        Self::Color(color)
    }
}

impl From<&str> for Fill {
    /// a string is a hex color like "#1e1e2e"
    fn from(hex: &str) -> Self {
        Self::Color(AlphaColor::from_hex(hex))
    }
}

impl From<Gradient> for Fill {
    fn from(gradient: Gradient) -> Self {
        Self::Gradient(gradient)
    }
}

impl From<super::Image> for Fill {
    fn from(image: Image) -> Self {
        Self::Image(image)
    }
}

impl From<Mask> for Fill {
    fn from(_: Mask) -> Self {
        Self::Mask
    }
}
