use std::io::{self, BufRead, Write};
use parsing::*;
use queue::*;
use todo::{STORAGE_FILE, TodoQueue};

use crate::handlers::{parsing, queue, todo};

/// Prints usage information
fn print_usage() {
    println!("\n=== Persistent Todo Queue ===");
    println!("Commands:");
    println!("  add <task>    - Add a new task");
    println!("  list          - List all pending tasks");
    println!("  done          - Complete the next task");
    println!("  help          - Show this help message");
    println!("  exit/quit     - Exit the application");
    println!();
}


mod handlers;


fn main() {
    let mut todo_queue = TodoQueue::new(STORAGE_FILE);

    println!("📝 Welcome to Persistent Todo Queue!");
    println!("Your tasks are saved to '{}' and will persist across restarts.", STORAGE_FILE);
    print_usage();

    let stdin = io::stdin();

    loop {
        print!("todo> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if stdin.lock().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let args: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        let command = args[0].to_lowercase();

        match command.as_str() {
            "add" => {
                if let Some(task) = parse_add_command(&args) {
                    let todo = todo_queue.add_task(task);
                    println!("✅ Task #{} added: \"{}\"", todo.id, todo.description);
                }
            }

            "list" => {
                let tasks = todo_queue.list_tasks();
                if tasks.is_empty() {
                    println!("📋 No pending tasks!");
                } else {
                    println!("\n📋 Pending Tasks ({}):", tasks.len());
                    println!("{}", "-".repeat(50));
                    for todo in tasks {
                        println!("  #{} - {}", todo.id, todo.description);
                    }
                    println!("{}", "-".repeat(50));
                }
            }

            "done" => {
                match todo_queue.complete_next() {
                    Some(todo) => {
                        println!("✅ Completed task #{}: \"{}\"", todo.id, todo.description);
                    }
                    None => {
                        println!("📋 No tasks to complete!");
                    }
                }
            }

            "help" => {
                print_usage();
            }

            "exit" | "quit" => {
                println!("👋 Goodbye!");
                break;
            }

            _ => {
                eprintln!("❌ Unknown command: '{}'", command);
                println!("Type 'help' for available commands.");
            }
        }
    }
}
