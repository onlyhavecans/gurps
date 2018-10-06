extern crate rand;

use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn help_me() {
    println!("h = help");
    println!("q = quit");
    println!("r = quick roll");
    println!("ra # = quick roll");
}

pub fn roll_dice(num: i64, sides: u64) -> i64 {
    let mut rng = thread_rng();
    (0..num.abs())
        .map(|_| rng.gen_range(1, sides as i64 + 1))
        .sum()
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("!!Failed to read line");
    String::from(input)
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
    if against == 0 || against > 18 {
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
    loop {
        let input = get_input();
        let mut iter = input.split_whitespace();
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
    println!("Later!");
}
