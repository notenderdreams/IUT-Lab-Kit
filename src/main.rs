use clap::{Parser, Subcommand};
use cliclack::{self, intro, outro, select, spinner};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::stdin;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new lab setup
    Init {
        /// Your student ID (e.g., 230041234)
        student_id: Option<String>,

        /// The lab number (e.g., 7)
        lab_number: Option<u32>,

        /// The number of tasks to create (e.g., 4)
        num_tasks: Option<u32>,
    },

    /// Clean up generated files
    Clean,

    /// Set test cases for a task
    Set {
        /// Task number to set test cases for
        task_number: u32,
    },

    /// Run tests for a task
    Run {
        /// Task number to run tests for
        task_number: u32,
    },
}

async fn get_user_input() -> Result<(String, u32, u32), Box<dyn std::error::Error>> {
    intro("IUT Lab Kit 🚀")?;

    let student_id = cliclack::input("Enter your student ID")
        .placeholder("230041234")
        .validate(|input: &String| {
            if input.len() < 5 {
                Err("Student ID must be at least 5 characters")
            } else {
                Ok(())
            }
        })
        .interact::<String>()?;

    let lab_number_str = cliclack::input("Enter lab number")
        .placeholder("1")
        .validate(|input: &String| {
            input
                .parse::<u32>()
                .map_err(|_| "Please enter a valid number")
                .and_then(|n| {
                    if n >= 1 {
                        Ok(())
                    } else {
                        Err("Number must be greater than 0")
                    }
                })
        })
        .interact::<String>()?;
    let lab_number = lab_number_str.parse::<u32>()?;

    let num_tasks_str = cliclack::input("Enter number of tasks")
        .placeholder("4")
        .validate(|input: &String| {
            input
                .parse::<u32>()
                .map_err(|_| "Please enter a valid number")
                .and_then(|n| {
                    if n >= 1 {
                        Ok(())
                    } else {
                        Err("Number must be greater than 0")
                    }
                })
        })
        .interact::<String>()?;
    let num_tasks = num_tasks_str.parse::<u32>()?;

    Ok((student_id, lab_number, num_tasks))
}

fn read_multiline_input(prompt: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let mut input = String::new();
    println!(
        "{} (Ctrl+C/Esc to cancel, Shift+Enter for newline, Enter to submit):",
        prompt
    );
    enable_raw_mode()?;

    loop {
        match read()? {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => {
                match code {
                    KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                        disable_raw_mode()?;
                        println!("\nCancelled");
                        return Ok(None);
                    }
                    KeyCode::Esc => {
                        disable_raw_mode()?;
                        println!("\nCancelled");
                        return Ok(None);
                    }
                    KeyCode::Enter => {
                        if modifiers.contains(KeyModifiers::SHIFT) {
                            input.push('\n');
                            print!("\r\n");
                        } else {
                            println!();
                            disable_raw_mode()?;
                            return Ok(Some(input));
                        }
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                        print!("{}", c);
                    }
                    KeyCode::Backspace => {
                        if !input.is_empty() {
                            input.pop();
                            print!("\x08 \x08");
                        }
                    }
                    _ => {}
                }
                std::io::stdout().flush()?;
            }
            _ => {}
        }
    }
}

async fn handle_set_command(task_number: u32) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = Path::new(".lab").join("config.json");
    if !config_path.exists() {
        outro("No lab configuration found. Please initialize a lab first.")?;
        return Ok(());
    }

    let config_str = fs::read_to_string(config_path.clone())?;
    let mut questions: Vec<Question> = serde_json::from_str(&config_str)?;

    let task_idx = task_number - 1;
    if task_idx as usize >= questions.len() {
        outro("Invalid task number.")?;
        return Ok(());
    }

    intro(&format!("Test Cases for Task {}", task_number))?;

    if questions[task_idx as usize].io.is_empty() {
        println!("No test cases have been added yet.");
    } else {
        println!("Current test cases:");
        for (i, test) in questions[task_idx as usize].io.iter().enumerate() {
            println!("{}. Input: {}, Output: {}", i + 1, test.input, test.output);
        }
    }

    let action = select("Choose an action")
        .item("add", "Add test case", "Add a new test case")
        .item("remove", "Remove test case", "Remove an existing test case")
        .interact()?;

    match action {
        "add" => loop {
            let input = cliclack::input("Enter input (use \\n for newlines)")
                .placeholder("e.g., 5 3\\n7 2")
                .interact::<String>()?
                .replace("\\n", "\n");

            let output = cliclack::input("Enter expected output (use \\n for newlines)")
                .placeholder("e.g., 8\\n9")
                .interact::<String>()?
                .replace("\\n", "\n");

            questions[task_idx as usize].io.push(TestCase {
                input: input.trim().to_string(),
                output: output.trim().to_string(),
            });

            let mut sp = spinner();
            sp.start("Saving test case...");
            fs::write(
                config_path.clone(),
                serde_json::to_string_pretty(&questions)?,
            )?;
            sp.stop("Test case added successfully! ✨");

            let continue_action = select("What would you like to do?")
                .item("add", "Add another test case", "Add more test cases")
                .item("exit", "Exit", "Return to main menu")
                .interact()?;

            if continue_action == "exit" {
                break;
            }
        },
        "remove" => {
            if questions[task_idx as usize].io.is_empty() {
                outro("No test cases to remove.")?;
                return Ok(());
            }

            let options: Vec<(String, usize)> = questions[task_idx as usize]
                .io
                .iter()
                .enumerate()
                .map(|(i, test)| (format!("Input: {}, Output: {}", test.input, test.output), i))
                .collect();

            let mut select = select("Select test case to remove");
            for (i, (label, idx)) in options.iter().enumerate() {
                select = select.item(idx.to_string(), label, "");
            }

            let selected = select.interact()?;
            let selected_idx = options
                .iter()
                .find(|(_, idx)| idx.to_string() == selected)
                .map(|(_, idx)| *idx)
                .unwrap();

            questions[task_idx as usize].io.remove(selected_idx);

            let mut sp = spinner();
            sp.start("Removing test case...");
            fs::write(config_path, serde_json::to_string_pretty(&questions)?)?;
            sp.stop("Test case removed successfully! 🗑️");
        }
        _ => unreachable!(),
    }

    outro("Operation completed successfully! ✨")?;
    Ok(())
}

async fn handle_run_command(task_number: u32) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = Path::new(".lab").join("config.json");
    if !config_path.exists() {
        outro("No lab configuration found. Please initialize a lab first.")?;
        return Ok(());
    }

    let config_str = fs::read_to_string(config_path)?;
    let questions: Vec<Question> = serde_json::from_str(&config_str)?;

    let task_idx = task_number - 1;
    if task_idx as usize >= questions.len() {
        outro("Invalid task number.")?;
        return Ok(());
    }

    let question = &questions[task_idx as usize];
    let file_path = &question.file_id;

    if !Path::new(file_path).exists() {
        outro(&format!("Source file {} not found!", file_path))?;
        return Ok(());
    }

    // Compile the C file
    let mut sp = spinner();
    sp.start("Compiling...");

    let output = Command::new("gcc")
        .arg(file_path)
        .arg("-o")
        .arg("temp_program")
        .output()?;

    if !output.status.success() {
        sp.stop("Compilation failed! ❌");
        println!("Compilation error:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
        return Ok(());
    }
    sp.stop("Compilation successful! ✨");

    println!("\nRunning tests...\n");
    let mut passed = 0;
    let total = question.io.len();

    for (i, test) in question.io.iter().enumerate() {
        println!("Test case #{}", i + 1);
        println!("Input: {}", test.input);

        let mut child = Command::new("./temp_program")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        // Write input to stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(test.input.as_bytes())?;
        }

        let output = child.wait_with_output()?;
        let actual_output = String::from_utf8_lossy(&output.stdout).trim().to_string();

        println!("Expected output: {}", test.output);
        println!("Actual output: {}", actual_output);

        if actual_output == test.output {
            println!("Status: ✅ Passed");
            passed += 1;
        } else {
            println!("Status: ❌ Failed");
        }
        println!();
    }

    // Clean up temporary executable
    let _ = fs::remove_file("temp_program");

    if total == 0 {
        outro("No test cases found! Add test cases using 'lab set' command.")?;
    } else {
        outro(&format!("Tests completed! {}/{} passed ✨", passed, total))?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init {
            student_id,
            lab_number,
            num_tasks,
        }) => {
            let rt = tokio::runtime::Runtime::new()?;
            let (student_id, lab_number, num_tasks) = match (student_id, lab_number, num_tasks) {
                (Some(id), Some(lab), Some(tasks)) => (id.clone(), *lab, *tasks),
                _ => rt.block_on(get_user_input())?,
            };

            let mut sp = spinner();
            sp.start("Creating lab files...");

            create_task_files(&student_id, lab_number, num_tasks)?;

            sp.stop("Lab files created ✨");
            outro("Lab initialized successfully! 🎉")?;
        }
        Some(Commands::Clean) => {
            let mut sp = spinner();
            sp.start("Cleaning up files...");

            // Clean implementation
            let pattern = format!("*_Lab*_Task*.c");
            match glob::glob(&pattern) {
                Ok(paths) => {
                    for entry in paths {
                        if let Ok(path) = entry {
                            if let Err(e) = std::fs::remove_file(&path) {
                                eprintln!("Error removing {}: {}", path.display(), e);
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Error matching files: {}", e),
            }

            if let Err(e) = fs::remove_dir_all(".lab") {
                if e.kind() != std::io::ErrorKind::NotFound {
                    eprintln!("Error removing .lab directory: {}", e);
                }
            }

            sp.stop("Cleanup complete 🧹");
        }
        Some(Commands::Set { task_number }) => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(handle_set_command(*task_number))?;
        }
        Some(Commands::Run { task_number }) => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(handle_run_command(*task_number))?;
        }
        None => {
            intro("IUT Lab Kit 🚀")?;
            outro("Use 'lab init' to create new lab files")?;
        }
    }
    Ok(())
}

fn create_task_files(student_id: &str, lab_number: u32, num_tasks: u32) -> std::io::Result<()> {
    for task_num in 1..=num_tasks {
        let filename = format!("{}_Lab{}_Task{}.c", student_id, lab_number, task_num);
        let mut file = File::create(Path::new(&filename))?;

        let template = format!(
            "#include <stdio.h>\n\n\
             int main() {{\n\
             \t// Your code here\n\
             \treturn 0;\n\
             }}\n"
        );

        file.write_all(template.as_bytes())?;
    }

    create_config_file(student_id, lab_number, num_tasks)?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct TestCase {
    input: String,
    output: String,
}

#[derive(Serialize, Deserialize)]
struct Question {
    file_id: String,
    io: Vec<TestCase>,
}

fn create_config_file(student_id: &str, lab_number: u32, num_tasks: u32) -> std::io::Result<()> {
    // Create .lab directory if it doesn't exist
    fs::create_dir_all(".lab")?;

    // Create questions vector with empty test cases
    let questions: Vec<Question> = (1..=num_tasks)
        .map(|i| Question {
            file_id: format!("{}_Lab{}_Task{}.c", student_id, lab_number, i),
            io: Vec::new(),
        })
        .collect();

    // Create the config file with pretty printing
    let config_path = Path::new(".lab").join("config.json");
    let config_json = serde_json::to_string_pretty(&questions)?;
    fs::write(config_path, config_json)?;

    println!("Created config file: .lab/config.json");
    Ok(())
}
