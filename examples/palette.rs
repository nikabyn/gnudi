use std::env;

use color::{OpaqueColor, Srgb};
use gnudi::{
    App, Center, Full, Layer, LayerWindow, Palette, Parent, Rectangle, Row, Service, Text,
    Vertical, Widget,
};

// try `cargo run --example palette -- ~/Pictures/wall.jpg`, then overwrite that file
const DEFAULT_IMAGE: &str = "examples/wallpaper.png";

fn main() {
    let image = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from(DEFAULT_IMAGE));

    Palette::write().open(&image, 16);

    for color in Palette::read().colors() {
        println!("#{:08x}", color.to_rgba8().to_u32());
    }

    App::new().window(view).run();
}

fn view() -> LayerWindow {
    let palette = Palette::read();

    let mut swatches: Vec<Box<dyn Widget>> = Vec::new();

    for &color in palette.colors() {
        swatches.push(Box::new(swatch(color)));
    }

    let label = format!(
        "accent #{:08x}  background #{:08x}",
        palette.accent().to_rgba8().to_u32(),
        palette.background().to_rgba8().to_u32()
    );

    let accent = Rectangle::new()
        .width(260.0)
        .height(Parent)
        .fill(palette.accent())
        .child(
            Text::new(&label)
                .size(14.0)
                .color(palette.on_accent().with_alpha(1.0)),
        );

    swatches.push(Box::new(accent));

    LayerWindow::new()
        .width(Full)
        .height(40.0)
        .anchor_vertical(Vertical::Top)
        .layer(Layer::Top)
        .child(
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill(palette.background())
                .child(Row::new(swatches).justify(Center)),
        )
}

fn swatch(color: OpaqueColor<Srgb>) -> Rectangle {
    Rectangle::new().width(40.0).height(Parent).fill(color)
}
