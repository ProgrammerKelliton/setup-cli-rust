use owo_colors::OwoColorize;
use setup::get_config_file_path;
use std::{
    fs::File,
    io::{stdin, Write},
    path::Path,
};

mod error_handlering;

fn override_config_file(name: String) {
    let mut override_file_answer: String = String::new();

    println!("Já existe um arquivo de configuração deseja sobrecrever ele? [s/n]");

    stdin()
        .read_line(&mut override_file_answer)
        .expect("Não foi possível obter resposta");

    match override_file_answer.as_str().trim() {
        "s" | "S" => create_config_file(name),
        _ => println!("{}", "Cancel".red()),
    }
}

fn create_content_file(commands: String, name: String) -> String {
    let all_commands: Vec<&str> = commands.split_inclusive(",").collect();
    let mut commands_in_json: Vec<String> = Vec::new();

    let mut count = 0;

    while count < all_commands.len() {
        commands_in_json.push(all_commands[count].replace(",", "").replace("\n", ""));

        count += 1;
    }

    let content: String = format!(
        "{{\n\t\"name\": \"{}\",\n\t\"packages\": {:?}\n}}",
        name.trim(),
        commands_in_json
    );

    return content;
}

fn create_config_file(name: String) {
    let mut commands: String = String::new();

    // Get commands
    println!("Escreva a sequência de comandos, Ex: yarn add package,yarn add package2,...");
    stdin()
        .read_line(&mut commands)
        .expect("Não foi possível ler comandos");

    error_handlering::is_empty_field(commands.clone())
        .expect("Você precisa dar um nome para esse setup");

    // Creating file
    println!("{}", "Creating file...".green().bold());

    let content: String = create_content_file(commands, name.clone());

    let mut file =
        File::create(get_config_file_path!(name)).expect("Não foi possível criar arquivo");

    file.write_all(content.as_bytes())
        .expect("Não foi possível escrever configuração");

    println!("\n{}", content.cyan());
}

pub fn verify_if_config_file_exists(name: String) {
    let path_config_file: String = get_config_file_path!(name.clone());

    let path = Path::new(&path_config_file);

    if path.exists() {
        override_config_file(name);
    } else {
        create_config_file(name);
    }
}
