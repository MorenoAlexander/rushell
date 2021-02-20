#[macro_use]

extern crate simple_error;
use simple_error::SimpleError;

use std::io::{stdin, stdout, Write};
use std::process::Command;




use std::error::Error;

fn main() {
    println!("Welcome to rushell!");

    loop {
        let input = match get_user_input() {
            Ok(res) => res,
            Err(_error) =>  {
                println!("An error occured while processing that input. Sorry!");
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

fn get_user_input()  -> Result<String, SimpleError> {
    print!("R$> ");
    stdout().flush();
    let mut buffer_input = String::new();
    
    
    if stdin().read_line(&mut buffer_input).is_err() {
        bail!(SimpleError::new("Error from stdin"))
    };

    

    Ok(buffer_input)
}


/**
 * breaks down input into command ..args
 */
fn process_command (input : String ) -> Result<bool, SimpleError> {
    if input.trim_end().len() == 0 {
        bail!(SimpleError::new("no input"));
    }

    let mut input_split : Vec<&str> = input.trim_end().split_whitespace().collect();
    let command : &str = input_split.remove(0);

    
    
    if command == "exit"{
        return Ok(true);
    }

    print!("command: {}",command);
    //Execute the command
    let comm_output = Command::new(command).args(input_split).output().expect("failed to execute process");

    println!("status: {},", comm_output.status);

    stdout().write_all(&comm_output.stdout).unwrap();
    stdout().write_all(&comm_output.stderr).unwrap();
    Ok(false)
}