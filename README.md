# ⏱ Rust Stopwatch 🍅🚫

A simple command-line stopwatch written in Rust.  
Built because I wanted to **learn Rust** and make something I’d *actually use* while studying.

It’s like a pomodoro timer — but without the pomodoro... 

**no tomatoes** 

---

## 🧠 Features

- Start / pause / clear the stopwatch from the terminal.
- Live timer display that updates in-place (`00:00:00` format).
- Thread-safe state with `Arc<Mutex<T>>`.
- Clean separation between logic and user interaction.
- Includes a test suite for timer logic.

---

## ▶️ Commands

Once running:

- `s` — Start or resume the stopwatch
- `p` — Pause and print the current time
- `c` — Clear/reset the timer
- `q` — Quit
- (Anything else prints a goofy echo)

---

## 🦀 Why?

This project started as a way to:
- Practice core Rust concepts like `Arc`, `Mutex`, and `std::thread`
- Handle user input and shared state in a clean way
- Build a real tool to track focused study time — without distractions

---

## 🛠 Usage

To run this stopwatch, you’ll need [Rust installed](https://rustup.rs).

Then clone the repo and run the app:

```bash
git clone https://github.com/bschelske/no-tomatoes.git
cd no-tomatoes
cargo run
