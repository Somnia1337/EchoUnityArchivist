use crate::{Confirmation, EnumValues, Prompts, RangeUsize};

use lettre::Address;
use std::io::{self, Write};

/// Reads user input from command line, with a customized prompt.
pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    input.trim().to_owned()
}

/// Prompt the user to enter an email address, loops until a valid value is provided.
pub fn read_email(prompt_read: &str, prompt_invalid: &str) -> Address {
    loop {
        match read_input(prompt_read).trim().parse().ok() {
            Some(x) => return x,
            _ => eprintln!("{}", prompt_invalid),
        }
    }
}

/// Prompt the user to enter a selection of `usize`, loops until a valid value is provided.
pub fn read_selection(
    prompt_read: &str,
    prompt_invalid: &str,
    prompt_object: &str,
    prompt_should_be: &str,
    range_usize: &RangeUsize,
) -> usize {
    loop {
        match read_input(prompt_read).trim().parse::<usize>().ok() {
            Some(x) if x >= range_usize.lo && x <= range_usize.hi => return x,
            _ => eprintln!(
                "\
{}{}: {}
  {}",
                prompt_invalid,
                prompt_object,
                prompt_should_be,
                range_usize.valid_values()
            ),
        }
    }
}

/// Prompt the user to enter the reconfirmation for sending a message, loops until a valid value is provided.
pub fn read_reconfirmation(prompts: &Prompts, reconfirmation: &Confirmation) -> bool {
    println!("{}", prompts.send_reconfirm_list);
    loop {
        let input = read_input(prompts.send_reconfirm_selection).to_lowercase();
        if matches!(input.as_str(), "yes" | "no") {
            return input == "yes";
        } else {
            eprintln!(
                "\
{}{}: {}
  {}",
                prompts.invalid_literal,
                prompts.send_confirm_literal,
                prompts.should_be_one_of_below_literal,
                reconfirmation.valid_values()
            );
        }
    }
}

/// Reads the email's body from user input, until 2 consecutive empty lines are met.
pub fn read_body(prompts: &Prompts) -> String {
    println!("{}", prompts.compose_content);
    let mut body = String::new();
    let mut buf;

    let mut empty_count = 0;
    while empty_count < 2 {
        buf = read_input("  ") + "\n";
        body += &buf;
        if buf.trim().is_empty() {
            empty_count += 1;
        } else {
            empty_count = 0;
        }
        buf.clear();
    }

    body.trim_end().to_string()
}

/// Prints the real body part of an email, ignores useless headers.
pub fn print_body(email: String, prompts: &Prompts) {
    println!("{}", prompts.horizontal_start);
    let mut body = false;
    for line in email.lines() {
        // Real body starts at line "From: "
        if line.starts_with("From: ") {
            body = true;
        }
        // Ignore "Content" & "To" headers
        if body && !(line.starts_with("Content") || line.starts_with("To")) {
            println!("  {}", line);
        }
    }
    println!("{}", prompts.horizontal_end);
}
