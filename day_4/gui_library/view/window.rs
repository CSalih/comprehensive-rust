use crate::view::Widget;

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    pub fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    pub fn inner_width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
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
