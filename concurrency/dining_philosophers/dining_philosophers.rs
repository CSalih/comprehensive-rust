use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::Sender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    #[allow(let_underscore_lock)]
    fn eat(&self) {
        // Pick up forks...
        let _ = self.left_fork.lock().unwrap();
        let _ = self.right_fork.lock().unwrap();

        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {
    let (tx, rx) = mpsc::channel();

    // Create forks
    let forks = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<_>>();

    // Create philosophers
    // and make each of them think and eat 100 times
    let handlers = PHILOSOPHERS
        .iter()
        .enumerate()
        .map(|(index, name)| Philosopher {
            name: String::from(*name),
            left_fork: forks[index].clone(),
            right_fork: forks[(index + 1) % forks.len()].clone(),
            thoughts: tx.clone(),
        })
        .map(|philosopher| {
            thread::spawn(move || {
                for _ in 0..100 {
                    philosopher.think();
                    philosopher.eat();
                }
            })
        });

    // Output their thoughts
    while let Ok(thought) = rx.recv() {
        println!("{}", thought);
    }

    handlers.for_each(|handler| handler.join().unwrap());
}
