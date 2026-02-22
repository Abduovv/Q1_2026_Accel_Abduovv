# Persistent Todo Queue

A CLI-based Todo application with a persistent FIFO queue using Borsh serialization.

## Features

- ✅ Add tasks to a FIFO queue
- 📋 List all pending tasks
- ✔️ Complete tasks in order (FIFO)
- 💾 Persistent storage using Borsh serialization
- 🔄 Tasks survive application restarts

## Project Structure

```
persistent-todo-queue/
├── Cargo.toml
├── src/
│   └── main.rs
└── todos.bin (created at runtime)
```

## Data Model

### Todo Struct

```rust
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
struct Todo {
    id: u64,
    description: String,
    created_at: u64,
}
```

### Generic Queue

```rust
pub struct Queue<T> {
    items: VecDeque<T>,
}
```

Supports:
- `enqueue(item)` - Add item to back
- `dequeue()` - Remove and return front item
- `peek()` - View front item without removing
- `len()` - Get queue size
- `is_empty()` - Check if empty

## CLI Commands

### Add a Task
```bash
todo add "Buy groceries"
```
Adds a task to the queue.

### List All Tasks
```bash
todo list
```
Print all pending tasks in FIFO order.

### Complete Next Task
```bash
todo done
```
Removes the oldest task from the queue and marks it as completed.

### Help
```bash
todo help
```
Show available commands.

### Exit
```bash
todo exit
# or
todo quit
```

## Building

```bash
cd persistent-todo-queue
cargo build --release
```

## Running

```bash
cargo run
```

## Example Session

```
📝 Welcome to Persistent Todo Queue!
Your tasks are saved to 'todos.bin' and will persist across restarts.

todo> add "Buy groceries"
✅ Task #1 added: "Buy groceries"

todo> add "Walk the dog"
✅ Task #2 added: "Walk the dog"

todo> add "Finish project"
✅ Task #3 added: "Finish project"

todo> list

📋 Pending Tasks (3):
--------------------------------------------------
  #1 - Buy groceries
  #2 - Walk the dog
  #3 - Finish project
--------------------------------------------------

todo> done
✅ Completed task #1: "Buy groceries"

todo> list

📋 Pending Tasks (2):
--------------------------------------------------
  #2 - Walk the dog
  #3 - Finish project
--------------------------------------------------

todo> exit
👋 Goodbye!
```

## Persistence

Tasks are stored in `todos.bin` using Borsh binary serialization. The file is:
- Created automatically on first task addition
- Updated after every add/done operation
- Loaded on application startup
- Fully cross-platform compatible

## Dependencies

- **borsh** (1.5) - Binary serialization framework
- **borsh-derive** (1.5) - Derive macros for Borsh
- **chrono** (0.4) - Date/time handling for task timestamps
