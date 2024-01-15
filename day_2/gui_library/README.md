---
minutes: 30
---

# Exercise: GUI Library

Let us design a classical GUI library using our new knowledge of traits and
trait objects. We'll only implement the drawing of it (as text) for simplicity.

We will have a number of widgets in our library:

- `Window`: has a `title` and contains other widgets.
- `Button`: has a `label`. In reality, it would also take a callback function to
  allow the program to do something when the button is clicked but we won't
  include that since we're only drawing the GUI.
- `Label`: has a `label`.

The widgets will implement a `Widget` trait, see below.

Copy the code below to <https://play.rust-lang.org/>, fill in the missing
`draw_into` methods so that you implement the `Widget` trait:

```rust
// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label { label: label.to_owned() }
    }
}

pub struct Button {
    label: Label,
}

impl Button {
    fn new(label: &str) -> Button {
        Button { label: Label::new(label) }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window { title: title.to_owned(), widgets: Vec::new() }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    fn inner_width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }
}

// TODO: Implement `Widget` for `Label`.

// TODO: Implement `Widget` for `Button`.

// TODO: Implement `Widget` for `Window`.

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new("Click me!")));
    window.draw();
}
```

The output of the above program can be something simple like this:

```text
========
Rust GUI Demo 1.23
========

This is a small text GUI demo.

| Click me! |
```

If you want to draw aligned text, you can use the
[fill/alignment](https://doc.rust-lang.org/std/fmt/index.html#fillalignment)
formatting operators. In particular, notice how you can pad with different
characters (here a `'/'`) and how you can control alignment:

```rust,editable
fn main() {
    let width = 10;
    println!("left aligned:  |{:/<width$}|", "foo");
    println!("centered:      |{:/^width$}|", "foo");
    println!("right aligned: |{:/>width$}|", "foo");
}
```

Using such alignment tricks, you can for example produce output like this:

```text
+--------------------------------+
|       Rust GUI Demo 1.23       |
+================================+
| This is a small text GUI demo. |
| +-----------+                  |
| | Click me! |                  |
| +-----------+                  |
+--------------------------------+
```
