use std::{
    env,
    fs::{self, File},
    process,
};

use colored::Colorize;

use crate::engine;

pub const FILE_NAME: &str = "db.jsonc";

fn config() {
    match fs::exists(FILE_NAME) {
        Ok(true) => {}
        Ok(false) => {
            let file = File::create(FILE_NAME);
            match file {
                Ok(_) => {
                    println!("Files created")
                }
                Err(err) => {
                    eprintln!("Failed to create file: {}", err);
                    process::exit(1);
                }
            }
        }
        Err(_e) => {
            println!("");
            process::exit(1);
        }
    }
}

/**
 * A function that intialize the application and make sure the required are in present.
 */
fn init() {
    config();
}

fn get_args() -> Vec<String> {
    let arguement: Vec<String> = env::args().skip(1).collect();
    if arguement.len() < 1 {
        println!("Arguments should not be empty");
        process::exit(1);
    }

    arguement
}

pub fn start() {
    init();
    let args = get_args();

    let pasred_command: String = engine::parse_command(&args[0]);
    let (pasred_message, options) = engine::pasred_message(&args[1..]);

    match pasred_command.as_str() {
        "add" => {
            engine::add(pasred_message, options);
            println!("{}", ">> added".green());
        }
        "list" => {
            engine::list();
        }
        "done" => {
            engine::done(pasred_message);
        }
        "remove" => {
            engine::remove(pasred_message);
            println!("{}", ">> removed".red())
        }
        _ => {
            println!("command: {} not found!", pasred_command.yellow());
        }
    }
}
