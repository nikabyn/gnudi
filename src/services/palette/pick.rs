use color::{OpaqueColor, Srgb};

// wcag's minimum contrast for normal sized text
const READABLE: f32 = 4.5;

// how much a vivid color counts over one near middle brightness
const SATURATION_WEIGHT: f32 = 1.4;
const TARGET_LUMINANCE: f32 = 0.52;
const LUMINANCE_PENALTY: f32 = 0.35;

pub fn darkest(colors: &[OpaqueColor<Srgb>]) -> Option<OpaqueColor<Srgb>> {
    let mut darkest = *colors.first()?;

    for &color in colors {
        if color.relative_luminance() < darkest.relative_luminance() {
            darkest = color;
        }
    }

    Some(darkest)
}

// saturated but not too dark or too bright, the color that stands out
pub fn most_vivid(colors: &[OpaqueColor<Srgb>]) -> Option<OpaqueColor<Srgb>> {
    let mut vivid = *colors.first()?;
    let mut vivid_score = f32::MIN;

    for &color in colors {
        let saturation = saturation(color) * SATURATION_WEIGHT;
        let distance = (color.relative_luminance() - TARGET_LUMINANCE).abs() * LUMINANCE_PENALTY;

        let score = saturation - distance;

        if score > vivid_score {
            vivid = color;
            vivid_score = score;
        }
    }

    Some(vivid)
}

/**
 * the palette color with the most contrast keeps the image's tint,
 * but only if it's readable, otherwise plain white or black
 */
pub fn readable_on(
    background: OpaqueColor<Srgb>,
    colors: &[OpaqueColor<Srgb>],
) -> OpaqueColor<Srgb> {
    let mut best = background;
    let mut best_contrast = 1.0;

    for &color in colors {
        let color_contrast = contrast(color, background);

        if color_contrast > best_contrast {
            best = color;
            best_contrast = color_contrast;
        }
    }

    if best_contrast < READABLE {
        return plain_on(background);
    }

    best
}

/// white or black, whichever stands out more
pub fn plain_on(background: OpaqueColor<Srgb>) -> OpaqueColor<Srgb> {
    if contrast(OpaqueColor::WHITE, background) >= contrast(OpaqueColor::BLACK, background) {
        return OpaqueColor::WHITE;
    }

    OpaqueColor::BLACK
}

/// 1 for the same brightness, up to 21 for black on white
pub fn contrast(first: OpaqueColor<Srgb>, second: OpaqueColor<Srgb>) -> f32 {
    let first = first.relative_luminance();
    let second = second.relative_luminance();

    let lighter = first.max(second);
    let darker = first.min(second);

    (lighter + 0.05) / (darker + 0.05)
}

/**
 * lightness as the eye judges it, 0 for black to 100 for white, with
 * middle grey at 50; luminance is light as measured, which the eye
 * sees on a curve, so a pale color's luminance looks low
 */
pub fn lightness(color: OpaqueColor<Srgb>) -> f32 {
    let luminance = color.relative_luminance();

    // very dark colors are on a straight line, the rest on a cube root
    if luminance <= 216.0 / 24389.0 {
        return luminance * 24389.0 / 27.0;
    }

    116.0 * luminance.cbrt() - 16.0
}

fn saturation(color: OpaqueColor<Srgb>) -> f32 {
    let [r, g, b] = color.components;

    let highest = r.max(g).max(b);
    let lowest = r.min(g).min(b);

    highest - lowest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lightness_matches_the_eye() {
        assert!(lightness(OpaqueColor::from_rgb8(0, 0, 0)) < 0.1);
        assert!((lightness(OpaqueColor::from_rgb8(255, 255, 255)) - 100.0).abs() < 0.1);

        // middle grey sits at about half
        assert!((lightness(OpaqueColor::from_rgb8(119, 119, 119)) - 50.0).abs() < 1.0);

        // a pale lilac from a light wallpaper: low luminance, but plainly light
        let lilac = OpaqueColor::from_rgb8(173, 157, 201);

        assert!(lilac.relative_luminance() < 0.5);
        assert!(lilac.relative_luminance() > 60.0);
    }
}
