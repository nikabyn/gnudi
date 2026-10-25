mod arc;
mod circle;
mod line;
mod path;
mod round;

use crate::graphics::{self, Cap};

pub use arc::Arc;
pub use circle::Circle;
use color::{AlphaColor, Srgb};
pub use line::Line;
pub use path::Path;

pub struct Style {
    pub(crate) fill: AlphaColor<Srgb>,

    pub(crate) stroke_thickness: f32,
    pub(crate) stroke_color: AlphaColor<Srgb>,
    pub(crate) cap: Cap,

    pub(crate) opacity: f32,
}

// something a canvas draws, every shape gets the same builder methods for how it looks
pub trait Shape {
    // the outline in the canvas's own coordinates, for a canvas of this size
    fn trace(&self, width: f32, height: f32) -> Option<graphics::BezierPath>;

    fn style(&self) -> &Style;

    fn style_mut(&mut self) -> &mut Style;

    fn fill(mut self, color: AlphaColor<Srgb>) -> Self
    where
        Self: Sized,
    {
        self.style_mut().fill = color;

        self
    }

    fn stroke(mut self, thickness: f32, color: AlphaColor<Srgb>) -> Self
    where
        Self: Sized,
    {
        self.style_mut().stroke_thickness = thickness;
        self.style_mut().stroke_color = color.into();

        self
    }

    fn cap(mut self, cap: Cap) -> Self
    where
        Self: Sized,
    {
        self.style_mut().cap = cap;

        self
    }

    fn opacity(mut self, opacity: f32) -> Self
    where
        Self: Sized,
    {
        self.style_mut().opacity = opacity;

        self
    }
}

// a new shape draws nothing until it gets a fill or a stroke
impl Default for Style {
    fn default() -> Self {
        Self {
            fill: AlphaColor::TRANSPARENT,

            stroke_thickness: 0.0,
            stroke_color: AlphaColor::TRANSPARENT,
            cap: Cap::Butt,

            opacity: 1.0,
        }
    }
}
