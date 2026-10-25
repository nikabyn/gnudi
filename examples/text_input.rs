use color::AlphaColor;
use gnudi::{App, Column, Keyboard, LayerWindow, Rectangle, TextInput, children};

fn main() {
    App::new().window(view).run();
}

fn view() -> LayerWindow {
    // without keyboard focus the window never receives keys
    LayerWindow::new()
        .width(320.0)
        .height(80.0)
        .keyboard(Keyboard::OnDemand)
        .child(Column::new(children![
            Rectangle::new()
                .width(320.0)
                .height(40.0)
                .fill(AlphaColor::WHITE)
                .child(
                    TextInput::new("name")
                        .size(20.0)
                        .placeholder("your name")
                        .on_change(|text| println!("name: {text}"))
                        .on_submit(|text| println!("hello, {text}"))
                ),
            Rectangle::new()
                .width(320.0)
                .height(40.0)
                .fill(AlphaColor::from_rgb8(230, 230, 230))
                .child(
                    TextInput::new("password")
                        .size(20.0)
                        .placeholder("password")
                        .password()
                        .on_submit(|_| println!("password submitted"))
                ),
        ]))
}
