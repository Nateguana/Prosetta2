use parking_lot::Mutex;

use super::commands::Command;
use std::cell::RefCell;

struct CommandArena {
    arena: Vec<Box<dyn Command>>,
}

impl CommandArena {
    fn new() -> Self {
        Self { arena: Vec::new() }
    }
    fn add(&mut self, command: impl Command + 'static) {
        self.arena.push(Box::new(command));
    }
    fn remove_until(&mut self, index:usize) {
        self.arena.drain(index..);
    }
}
