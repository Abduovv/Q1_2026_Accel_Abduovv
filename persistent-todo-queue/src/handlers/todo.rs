use borsh_derive::{BorshDeserialize, BorshSerialize};
use chrono::Utc;
use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;
use crate::Queue;

pub const STORAGE_FILE: &str = "todos.bin";

/// Represents a single todo item with metadata
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Todo {
    pub(crate) id: u64,
    pub(crate) description: String,
    created_at: u64,
}

/// TodoQueue manages persistence for the todo queue
pub struct TodoQueue {
    queue: Queue<Todo>,
    next_id: u64,
    storage_path: String,
}

impl TodoQueue {
    /// Creates a new TodoQueue, loading from disk if available
    pub fn new(storage_path: &str) -> Self {
        let mut queue = TodoQueue {
            queue: Queue::new(),
            next_id: 1,
            storage_path: storage_path.to_string(),
        };

        if Path::new(storage_path).exists() {
            if let Err(e) = queue.load() {
                eprintln!("Warning: Could not load existing todos: {}", e);
            }
        }

        queue
    }

    /// Adds a new task to the queue
    pub fn add_task(&mut self, description: String) -> &Todo {
        let todo = Todo {
            id: self.next_id,
            description,
            created_at: Utc::now().timestamp() as u64,
        };

        self.next_id += 1;
        self.queue.enqueue(todo);
        self.save().expect("Failed to save todos");

        self.queue.peek().unwrap()
    }

    /// Completes (removes) the next task in the queue
    pub fn complete_next(&mut self) -> Option<Todo> {
        let completed = self.queue.dequeue();
        if completed.is_some() {
            self.save().expect("Failed to save todos");
        }
        completed
    }

    /// Returns all pending tasks
    pub fn list_tasks(&self) -> Vec<&Todo> {
        self.queue.to_vec()
    }

    /// Saves the queue to disk using Borsh serialization
    pub fn save(&self) -> io::Result<()> {
        let mut file = File::create(&self.storage_path)?;
        let encoded = borsh::to_vec(&(&self.queue, self.next_id))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        file.write_all(&encoded)?;
        file.sync_all()?;
        Ok(())
    }

    /// Loads the queue from disk using Borsh deserialization
    pub fn load(&mut self) -> io::Result<()> {
        let data = fs::read(&self.storage_path)?;
        let (queue, next_id): (Queue<Todo>, u64) = borsh::from_slice(&data)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        self.queue = queue;
        self.next_id = next_id;
        Ok(())
    }
}