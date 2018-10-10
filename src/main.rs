extern crate rand;
extern crate rustyline;

use rand::{thread_rng, Rng};
use rustyline::Editor;
use std::cmp::Ordering;

static PROMPT: &str = ">> ";

fn help_me() {
    println!("h = help");
    println!("q = quit");
    println!("r = quick roll");
    println!("ra # = quick roll");
}

fn roll_dice(num: i64, sides: u64) -> i64 {
    let mut rng = thread_rng();
    (0..num.abs())
        .map(|_| rng.gen_range(1, sides as i64 + 1))
        .sum()
}

fn is_next_number(i: std::str::SplitWhitespace) -> Result<i64, &str> {
    let mut iter = i;
    if let Some(s) = iter.next() {
        if let Ok(n) = s.parse::<i64>() {
            return Ok(n);
        }
    };
    println!("!! A number was expected next");
    Err("Next was not exist or a number")
}

fn quick_roll() {
    let roll = roll_dice(3, 6);
    println!("Rolled a {}", roll);
}

fn roll_against(against: i64) {
    if against < 1 {
        println!("!! Rolling against {} is an error", against);
        return;
    };
    let roll: i64 = roll_dice(3, 6);
    match roll.cmp(&against) {
        Ordering::Less | Ordering::Equal => {
            println!("Success! delta {}", against - roll);
        }
        Ordering::Greater => {
            println!("Failure! delta {}", roll - against);
        }
    };
}

fn main() {
    println!("Welcome to the roller;");
    println!("q to quit, h to help");
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(PROMPT);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());
                let mut iter = line.split_whitespace();
                match iter.next() {
                    Some("q") => break,
                    Some("h") => help_me(),
                    Some("r") => quick_roll(),
                    Some("ra") => {
                        if let Ok(n) = is_next_number(iter) {
                            roll_against(n);
                        }
                    }
                    _ => continue,
                };
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
    println!("Later!");
}
