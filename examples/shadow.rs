use color::AlphaColor;
use gnudi::{App, Column, LayerWindow, Parent, Rectangle, Row, Shadow, children};

fn main() {
    App::new().window(view).run();
}

fn view() -> LayerWindow {
    LayerWindow::new().width(440.0).height(200.0).child(
        Rectangle::new()
            .width(Parent)
            .height(Parent)
            .fill("#cdd6f4")
            .child(Column::new(children![
                Rectangle::new().width(Parent).height(40.0),
                Row::new(children![
                    Rectangle::new().width(40.0).height(120.0),
                    Rectangle::new()
                        .width(160.0)
                        .height(120.0)
                        .radius(20.0)
                        .fill(AlphaColor::WHITE)
                        .shadow(
                            Shadow::drop(AlphaColor::BLACK)
                                .with_alpha(0.3)
                                .blur(12.0)
                                .offset(0.0, 4.0)
                        ),
                    Rectangle::new().width(40.0).height(120.0),
                    Rectangle::new()
                        .width(160.0)
                        .height(120.0)
                        .radius(20.0)
                        .fill(AlphaColor::WHITE)
                        .shadow(Shadow::inner(AlphaColor::BLACK).with_alpha(0.3).blur(8.0)),
                ]),
            ])),
    )
}
