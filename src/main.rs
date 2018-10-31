extern crate rand;
extern crate rustyline;

use rand::{thread_rng, Rng};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::cmp::Ordering;
use std::fmt;

static PROMPT: &str = ">> ";

struct DieRoll {
    multiplier: u64,
    sides: u64,
}

// If you print or display this, it returns a roll
impl fmt::Display for DieRoll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = format!("{}d{}, Total: {}", self.multiplier, self.sides, self.roll());
        write!(f, "{}", out)
    }
}

impl DieRoll {
    fn new(multiplier: u64, sides: u64) -> DieRoll {
        DieRoll {
            multiplier: multiplier,
            sides: sides,
        }
    }

    fn roll(&self) -> i64 {
        let mut rng = thread_rng();
        (0..self.multiplier)
            .map(|_| rng.gen_range(1, self.sides as i64 + 1))
            .sum()
    }
}

fn help_me() {
    println!("h = help");
    println!("q = quit");
    println!("g = quick gurps roll");
    println!("d = quick d20 roll");
    println!("r #d# = quick roll");
    println!("ra # = quick roll");
}

fn print_quick_roll(m: u64, s: u64) {
    let die = DieRoll::new(m, s);
    println!("Rolled a {}", die);
}

fn die_from_term(term: &str) -> Result<DieRoll, String> {
    if term.to_lowercase().contains('d') {
        let v: Vec<&str> = term.split('d').collect();

        let multiplier = v[0].parse::<u64>().map_err(|e| e.to_string())?;
        let sides = v[1].parse::<u64>().map_err(|e| e.to_string())?;

        if multiplier < 1 || sides < 1 {
            return Err(String::from(
                "Both multiplier and sides need to be greater than 0",
            ));
        }

        Ok(DieRoll::new(multiplier, sides))
    } else {
        Err(String::from("Your roll was missing a d"))
    }
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

fn roll_against(against: i64) {
    if against < 1 {
        println!("!! Rolling against {} is an error", against);
        return;
    };
    let roll: i64 = DieRoll::new(3, 6).roll();
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
    help_me();
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
                    Some("g") => print_quick_roll(3, 6),
                    Some("d") => print_quick_roll(1, 20),
                    Some("r") => {
                        if let Some(n) = iter.next() {
                            match die_from_term(n) {
                                Ok(d) => println!("You rolled a {}", d),
                                Err(e) => println!("Error: {}", e),
                            }
                        } else {
                            println!("Specify what you are rolling with #d#")
                        }
                    }
                    Some("ra") => {
                        if let Ok(n) = is_next_number(iter) {
                            roll_against(n);
                        }
                    }
                    _ => continue,
                };
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
    println!("Thanks for playing!");
}
