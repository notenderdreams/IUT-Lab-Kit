
mod cli;
mod core;
mod models;
mod utils;

use clap::Parser;
use cliclack::{intro, outro};

use cli::commands::{Cli, Commands};
use core::{config, test_runner};
use utils::file_handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init { student_id, lab_number, num_tasks }) => {
            let (student_id, lab_number, num_tasks) = match (student_id, lab_number, num_tasks) {
                (Some(id), Some(lab), Some(tasks)) => (id.clone(), *lab, *tasks),
                _ => config::get_user_input().await?,
            };

            let mut sp = cliclack::spinner();
            sp.start("Creating lab files...");

            file_handler::create_task_files(&student_id, lab_number, num_tasks)?;

            sp.stop("Lab files created ✨");
            outro("Lab initialized successfully! 🎉")?;
        }
        Some(Commands::Clean) => {
            let mut sp = cliclack::spinner();
            sp.start("Cleaning up files...");
            file_handler::clean_files()?;
            sp.stop("Cleanup complete 🧹");
        }
        Some(Commands::Set { task_number }) => {
            config::handle_set_command(*task_number).await?;
        }
        Some(Commands::Run { task_number }) => {
            config::handle_run_command(*task_number).await?;
        }
        None => {
            intro("IUT Lab Kit 🚀")?;
            outro("Use 'lab init' to create new lab files")?;
        }
    }
    Ok(())
}