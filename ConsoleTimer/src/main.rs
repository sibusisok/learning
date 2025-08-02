use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("Timer application started!");
    println!("Timer will trigger every 5 minutes.");
    println!("Type 'terminate' and press Enter to stop the application.\n");

    // Shared flag to signal termination
    let should_terminate = Arc::new(Mutex::new(false));
    let should_terminate_clone = Arc::clone(&should_terminate);

    // Start the timer thread
    let timer_thread = thread::spawn(move || {
        let timer_duration = Duration::from_secs(5 * 60); // 5 minutes
        let mut last_trigger = Instant::now();

        loop {
            // Check if we should terminate
            {
                let terminate = should_terminate_clone.lock().unwrap();
                if *terminate {
                    println!("Timer thread stopping...");
                    break;
                }
            }

            // Check if 5 minutes have passed
            if last_trigger.elapsed() >= timer_duration {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap();

                println!("⏰ TIMER TRIGGERED! [{}]", format_timestamp(now.as_secs()));
                last_trigger = Instant::now();
            }

            // Sleep for a short interval to avoid busy waiting
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Main thread handles user input
    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Ensure a prompt is displayed

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim().to_lowercase();

                if input == "terminate" {
                    println!("Termination command received. Stopping application...");

                    // Signal the timer thread to stop
                    {
                        let mut terminate = should_terminate.lock().unwrap();
                        *terminate = true;
                    }

                    // Wait for the timer thread to finish
                    timer_thread.join().unwrap();

                    println!("Application terminated successfully.");
                    break;
                } else if !input.is_empty() {
                    println!("Unknown command: '{}'. Type 'terminate' to stop.", input);
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
            }
        }
    }
}

fn format_timestamp(timestamp: u64) -> String {
    use std::time::UNIX_EPOCH;

    let system_time = UNIX_EPOCH + Duration::from_secs(timestamp);

    // Simple timestamp formatting (you could use cron crate for better formatting)
    match system_time.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let total_seconds = duration.as_secs();
            let hours = (total_seconds / 3600) % 24;
            let minutes = (total_seconds / 60) % 60;
            let seconds = total_seconds % 60;
            format!("{:02}:{:02}:{:02} UTC", hours, minutes, seconds)
        }
        Err(_) => "Invalid timestamp".to_string(),
    }
}

// Alternative version with better timestamp formatting using cron
// Add to Cargo.toml: cron = { version = "0.4", features = ["serde"] }
/*
use cron::{DateTime, Utc};

fn format_timestamp_cron() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}
*/

// Cargo.toml for this project:
/*
[package]
name = "timer-console-app"
version = "0.1.0"
edition = "2021"

[dependencies]
# No external dependencies needed for a basic version
# cron = { version = "0.4", features = ["serde"] } # Optional for better timestamps
*/