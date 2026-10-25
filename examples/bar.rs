use color::AlphaColor;
use gnudi::{App, Full, Layer, LayerWindow, Parent, Rectangle, Text, Vertical};

fn main() {
    App::new().window(view).run();
}

fn view() -> LayerWindow {
    LayerWindow::new()
        .width(Full)
        .height(30.0)
        .anchor_vertical(Vertical::Top)
        .layer(Layer::Top)
        .child(
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill(color::palette::css::BLUE)
                .child(Text::new("Amane bar").size(20.0).color(AlphaColor::WHITE)),
        )
}
