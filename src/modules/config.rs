use crate::modules::utils;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use std::{env, fs, io};
use utils::ops;
use utils::str_input;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub username: String,
    pub std_id: i32,
}

pub fn read_config(config_file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(config_file_path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

pub fn create_config(config_file_path: &str) -> io::Result<()> {
    println!("{}", "^•ﻌ•^".blue());

    let username: String = str_input("Enter your name: ");
    let std_id: i32 = str_input("Enter your student ID: ").parse().unwrap();

    let config = Config { username, std_id };
    let toml_str = toml::to_string(&config).unwrap();

    let mut file = fs::File::create(config_file_path)?;
    file.write_all(toml_str.as_bytes())?;

    println!(
        "{} file created successfully and saved in \"/Documents/enderdreams/config/\"!\n",
        "lab_config.toml".green()
    );
    Ok(())
}

pub fn save_config() -> io::Result<()> {
    let user_profile = env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .expect("Unable to find user directory");
    let config_dir = format!("{}/Documents/enderdreams/config", user_profile);
    let config_file_path = format!("{}/lab_config.toml", config_dir);

    fs::create_dir_all(&config_dir)?;
    if Path::new(&config_file_path).exists() {
        match read_config(&config_file_path) {
            Ok(config) => {
                println!(
                    "Hey, {}! ({})",
                    config.username.purple(),
                    config.std_id.to_string().blue()
                );

                let lab_id: i32 = str_input(">>  Lab no      : ").parse().unwrap();
                let num_of_tasks: i32 = str_input(">>  Number of tasks: ").parse().unwrap();
                ops(config.std_id, lab_id, num_of_tasks, false).expect("Failed to create files");
            }
            Err(err) => {
                println!("Failed to read config: {}", err);
            }
        }
    } else {
        create_config(&config_file_path)?;
        let _ = save_config();
    }

    Ok(())
}
