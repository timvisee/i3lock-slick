use std::process::Command;

fn main() {
    println!("Hello, world!");

    // Show the lock screen
    lock();
}

fn lock() {
    // Invoke i3lock
    Command::new("i3lock")
        .spawn()
        .expect("Failed to invoke i3lock");
}
