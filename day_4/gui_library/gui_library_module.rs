use crate::component::{Button, Label};
use crate::view::{Widget, Window};

mod component;
mod view;

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new(
        "The code is copied from exercise gui_library.",
    )));
    window.add_widget(Box::new(Button::new("Click me!")));
    window.draw();
}
