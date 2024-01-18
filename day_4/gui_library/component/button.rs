use crate::component::label::Label;
use crate::view::Widget;

pub struct Button {
    label: Label,
}

impl Button {
    pub fn new(label: &str) -> Button {
        Button {
            label: Label::new(label),
        }
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
