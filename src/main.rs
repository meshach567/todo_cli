use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    #[serde(default)]
    done: bool,
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Todo CLI")]
#[command(about = "A simple command-line to-do app", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    List,
    Done { index: usize },
}

fn load_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let file = fs::read_to_string("tasks.json").unwrap_or("[]".to_string());
    let tasks: Vec<Task> = serde_json::from_str(&file)?;
    Ok(tasks)
}

fn save_tasks(tasks: &Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(tasks)?;
    fs::write("tasks.json", json)?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let mut tasks = load_tasks().expect("Could not load tasks");

    match cli.command {
        Commands::Add { task } => {
            tasks.push(Task { description: task, done: false });
            save_tasks(&tasks).expect("Failed to save tasks");
            println!("Task added.");
        }
        Commands::List => {
            for (i, task) in tasks.iter().enumerate() {
                let status = if task.done { "[x]" } else { "[ ]" };
                println!("{}: {} {}", i, status, task.description);
            }
        }
        Commands::Done { index } => {
            if let Some(task) = tasks.get_mut(index) {
                task.done = true;
                save_tasks(&tasks).expect("Failed to save tasks");
                println!("Task marked as done.");
            } else {
                println!("Task not found.");
            }
        }
    }
}

