use cliclack::{input, intro, outro, select};
use colored::Colorize;
use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};
use serde_json;
use std::{fs, io::stdout, path::Path};

use crate::models::question::Question;

pub async fn get_user_input() -> Result<(String, u32, u32), Box<dyn std::error::Error>> {
    intro("IUT Lab Kit 🚀")?;

    let student_id = input("Enter your student ID")
        .placeholder("230041234")
        .validate(|input: &String| {
            if input.len() < 5 {
                Err("Student ID must be at least 5 characters")
            } else {
                Ok(())
            }
        })
        .interact::<String>()?;

    let lab_number = input("Enter lab number")
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
        .interact::<String>()?
        .parse()?;

    let num_tasks = input("Enter number of tasks")
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
        .interact::<String>()?
        .parse()?;

    Ok((student_id, lab_number, num_tasks))
}

pub async fn handle_set_command(task_number: u32) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = Path::new(".lab").join("config.json");
    if !config_path.exists() {
        outro("Config file not found. Run 'lab init' first!")?;
        return Ok(());
    }

    let mut questions: Vec<Question> = serde_json::from_str(&fs::read_to_string(&config_path)?)?;
    let question_index = (task_number - 1) as usize;
    let question = questions
        .get_mut(question_index)
        .ok_or("Invalid task number")?;

    intro("Set Test Cases")?;
    enable_raw_mode()?;

    loop {
        stdout().execute(Clear(ClearType::All))?;
        println!("\nCurrent test cases:");
        for (i, test) in question.io.iter().enumerate() {
            println!(
                "{} Input: {}, Output: {}",
                format!("{}.", i + 1).blue(),
                test.input,
                test.output
            );
        }

        println!("\n{}", "Commands:".blue());
        println!("a - Add test case");
        println!("d - Delete test case");
        println!("r - Run tests");
        println!("q - Save and quit");

        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('a'),
                ..
            }) => {
                disable_raw_mode()?;
                let input_prompt = input("Enter test input")
                    .placeholder("5 2")
                    .interact::<String>()?;

                let output_prompt = input("Enter expected output")
                    .placeholder("10")
                    .interact::<String>()?;

                question.io.push(crate::models::question::TestCase {
                    input: input_prompt,
                    output: output_prompt,
                });
                enable_raw_mode()?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('d'),
                ..
            }) => {
                if !question.io.is_empty() {
                    disable_raw_mode()?;
                    let indices: Vec<(usize, String, String)> = question
                        .io
                        .iter()
                        .enumerate()
                        .map(|(i, test)| {
                            (
                                i,
                                format!("Test {}", i + 1),
                                format!("Input: {}, Output: {}", test.input, test.output),
                            )
                        })
                        .collect();
                    let index = select("Select test case to delete")
                        .items(&indices)
                        .interact()?;
                    question.io.remove(index);
                    enable_raw_mode()?;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('r'),
                ..
            }) => {
                disable_raw_mode()?;
                let test_question = questions[question_index].clone();
                fs::write(&config_path, serde_json::to_string_pretty(&questions)?)?;
                let (_passed, _total, _failed) =
                    crate::core::test_runner::run_tests_for_task(&test_question)?;
                outro("Test cases saved and executed! ✨")?;
                return Ok(());
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => break,
            _ => {}
        }
    }

    disable_raw_mode()?;
    fs::write(config_path, serde_json::to_string_pretty(&questions)?)?;
    outro("Test cases saved successfully! ✨")?;
    Ok(())
}

pub async fn handle_run_command(
    task_number: Option<u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = Path::new(".lab").join("config.json");
    if !config_path.exists() {
        outro("Config file not found. Run 'lab init' first!")?;
        return Ok(());
    }

    let questions: Vec<Question> = serde_json::from_str(&fs::read_to_string(&config_path)?)?;
    let mut total_passed = 0;
    let mut total_tests = 0;
    let mut failed_tests = Vec::new();

    match task_number {
        Some(task_num) => {
            let question = questions
                .get((task_num - 1) as usize)
                .ok_or("Invalid task number")?;
            let (passed, total, failed) = crate::core::test_runner::run_tests_for_task(question)?;
            total_passed += passed;
            total_tests += total;
            failed_tests.extend(failed);
        }
        None => {
            for question in questions.iter() {
                let (passed, total, failed) =
                    crate::core::test_runner::run_tests_for_task(question)?;
                total_passed += passed;
                total_tests += total;
                failed_tests.extend(failed);
            }
        }
    }

    println!("\n{}", "Test Summary".bold());
    println!("{}", "─".repeat(50));
    println!(
        "{:<11}{:>4} {}",
        "Total Tests".bold().blue(),
        ":".bold().blue(),
        total_tests
    );
    println!(
        "{:<11}{:>4} {}",
        "Tests Passed".bold().blue(),
        ":".bold().blue(),
        total_passed.to_string().green()
    );
    println!(
        "{:<11}{:>4} {}",
        "Tests Failed".bold().blue(),
        ":".bold().blue(),
        (total_tests - total_passed).to_string().red()
    );

    Ok(())
}
