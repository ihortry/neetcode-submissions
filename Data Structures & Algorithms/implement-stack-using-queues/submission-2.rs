use std::collections::VecDeque;

struct MyStack {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        Self{
            q1: VecDeque::new(),
            q2: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.q2.push_back(x);

        while let Some(value) = self.q1.pop_front(){
            self.q2.push_back(value);
        }

        std::mem::swap(&mut self.q1, &mut self.q2);
    }

    pub fn pop(&mut self) -> i32 {
        self.q1.pop_front().unwrap()
    }

    pub fn top(&self) -> i32 {
        *self.q1.front().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.q1.is_empty()
    }
}
