#![feature(total_cmp)]

use regex::Regex;
use std::io;

#[macro_use]
extern crate lazy_static;

mod nutrients;
mod pretty;

use pretty::pretty;

macro_rules! do_while {
    (do $stmt:block while $condition:expr, on fail => $on_fail:block) => {
        loop {
            $stmt;
            if !$condition {
                break;
            }
            $on_fail;
        }
    };
    (do $stmt:block while $condition:expr) => {
        do_while!(do $stmt while $condition, on fail => {});
    };
}

/// Clears the screen
fn cls() {
    // print!("{esc}c", esc = 27 as char);
    // This clears the screen better than the one above
    // print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    let mut nutrients = nutrients::Nutrients::default();

    loop {
        cls();

        let table = nutrients.table_with_shortest_alias();
        println!("{}", table);
        let mut name;
        // While shouldn't quit or while nutrient name is not valid
        // keep asking the question.
        println!(); // For formatting so error doesnt move table
        do_while!(
            do {
                name = String::new();
                println!("{}", pretty!("Enter a nutrient name: ", fg@green));
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                name = name.trim().to_string();
            // Repeat while name is not empty and nutrient alias is not valid
            } while !name.is_empty() && nutrients.get_nutrient(&name).is_none(),
            on fail => {
                cls();
                println!("{}", table);
                println!("{} {}",
                    pretty!("Could not parse input as value:", fg@red),
                    pretty!(name, b + fg@red)
                );
            }
        );

        // Quit if the user wants to quit.
        if name.is_empty() {
            break;
        }

        cls();
        let table = nutrients.table_with_shortest_alias_highlight(&name);
        println!("{}", table);
        println!(); // For formatting so error doesnt move table

        // Get the nutrient.
        let nutrient = nutrients.get_nutrient_mut(&name).unwrap();

        let mut value = None;
        let mut value_str;
        do_while!(
            do {
                value_str = String::new();

                if nutrient.unit.is_empty() {
                    println!("{} {}{}", pretty!("Enter a value for", fg@green), pretty!(nutrient.name, b | _ + fg@green), pretty!(": ", fg@green));
                } else {
                    println!("{} {} {}{}",
                        pretty!("Enter a value for", fg@green),
                        pretty!(nutrient.name, b | _ + fg@green),
                        pretty!(format!("({})", nutrient.unit), b | i + fg@green),
                        pretty!(": ", fg@green)
                    );
                }

                // Read from StdIn
                io::stdin()
                    .read_line(&mut value_str)
                    .expect("Failed to read line");
                value_str = value_str.trim().to_string();

                // Try parse the value
                // Matches `{digits}`, `{digits}.{digits}`, or `.{digits}`
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"(\d+\.\d+)|(\.\d+)|(\d+)").unwrap();
                }
                if let Some(found) = RE.find(&value_str) {
                    let value_str = found.as_str();
                    value = value_str.parse::<f64>().ok();
                }
            } while value.is_none(),
            on fail => {
                cls();
                println!("{}", table);
                println!("{} {}",
                    pretty!("Could not parse input as value:", fg@red),
                    pretty!(value_str, b + fg@red)
                );
            }
        );

        nutrient.set_value(value.unwrap());
    }

    cls();

    println!("{}", nutrients);
}
