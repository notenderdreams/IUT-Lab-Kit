mod modules;
use modules::config::save_config;
use modules::utils::{ops, cleanup, USAGE};
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "help" {
        println!("{}", USAGE);
        return Ok(());
    }

    if args.len() == 2 && args[1] == "clean" {
        cleanup().expect("Cleanup Failed!");
        return Ok(());
    }

    if args.len() == 1 {
        let _ = save_config();
        return Ok(());
    }

    if args.len() < 4 || args.len() > 5 {
        println!("{}", USAGE);
        return Ok(());
    }

    let std_id = args[1].parse::<i32>().expect("Invalid student ID");
    let lab_id = args[2].parse::<i32>().expect("Invalid lab number");
    let num_of_tasks = args[3].parse::<i32>().expect("Invalid number of tasks");

    let use_current_dir = args.len() == 5 && args[4] == ".";

    ops(std_id, lab_id, num_of_tasks, use_current_dir).expect("Failed to create files");

    Ok(())
}
