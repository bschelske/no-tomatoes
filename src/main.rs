use std::io;
use std::thread::sleep;
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
            println!("Resuming stopwatch at {:?}", self.display)
        } else {
            println!("stopwatch starts NOW.");
            self.start_time = Instant::now();
        }
    }

    fn pause(&mut self) {
        if self.active == false {
            println!("Timer is already paused.");
            self.status()
        } else {
            self.active = false;
            self.elapsed = self.start_time.elapsed();
            self.display += self.elapsed;
            self.status();
        }
    }

    fn clear(&mut self) {
        self.active = false;
        println!("Timer cleared at {:?}", self.display);
        self.elapsed = Duration::ZERO;
        self.display = Duration::ZERO;
    }

    fn status(&self) {
        println!("Stopwatch: {:?}", self.display)
    }
}

fn timer_testing(mut timer: Timer) {
    timer.start();
    println!("Start the timer!");
    timer.status();

    println!("wait a sec");
    sleep(Duration::from_secs(1));
    timer.pause();

    println!("Done pausing");
    timer.status();

    println!("Lets start it up again!");
    timer.start();
    timer.status();

    println!("wait a half sec");
    sleep(Duration::from_millis(500));
    timer.status();
    timer.clear();
    println!("All clear");
    timer.status();
}

fn main() {
    let mut timer = Timer {
        active: false,
        start_time: Instant::now(),
        elapsed: Duration::ZERO,
        display: Duration::ZERO,
    };

    // timer_testing(timer)

    println!(
        r"
     _                           _       _     
 ___| |_ ___  _ ____      ____ _| |_ ___| |__  
/ __| __/ _ \| '_ \ \ /\ / / _` | __/ __| '_ \ 
\__ \ || (_) | |_) \ V  V / (_| | || (__| | | |
|___/\__\___/| .__/ \_/\_/ \__,_|\__\___|_| |_|
             |_|                               
"
    ); // I am obsessed with figlet. Sue me.

    println!("start (s) pause (p) clear (c) quit (q)");

    let mut running: bool = true;
    while running {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input = user_input.trim();

        match user_input {
            "q" => {
                running = false;
                println!("later")
            }
            "s" => timer.start(),
            "p" => timer.pause(),
            "c" => timer.clear(),
            _ => {
                println!("You said uhhh: {}", user_input);
            }
        }
    }
}
