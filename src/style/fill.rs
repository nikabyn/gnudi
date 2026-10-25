use color::{AlphaColor, Srgb};

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

impl From<AlphaColor<Srgb>> for Fill {
    fn from(color: AlphaColor<Srgb>) -> Self {
        Self::Color(color)
    }
}

impl TryFrom<&str> for Fill {
    type Error = color::ParseError;

    /// a string is a hex color like "#1e1e2e"
    fn try_from(hex: &str) -> Result<Self, Self::Error> {
        AlphaColor::try_from_hex(hex).map(Self::Color)
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
