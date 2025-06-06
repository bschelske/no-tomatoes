use std::time::{Duration, Instant};

struct Timer {
    active: bool,
    start_time: Instant,
    elapsed: Duration,
    display: Duration,
}

impl Timer {
    fn start(&mut self) {
        self.active = true;
        if self.elapsed > Duration::ZERO {
            println!("Resuming timer at {:?}", self.display)
        } else {
            self.start_time = Instant::now();
        }
    }

    fn pause(&mut self) {
        self.active = false;
        self.elapsed = self.start_time.elapsed();
        self.display += self.elapsed;
    }

    fn clear(&mut self) {
        self.active = false;
        self.elapsed = Duration::ZERO;
        self.display = Duration::ZERO;
    }
}

fn main() {
    let mut timer = Timer {
        active: false,
        start_time: Instant::now(),
        elapsed: Duration::ZERO,
        display: Duration::ZERO,
    };

    timer.start();
    let current_display = timer.display;
    println!("Lets begin, its {:?}", current_display);

    timer.pause();
    let current_display = timer.display;
    println!("After pausing!, its {:?}", current_display);

    println!("Lets start it up again!");
    timer.start();

    timer.clear();
    let current_display = timer.display;
    println!("After clearing!, its {:?}", current_display);
}
