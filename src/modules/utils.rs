use std::{fs, io};
use std::io::Write;
use colored::Colorize;

pub static C_CODE: &str = r#"
#include <stdio.h>

int main(){


    return 0;
}
"#;


pub const USAGE: &str = r#"
Usage: lab [student_id] [lab_number] [number_of_tasks]

Arguments:
    student_id    Your student ID (e.g., 230041234)
    lab_number    The lab number (e.g., 7)
    number_of_tasks    The number of tasks to create (e.g., 4)

Example:
    lab 230041234 7 4
"#;


//functions

pub fn str_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}


pub fn ops(std_id: i32, lab_id: i32, tasks: i32, use_current_dir: bool) -> std::io::Result<()> {
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
        fs::write(file_name.clone(), C_CODE)?;  // Using the C_CODE static variable as content
        println!("[✔] {}", file_name.green());
    }
    println!("{}","All files created successfully! 🎉".blue());
    Ok(())
}

