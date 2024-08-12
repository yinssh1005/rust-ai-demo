mod dbConnect;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use chrono::*;
use clap::{Arg, Command};
use dialoguer::{Input, Confirm};

fn tz_convert(datestr: &str, target_tz: &str) -> Option<DateTime<Tz>> {
    let naive_datetime = NaiveDateTime::parse_from_str(datestr, "%Y-%m-%d %H:%M:%S").ok()?;
    let tz: Tz = target_tz.parse().ok()?;

    // Some(tz.from_local_datetime(&naive_datetime).single()?)
    Some(tz.from_local_datetime(&naive_datetime).single()?)
}

// This function takes time in UTC and returns time in target Timezone
fn convert_to_timezone(dt: DateTime<Utc>, target_tz: Tz) -> DateTime<Tz> {
    // Here we convert the DateTime<Utc> to DateTime<FixedOffset>
    let dtf = dt.with_timezone(&target_tz);

    dtf
}

// Example usage:

fn main() {
    let matches = Command::new("Interactive Timezone Converter Tool")
        .version("1.0")
        .author("Ethan.Yin <ethan.yin@jetbrains.com>")
        .about("A simple interactive CLI tool example")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                // .value_name("NAME")
                // .help("-h")
                .takes_value(true)
        )
        .get_matches();

    let name: String = match matches.value_of("name") {
        Some(name) => name.to_string(),
        None => Input::new()
            .with_prompt("What's your name?")
            .interact_text()
            .unwrap(),
    };

    let age: u32 = Input::new()
        .with_prompt("How old are you?")
        .interact_text()
        .unwrap();

    let likes_rust: bool = Confirm::new()
        .with_prompt("Do you like Rust?")
        .default(true)
        .interact()
        .unwrap();

    println!("\nSummary:");
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Likes Rust: {}", if likes_rust { "Yes" } else { "No" });

    // println!("1st, convert the fixed timezone string to the target timezone format");
    // println!("...");
    //
    // match tz_convert("2024-01-01 01:00:00", "Europe/Berlin") {
    //     Some(dt) => println!("{}", dt),
    //     None => println!("Invalid date/time or timezone"),
    // }
    // println!("2nd, calculate the fixed timezone string to the target timezone");
    // println!("...");
    //
    // // let utc_dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 30, 0);
    // // Create a datetime in UTC
    // let utc_dt = Utc.ymd(2024, 1, 1).and_hms(0, 30, 0);
    //
    // // Set up the target timezone
    // let jst = chrono_tz::Asia::Tokyo;
    //
    // // Here we convert the time to Tokyo's timezone
    // let result = convert_to_timezone(utc_dt, jst);
    //
    // println!("Result: {}", result);
}