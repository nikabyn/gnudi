use color::AlphaColor;
use gnudi::{
    App, Grab, Keyboard, LayerWindow, NotAllowed, Parent, Pointer, Rectangle, ResizeHorizontal,
    Row, TextInput, children,
};

fn main() {
    App::new().window(view).run();
}

fn view() -> LayerWindow {
    LayerWindow::new()
        .width(600.0)
        .height(120.0)
        .keyboard(Keyboard::OnDemand)
        .child(Row::new(children![
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill(color::palette::css::RED)
                .cursor(Pointer),
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill(color::palette::css::GREEN)
                .cursor(Grab),
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill(color::palette::css::BLUE)
                .cursor(ResizeHorizontal),
            Rectangle::new()
                .width(Parent)
                .height(Parent)
                .fill(AlphaColor::BLACK)
                .cursor(NotAllowed),
            // a text input shows the text cursor on its own
            Rectangle::new()
                .width(150.0)
                .height(Parent)
                .fill(AlphaColor::WHITE)
                .child(TextInput::new("cursor").width(150.0))
        ]))
}
