use std::ops::Deref;

#[derive(Debug)]
pub struct LocalStack<T> {
    stack: Vec<T>,
    local_indices: Vec<usize>,
}

impl<T> LocalStack<T> {
    pub fn new() -> LocalStack<T> {
        Self {
            stack: Vec::new(),
            local_indices: vec![0],
        }
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    pub fn extend(&mut self, iter: impl Iterator<Item = T>) {
        self.stack.extend(iter);
    }

    pub fn enter(&mut self) {
        self.local_indices.push(self.stack.len());
    }

    pub fn exit(&mut self) {
        let block_index = self.local_indices.pop().unwrap();
        self.stack.truncate(block_index);
    }

    pub fn locals(&self) -> &[T] {
        &self.stack[(*self.local_indices.last().unwrap())..(self.stack.len())]
    }

    pub fn for_all_local(&self, f: impl Fn(&T) -> bool) -> bool {
        self.locals().iter().all(f)
    }

    pub fn for_any_local(&self, f: impl Fn(&T) -> bool) -> bool {
        self.locals().iter().any(f)
    }

    pub fn find(&self, f: impl Fn(&T) -> bool) -> Option<&T> {
        self.stack.iter().rev().find(|item| f(*item))
    }
}

impl<T> Deref for LocalStack<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.stack
    }
}
