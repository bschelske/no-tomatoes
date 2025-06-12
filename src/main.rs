use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct Timer {
    active: bool,
    start_time: Instant,
    elapsed: Duration,
    display: Duration,
}

impl Timer {
    fn start(&mut self) {
        if self.active == true {
            println!("\rThe stopwatch has already started");
        } else {
            self.active = true;
            if self.elapsed > Duration::ZERO {
                println!("\rResuming stopwatch at {}", self.format_display());
                self.start_time = Instant::now();
            } else {
                println!("\rstopwatch starts NOW.");
                self.start_time = Instant::now();
            }
        }
    }

    fn pause(&mut self) {
        if self.active == false {
            println!("\rTimer is already paused.");
            self.status()
        } else {
            self.active = false;
            self.elapsed = self.start_time.elapsed();
            self.display += self.elapsed;
            println!("\rStopwatch paused at: {}", self.format_display());
        }
    }

    fn clear(&mut self) {
        self.active = false;
        println!("\rTimer cleared at {}", self.format_display());
        self.elapsed = Duration::ZERO;
        self.display = Duration::ZERO;
    }

    
    fn format_display(&mut self) -> String {
        let total_seconds = self.display.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    fn status(&mut self) {
        println!("\rStopwatch: {}", self.format_display());
    }
}

fn main() {
    let timer = Arc::new(Mutex::new(Timer {
        active: false,
        start_time: Instant::now(),
        elapsed: Duration::ZERO,
        display: Duration::ZERO,
    }));

    let timer_clone = Arc::clone(&timer);

    thread::spawn(move || loop {
        {
            let timer = timer_clone.lock().unwrap();

            let current_elapsed = if timer.active {
                timer.display + timer.start_time.elapsed()
            } else {
                timer.display
            };
            let total_seconds = current_elapsed.as_secs();
            let hours = total_seconds / 3600;
            let minutes = (total_seconds % 3600) / 60;
            let seconds = total_seconds % 60;
            let formatted_time = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
            print!("\rStopwatch: {}", formatted_time);
            io::stdout().flush().unwrap();
        }
        thread::sleep(Duration::from_millis(100));
    });

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
            "s" => {
                let mut timer = timer.lock().unwrap();
                timer.start();
            }
            "p" => {
                let mut timer = timer.lock().unwrap();
                timer.pause();
            }
            "c" => {
                let mut timer = timer.lock().unwrap();
                timer.clear();
            }
            _ => {
                println!("\rYou said uhhh: {}", user_input);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_start_pause_clear() {
        let mut timer = Timer {
            active: false,
            start_time: Instant::now(),
            elapsed: Duration::ZERO,
            display: Duration::ZERO,
        };

        timer.start();
        assert!(timer.active);

        timer.pause();
        assert!(!timer.active);

        timer.clear();
        assert_eq!(timer.display, Duration::ZERO);
    }
}
