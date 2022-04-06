use chrono::{Local, NaiveTime, SubsecRound};
use notify_rust::Notification;
use std::{env::args, process::exit, thread::sleep, time::Duration};

fn get_sleep_seconds() -> u64 {
    let args: Vec<String> = args().collect();
    if args.len() <= 1 {
        println!("ERROR: provide a time to set the alarm");
        exit(1);
    }

    let alarm_time = match NaiveTime::parse_from_str(&args[1], "%H:%M:%S") {
        Ok(value) => value,
        Err(_) => {
            println!("ERROR: Bad input format, insert alarm time as HH:MM:SS");
            exit(1);
        }
    };
    let now = Local::now().time().round_subsecs(0);

    let sleep_seconds = (alarm_time - now).num_seconds();
    if sleep_seconds <= 0 {
        println!("ERROR: Input time is past! Please input valid alarm time.");
        exit(1);
    } else {
        sleep_seconds as u64
    }
}

fn main() {
    sleep(Duration::from_secs(get_sleep_seconds()));
    Notification::new()
        .summary("simple-alarm")
        .body("It's time!")
        .icon("xclock")
        .appname("simple-alarm")
        .timeout(0) // never expire
        .show()
        .unwrap();
}
