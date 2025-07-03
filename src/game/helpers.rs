
use super::cases::Cases;
use colored::*;
use std::io::{self, Write};

const NUM_CASES:u8 = 26;

pub fn format_banner(title: &str) -> String {
    format!(
        "{}\n{}\n{}",
        "=".repeat(40),
        title.bold().cyan(),
        "=".repeat(40)
    )
}

pub fn prompt_oi(msg: &str, buf: &mut String) {
    println!("{msg}");
    io::stdout().flush().unwrap();
    io::stdin().read_line(buf).unwrap();
    clear_screen();
    println!("\n {}\n", "=".repeat(40));
}

pub fn read_case_number(cases: &Cases) -> u8 {
    loop {
        let mut buf = String::new();
        prompt_oi("Please enter a number from 1-26!", &mut buf);

        match buf.trim().parse::<u8>() {
            Ok(val) => {
                if (1..=NUM_CASES).contains(&val) && cases.is_valid(val) {
                    break val;
                } else if (1..=NUM_CASES).contains(&val) {
                    println!(
                        "{}",
                        "This number is already out of the game!".yellow().bold()
                    );
                } else {
                    println!("{}", "Not valid number!".red().bold());
                }
            }
            _ => {
                continue;
            }
        }
    }
}

pub fn read_yes_no() -> bool {
    loop {
        let mut buf = String::new();
        prompt_oi(&format!("{}", " [y/n] ".bold()), &mut buf);

        match buf.trim().to_lowercase().as_str() {
            "y" => break true,
            "yes" => break true,

            "n" => break false,
            "no" => break false,

            _ => {
                continue;
            }
        }
    }
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
