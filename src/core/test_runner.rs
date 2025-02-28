use colored::*;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::models::question::Question;

pub fn run_tests_for_task(
    question: &Question,
) -> Result<(usize, usize, Vec<(usize, String)>), Box<dyn std::error::Error>> {
    let file_path = &question.file_id;
    let mut failed_tests = Vec::new();

    if !Path::new(file_path).exists() {
        return Ok((0, 0, failed_tests));
    }

    // Compile the C file
    let mut sp = cliclack::spinner();
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
        return Ok((0, 0, failed_tests));
    }
    sp.stop("Compilation successful! ✨");

    println!("\nRunning tests...\n");
    let mut passed = 0;
    let total = question.io.len();

    for (i, test) in question.io.iter().enumerate() {
        println!("{}", format!("Test case #{}", i + 1).bold());
        println!("{}", "─".repeat(50));

        let mut child = Command::new("./temp_program")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(test.input.as_bytes())?;
        }

        let output = child.wait_with_output()?;
        let actual_output = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if actual_output == test.output {
            println!("{:>12} {}", "Input:".bold().blue(), test.input);
            println!("{:>12} {}", "Output:".bold().blue(), actual_output);
            println!("{:>12} {}", "Status:".bold(), "✅ Passed".green());
            passed += 1;
        } else {
            println!("{:>12} {}", "Input:".bold().blue(), test.input);
            println!("{:>12} {}", "Expected:".bold().yellow(), test.output);
            println!("{:>12} {}", "Actual:".bold().red(), actual_output);
            println!("{:>12} {}", "Status:".bold(), "❌ Failed".red());
            failed_tests.push((i + 1, question.file_id.clone()));
        }
        println!();
    }

    // Clean up temporary executable
    let _ = fs::remove_file("temp_program");

    Ok((passed, total, failed_tests))
}
