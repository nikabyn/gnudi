use color::{AlphaColor, Srgb};

// colors that blend into each other, each stop is a spot from 0 to 1 and its color
#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {
    // 0 degrees runs bottom to top, 90 runs left to right
    Linear {
        angle: f32,
        stops: Vec<(f32, AlphaColor<Srgb>)>,
    },

    // from the center out to the corners
    Radial {
        stops: Vec<(f32, AlphaColor<Srgb>)>,
    },
}

impl Gradient {
    pub fn linear<C: Into<AlphaColor<Srgb>>>(
        angle: f32,
        stops: impl IntoIterator<Item = (f32, C)>,
    ) -> Self {
        Self::Linear {
            angle,
            stops: collect(stops),
        }
    }

    pub fn radial<C: Into<AlphaColor<Srgb>>>(stops: impl IntoIterator<Item = (f32, C)>) -> Self {
        Self::Radial {
            stops: collect(stops),
        }
    }
}

// the stops can be plain hex strings like "#1e1e2e" as well as colors
fn collect<C: Into<AlphaColor<Srgb>>>(
    stops: impl IntoIterator<Item = (f32, C)>,
) -> Vec<(f32, AlphaColor<Srgb>)> {
    let mut collected = Vec::new();

    for (offset, color) in stops {
        collected.push((offset, color.into()));
    }

    collected
}
