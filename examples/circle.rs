use gnudi::{App, Full, LayerWindow, Rectangle, Row, children};

fn main() {
    App::new().window(view).run();
}

fn view() -> LayerWindow {
    LayerWindow::new()
        .width(400.0)
        .height(120.0)
        .child(Row::new(children![
            Rectangle::new()
                .width(100.0)
                .height(100.0)
                .fill(color::palette::css::RED)
                .radius(Full),
            Rectangle::new()
                .width(200.0)
                .height(40.0)
                .fill(color::palette::css::BLUE)
                .radius(Full),
            Rectangle::new()
                .width(80.0)
                .height(80.0)
                .fill(color::palette::css::GREEN)
                .radius(12.0),
        ]))
}
