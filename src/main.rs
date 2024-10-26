mod lib;
use std::env;
use std::path::Path;
use std::fs;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use colored::*;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    username: String,
    std_id: i32,
}

fn input(prompt: &str) -> i32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Error[input_s1]!");

    match input_string.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Error[input_s2]! Please enter a valid integer.");
            input(prompt)
        }
    }
}

fn read_config(config_file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(config_file_path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

fn create_config(config_file_path: &str) -> io::Result<()> {
    println!("{}","^•ﻌ•^".blue());

    let username: String = lib::str_input("Enter your name: ");
    let std_id: i32 = input("Enter your student ID: ");

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

fn save_config() -> io::Result<()> {
    let user_profile = env::var("USERPROFILE").or_else(|_| env::var("HOME")).expect("Unable to find user directory");
    let config_dir = format!("{}/Documents/enderdreams/config", user_profile);
    let config_file_path = format!("{}/lab_config.toml", config_dir);

    fs::create_dir_all(&config_dir)?;
    if Path::new(&config_file_path).exists() {
        match read_config(&config_file_path) {
            Ok(config) => {
                println!("Hey, {}! ({})", config.username.purple(), config.std_id.to_string().blue());

                let lab_id: i32 = input(">>  Lab no      : ");
                let num_of_tasks: i32 = input(">>  Number of tasks: ");
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

fn ops(std_id: i32, lab_id: i32, tasks: i32, use_current_dir: bool) -> std::io::Result<()> {
    let dir_path = if use_current_dir {
        ".".to_string()
    } else {
        format!("{}_Lab{}", std_id, lab_id)
    };

    if !use_current_dir {
        fs::create_dir_all(&dir_path)?;
    }

    println!("Creating files...");
    for i in 1..=tasks {
        let file_name = format!("{}/{}_lab{}_Task{}.c", dir_path, std_id, lab_id, i);
        fs::write(file_name.clone(), lib::C_CODE)?;
        println!("[✔] {}", file_name.green());
    }
    println!("{}","All files created successfully! 🎉".blue());
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let _ = save_config();
        return Ok(());
    } else if args.len() < 4 || args.len() > 5 {
        println!("{}", lib::USAGE);
        return Ok(());
    }

    let std_id = args[1].parse::<i32>().expect("Invalid student ID");
    let lab_id = args[2].parse::<i32>().expect("Invalid lab number");
    let num_of_tasks = args[3].parse::<i32>().expect("Invalid number of tasks");

    // Check if the last argument is a period, indicating current directory usage
    let use_current_dir = args.len() == 5 && args[4] == ".";

    ops(std_id, lab_id, num_of_tasks, use_current_dir).expect("Failed to create files");

    Ok(())
}
