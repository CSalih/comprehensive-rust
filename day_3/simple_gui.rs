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
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
}

impl Button {
    fn new(label: &str) -> Button {
        Button {
            label: Label::new(label),
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
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

impl Widget for Label {
    fn width(&self) -> usize {
        self.label.len()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        writeln!(buffer, "{}", &self.label).unwrap();
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        // Adding a padding of 2 makes it look nicer.
        let width = self.width() + 2;

        let mut local_buffer = String::new();
        self.label.draw_into(&mut local_buffer);

        // TODO: we may replace this with fill character: https://doc.rust-lang.org/std/fmt/index.html#fillalignment
        let border_content = std::iter::repeat("-").take(width).collect::<String>();
        writeln!(buffer, "+{}+", border_content).unwrap();
        local_buffer.lines().for_each(|line| {
            writeln!(buffer, "|{:^width$}|", &line).unwrap();
        });
        writeln!(buffer, "+{}+", border_content).unwrap();
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        self.inner_width()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        // Adding a padding of 2 makes it look nicer.
        let width = self.width() + 2;

        // TODO: we may replace this with https://doc.rust-lang.org/std/fmt/index.html#fillalignment
        let border_content = std::iter::repeat("-")
            .take(self.width() + 2)
            .collect::<String>();

        writeln!(buffer, "+{}+", border_content).unwrap();
        writeln!(buffer, "{}", format!("|{:^width$}|", self.title)).unwrap();

        if !self.widgets.is_empty() {
            let border = std::iter::repeat("=")
                .take(self.width() + 2)
                .collect::<String>();
            writeln!(buffer, "+{:^width$}+", border).unwrap();
        }
        self.widgets.iter().for_each(|w| {
            let mut local_buffer = String::new();
            w.draw_into(&mut local_buffer);

            local_buffer.lines().for_each(|line| {
                writeln!(buffer, "|{:^width$}|", line).unwrap();
            });
        });
        writeln!(buffer, "+{:^width$}+", border_content).unwrap();
    }
}

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new("Click me!")));
    window.draw();
}
