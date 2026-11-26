use std::collections::VecDeque;

struct MyStack {
    queue: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        Self{
            queue: VecDeque::new()
        }
    }

    pub fn push(&mut self, x: i32) {
        self.queue.push_back(x);
    }

    pub fn pop(&mut self) -> i32 {
        self.queue.pop_back().unwrap()
    }

    pub fn top(&self) -> i32 {
        *self.queue.back().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}
