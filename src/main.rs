extern crate d20;
use std::io;
use std::cmp::Ordering;

static GRUPS_ROLL: &str = "3d6";

fn roll_me(s: &str) -> i32 {
    let r = d20::roll_dice(s).unwrap();
    //println!("Roll: {}", r);
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

fn roll_against(i: std::str::SplitWhitespace){
    let mut mi = i;
    let s = match mi.next() {
        Some(s) => s,
        _ => "10"
    };

    let n: i32 = match s.parse::<i32>() {
        Ok(n) => n,
        Err(_) => 0,
    };

    if n == 0 || n > 18 {
        println!("Rolling against {} is an error", n);
        return
    };

    let r: i32 = roll_me(GRUPS_ROLL);
    let win: bool = match r.cmp(&n) {
        Ordering::Less => true,
        Ordering::Equal => true,
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
    println!("Welcome to the roller. q to quit");
    loop {
        let input = get_input();
        let mut iter = input.split_whitespace();
        match iter.next() {
            Some("q") => break,
            Some("h") => help_me(),




            Some("r") => quick_roll(),
            Some("ra") => roll_against(iter),
            _ => continue,
        };
    }
    println!("Later!");
}
