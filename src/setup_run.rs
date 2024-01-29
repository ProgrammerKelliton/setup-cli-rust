use std::{fs, process::Command};

use owo_colors::OwoColorize;
use setup::get_config_file_path;

pub fn run(name: String) {
    let path: String = get_config_file_path!(name);

    let content_json: String = fs::read_to_string(path).expect("Não foi possível ler setup");

    let content: serde_json::Value =
        serde_json::from_str(content_json.as_str()).expect("Não foi possível ler json");

    let packages: &Vec<serde_json::Value> = content["packages"]
        .as_array()
        .expect("Não foi possível ler pacotes");

    println!("{}", "Prepare to install packages:".cyan());

    for command in packages {
        let current_command: String = command.to_string().replace("\"", "");

        println!("Run {}...", current_command.cyan());

        Command::new(current_command.as_str())
            .spawn()
            .expect("Failed to execute command");
    }
}
