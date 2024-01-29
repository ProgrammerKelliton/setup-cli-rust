use std::env;

use clap::{Arg, Command};
use owo_colors::OwoColorize;

mod manager_files;
mod setup_run;

fn setup_parser() {
    struct Info {
        cli_name: &'static str,
        version: &'static str,
        author: &'static str,
        about: &'static str,
    }

    let info: Info = Info {
        cli_name: "setup",
        version: "1.0",
        about: "Use templates prontos para criar suas aplicações",
        author: "Kelliton Santana",
    };

    let init_arg: Arg = Arg::new("NEW SETUP NAME")
        .long("init")
        .short('i')
        .required(false);

    let load_arg: Arg = Arg::new("RUN SETUP NAME")
        .long("run")
        .short('r')
        .required(false);

    let matches = Command::new(info.cli_name)
        .version(info.version)
        .author(info.author)
        .about(info.about)
        .arg(init_arg)
        .arg(load_arg)
        .get_matches();

    let init_setup_name = matches.get_one::<String>("NEW SETUP NAME");
    let run_setup_name = matches.get_one::<String>("RUN SETUP NAME");

    match init_setup_name {
        Some(value) => manager_files::verify_if_config_file_exists(value.to_string()),
        None => (),
    }

    match run_setup_name {
        Some(value) => setup_run::run(value.to_string()),
        None => (),
    }
}

fn main() {
    println!(
        "Using path: {}",
        env::var("SETUP_ROOT_PATH")
            .unwrap_or("./".to_string())
            .cyan(),
    );

    setup_parser();
}
