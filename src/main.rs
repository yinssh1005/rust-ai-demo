// mod dbConnect;

use clap::{Arg, Command};
use dialoguer::Input;

// Example usage:

fn main() {
    let matches = Command::new("Interactive Timezone Converter Tool")
        .version("1.0")
        .author("Ethan.Yin <ethan.yin@jetbrains.com>")
        .about("A simple interactive CLI tool example")
        .arg(
            Arg::new("option")
            .help("-h")
        )
        .get_matches();

    let meetingDateTime: String = Input::new()
        .with_prompt("Please type the meeting time in your timezone 'YYYY-MM-DD HH:MM' format.")
        .interact_text()
        .unwrap();

    let localTimezone: String = Input::new()
        .with_prompt("Your timezone? (UTC+8)")
        .default("UTC+8".to_string())
        .interact_text()
        .unwrap();

    let targetTimezone: String = Input::new()
        .with_prompt("Target timezone?(UTC+2)")
        .default("UTC+2".to_string())
        .interact_text()
        .unwrap();

    println!("\nThe meeting time is going to be:");
    println!("Date & Time: {}", meetingDateTime);
    println!("Your Timezone: {}", localTimezone);
    println!("Target Timezone: {}", targetTimezone);
}