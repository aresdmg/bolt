use std::fs;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::cli::FILE_NAME;

#[derive(Serialize, Deserialize)]
struct Todos {
    id: String,
    message: String,
    completed: bool,
    created_at: String,
}

pub fn parse_command(command: &String) -> String {
    return String::from(command).trim().to_lowercase();
}

pub fn pasred_message(message: &[String]) -> (String, Vec<String>) {
    let mut words: Vec<String> = Vec::new();
    let mut options: Vec<String> = Vec::new();

    for item in message {
        if let Some(stripped) = item.strip_prefix("--") {
            options.push(stripped.to_string());
        } else {
            words.push(item.clone());
        }
    }

    let parsed_msg = words.join(" ");

    (parsed_msg, options)
}

pub fn add(messages: String, _options: Vec<String>) {
    let mut todos: Vec<Todos>;

    let file_result = fs::read_to_string(FILE_NAME);
    if file_result.is_ok() {
        let file_content = file_result.unwrap();

        let parsed_content: Result<Vec<Todos>, serde_json::Error> =
            serde_json::from_str(&file_content);
        if parsed_content.is_ok() {
            todos = parsed_content.unwrap();
        } else {
            todos = Vec::new();
        }
    } else {
        todos = Vec::new();
    }

    let new_todo = Todos {
        id: Uuid::new_v4().to_string(),
        message: messages,
        completed: false,
        created_at: Utc::now().to_rfc3339(),
    };

    todos.push(new_todo);
    let json = serde_json::to_string_pretty(&todos).unwrap();

    fs::write(FILE_NAME, json).unwrap();
    return;
}
