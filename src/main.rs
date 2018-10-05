extern crate d20;

use std::cmp::Ordering;
use std::io;

static GRUPS_ROLL: &str = "3d6";
#[allow(dead_code)]
static D20_ROLL: &str = "1d20";

fn help_me() {
    println!("h = help");
    println!("q = quit");
    println!("r = quick roll");
    println!("ra # = quick roll");
}

fn roll_me(s: &str) -> i32 {
    let r = d20::roll_dice(s).unwrap();
    println!("Roll: {}", r);
    match r.total {
        17...18 => println!("Crit Fail!"),
        1...2 => println!("Crit Success!"),
        _ => {}
    };
    r.total
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("!!Failed to read line");
    String::from(input)
}

fn is_next_number(i: std::str::SplitWhitespace) -> Result<i32, &str> {
    let mut iter = i;
    if let Some(s) = iter.next() {
        if let Ok(n) = s.parse::<i32>() {
            return Ok(n);
        }
    };
    println!("!! A number was expected next");
    Err("Next was not exist or a number")
}

fn quick_roll() {
    let roll = roll_me(GRUPS_ROLL);
    println!("Rolled a {}", roll);
}

fn roll_against(against: i32) {
    if against == 0 || against > 18 {
        println!("!! Rolling against {} is an error", against);
        return;
    };
    let roll: i32 = roll_me(GRUPS_ROLL);
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
