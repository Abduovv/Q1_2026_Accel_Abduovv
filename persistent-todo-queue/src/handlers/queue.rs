use borsh_derive::{BorshDeserialize, BorshSerialize};
use chrono::Utc;
use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

/// Generic FIFO queue implementation for any serializable type
#[derive(Debug, Default, BorshSerialize, BorshDeserialize)]
pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    /// Creates a new empty queue
    pub fn new() -> Self {
        Queue {
            items: VecDeque::new(),
        }
    }

    /// Adds an item to the back of the queue
    pub fn enqueue(&mut self, item: T) {
        self.items.push_back(item);
    }

    /// Removes and returns the front item from the queue
    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    /// Returns a reference to the front item without removing it
    pub fn peek(&self) -> Option<&T> {
        self.items.front()
    }

    /// Returns the number of items in the queue
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns all items as a vector (for listing)
    pub fn to_vec(&self) -> Vec<&T> {
        self.items.iter().collect()
    }
}