mod directory;
mod prompt;
mod tokenizer;

#[macro_use]
extern crate simple_error;

use crate::prompt::print_prompt;
use ctrlc;
use simple_error::SimpleError;
use tokenizer::command::CommandInput;
use std::io::{stderr, stdin, stdout, Write};


fn main() {
    ctrlc::set_handler(move || {}).expect("Error setting Ctrl-C handler");

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

    let input_split: Vec<&str> = input.trim_end().split_whitespace().collect();
    let command_input = CommandInput::new(input_split);

    dbg!(&command_input);

    match command_input.command {
        "cd" => return Ok(false),
        "exit" => return Ok(true),
        _ => {}
    }

    //Execute the command
    let comm_output = command_input.execute();

    match comm_output {
        Ok(output) => {
            stdout().write_all(&output.stdout).unwrap();
            stdout().write_all(&output.stderr).unwrap();
        }
        Err(simple_error) => stderr()
            .write_all(
                format!(
                    "command not found: {}, {}\n",
                    command_input.command,
                    simple_error.to_string()
                )
                .as_bytes(),
            )
            .unwrap(),
    }

    Ok(false)
}
