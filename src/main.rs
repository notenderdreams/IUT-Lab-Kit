mod lib;

use std::env;
use std::fs;
use std::io;
use colored::*;

fn input(prompt: &str) -> i32 {
    println!("{}", prompt.green());

    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Error[input_s1]!");

    match input_string.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "Error[input_s2]! Please enter a valid integer.".red());
            input(prompt)
        }
    }
}

fn ops(std_id: i32, lab_id: i32, tasks: i32) -> std::io::Result<()> {
    let dir_path = format!("{}_Lab{}", std_id, lab_id);
    let mut i = 1;

    println!("Creating files...");
    while i <= tasks {
        let file_name = format!("{}/{}_lab{}_Task{}.c", dir_path, std_id, lab_id, i);
        fs::write(file_name.clone(), lib::C_CODE)?;
        println!("[✔] {}", file_name.green());
        i += 1;
    }
    println!("{}","All files created successfully! 🎉".blue());
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("{}",lib::USAGE);
        return Ok(());
    }

    let std_id = args[1].parse::<i32>().expect("Invalid student ID");
    let lab_id = args[2].parse::<i32>().expect("Invalid lab number");
    let num_of_tasks = args[3].parse::<i32>().expect("Invalid number of tasks");


    fs::create_dir_all(format!("{}_Lab{}", std_id, lab_id))?;
    ops(std_id, lab_id, num_of_tasks).expect("Failed!");
    Ok(())
}
