use color::AlphaColor;
use gnudi::{App, Center, Column, LayerWindow, Rectangle, Stack, Text, children};

fn main() {
    App::new().window(view).run();
}

fn view() -> LayerWindow {
    // one edge: white letters with a black stroke
    let single = Text::new("stroke")
        .size(64.0)
        .color(AlphaColor::WHITE)
        // .stroke(6.0, AlphaColor::BLACK)
    ;

    /*
     * two edges: the same text three times, widest stroke first,
     * each stroke drawn in its own color with letters to match
     */
    let double = Stack::new(children![
        Text::new("double").size(64.0).color(AlphaColor::WHITE), // .stroke(16.0, AlphaColor::WHITE)
        Text::new("double").size(64.0).color(AlphaColor::BLACK), // .stroke(9.0, AlphaColor::BLACK)
        Text::new("double").size(64.0).color(AlphaColor::WHITE),
    ]);

    let words = Column::new(children![single, double]).gap(30.0);

    LayerWindow::new().width(420.0).height(260.0).child(
        Rectangle::new()
            .width(420.0)
            .height(260.0)
            .fill("#e5191c")
            .align_child(Center, Center)
            .child(words),
    )
}
