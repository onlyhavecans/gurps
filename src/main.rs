extern crate d20;
use std::io;
use std::cmp::Ordering;

static GRUPS_ROLL: &str = "3d6";

fn roll_me(s: &str) -> i32 {
    let r = d20::roll_dice(s).unwrap();
    //println!("Roll: {}", r);
    match r.total {
        17...18 => println!("Crit Fail!"),
        1...2 => println!("Crit Success!"),
        _ => {},
    };
    r.total
}

fn get_input() -> String {
    let mut i = String::new();
    io::stdin().read_line(&mut i)
        .expect("Failed to read line");

    let i: String = match i.trim().parse() {
        Ok(l) => l,
        Err(_) => String::new(),
    };
    return i
}

fn is_next_number(i: std::str::SplitWhitespace) -> Result<i32, &str> {
    let mut iter = i;
    if let Some(s) = iter.next() {
        if let Ok(n) = s.parse::<i32>() {
            return Ok(n)
        }
    };
    println!("A number was expected next");
    return Err("Next was not exist or a number")
}

fn help_me() {
    println!("h = help");
    println!("q = quit");
    println!("r = quick roll");
    println!("ra # = quick roll");
}

fn quick_roll(){
    let r = roll_me(GRUPS_ROLL);
    println!("Rolled a {}", r);
}

fn roll_against(n: i32){
    if n == 0 || n > 18 {
        println!("Rolling against {} is an error", n);
        return
    };

    let r: i32 = roll_me(GRUPS_ROLL);
    let win: bool = match r.cmp(&n) {
        Ordering::Less | Ordering::Equal => true,
        Ordering::Greater => false
    };

    let delta: i32 = if win {
        print!("Success!");
        n - r
    } else {
        print!("Failure!");
        r - n
    };
    println!(" delta {}", delta);
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
            },
            _ => continue,
        };
    }
    println!("Later!");
}
