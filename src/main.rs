mod directory;
mod prompt;

#[macro_use]
extern crate simple_error;

use crate::prompt::print_prompt;
use simple_error::SimpleError;
use std::error::Error;
use std::fmt::format;
use std::io::{stderr, stdin, stdout, Write};
use std::process::Command;
use users::{get_current_uid, get_user_by_uid};

fn main() {
    println!("Welcome to rushell!");

    loop {
        print_prompt();
        stdout().flush().expect("Error flushing stdout");
        let input = match get_user_input() {
            Ok(res) => res,
            Err(_error) => {
                println!("An error occurred while processing that input. Sorry!");
                continue;
            }
        };

        let result = match process_command(input) {
            Ok(done) => done,
            Err(_error) => {
                continue;
            }
        };

        if result {
            std::process::exit(0)
        }
    }
}

fn get_user_input() -> Result<String, SimpleError> {
    let mut buffer_input = String::new();
    if stdin().read_line(&mut buffer_input).is_err() {
        bail!(SimpleError::new("Error from stdin"))
    };

    Ok(buffer_input)
}

/**
 * breaks down input into command ..args
 */
fn process_command(input: String) -> Result<bool, SimpleError> {
    if input.trim_end().len() == 0 {
        bail!(SimpleError::new("no input"));
    }

    let mut input_split: Vec<&str> = input.trim_end().split_whitespace().collect();
    let command: &str = input_split.remove(0);

    match command {
        "exit" => return Ok(true),
        _ => {}
    }

    //Execute the command
    let comm_output = Command::new(command).args(input_split).output();

    match comm_output {
        Ok(output) => {
            stdout().write_all(&output.stdout).unwrap();
            stdout().write_all(&output.stderr).unwrap();
        }
        Err(simple_error) => stderr()
            .write_all(
                format!(
                    "command not found: {}, {}\n",
                    command,
                    simple_error.to_string()
                )
                .as_bytes(),
            )
            .unwrap(),
    }

    Ok(false)
}
