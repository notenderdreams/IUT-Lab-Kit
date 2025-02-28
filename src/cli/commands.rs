use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
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
        /// Task number to run tests for (optional)
        task_number: Option<u32>,
    },
}
