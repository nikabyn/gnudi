use gnudi::{
    App, Center, Column, End, LayerWindow, Parent, Rectangle, Row, SpaceBetween, SpaceEvenly,
    children,
};

fn main() {
    App::new().window(view).run();
}

fn view() -> LayerWindow {
    LayerWindow::new().width(400.0).height(400.0).child(
        Column::new(children![
            Row::new(children![
                Rectangle::new()
                    .width(60.0)
                    .height(60.0)
                    .fill(color::palette::css::RED),
                Rectangle::new()
                    .width(60.0)
                    .height(40.0)
                    .fill(color::palette::css::GREEN),
                Rectangle::new()
                    .width(60.0)
                    .height(20.0)
                    .fill(color::palette::css::BLUE),
            ])
            .width(Parent)
            .height(100.0)
            .justify(SpaceBetween)
            .align(Center),
            Row::new(children![
                Rectangle::new()
                    .width(60.0)
                    .height(60.0)
                    .fill(color::palette::css::RED),
                Rectangle::new()
                    .width(60.0)
                    .height(40.0)
                    .fill(color::palette::css::GREEN),
            ])
            .width(Parent)
            .height(100.0)
            .justify(SpaceEvenly)
            .align(End),
        ])
        .width(Parent)
        .height(Parent)
        .justify(Center),
    )
}
