use borsh_derive::{BorshDeserialize, BorshSerialize};
use chrono::Utc;
use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

/// Parses command line arguments for add command
pub fn parse_add_command(args: &[String]) -> Option<String> {
    if args.len() < 2 {
        eprintln!("Error: Please provide a task description");
        eprintln!("Usage: add <task description>");
        return None;
    }

    // Join all arguments after "add" to support multi-word tasks
    let task = args[1..].join(" ");
    
    // Remove surrounding quotes if present
    let task = task.trim_matches('"').trim_matches('\'').to_string();
    
    if task.is_empty() {
        eprintln!("Error: Task description cannot be empty");
        return None;
    }

    Some(task)
}