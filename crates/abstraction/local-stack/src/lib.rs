mod find;

use find::Find;
use std::ops::Deref;

#[derive(Debug)]
pub struct LocalStack<T, Prelude = ()> {
    prelude: Prelude,
    stack: Vec<T>,
    local_indices: Vec<usize>,
}

#[test]
fn test_size() {
    struct A {
        _a: (),
    }

    assert_eq!(std::mem::size_of::<A>(), 0);
}

impl<T> Default for LocalStack<T> {
    fn default() -> Self {
        Self::new_empty()
    }
}

impl<T, Prelude> LocalStack<T, Prelude> {
    pub fn new_empty() -> Self
    where
        Prelude: Default,
    {
        Self {
            stack: Vec::new(),
            local_indices: vec![0],
            prelude: Default::default(),
        }
    }

    pub fn new(prelude: Prelude) -> Self {
        Self {
            stack: Vec::new(),
            local_indices: vec![0],
            prelude,
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

    // pub fn locals(&self) -> &[T] {
    //     &self.stack[(*self.local_indices.last().unwrap())..(self.stack.len())]
    // }

    // pub fn for_all_local(&self, f: impl Fn(&T) -> bool) -> bool {
    //     self.locals().iter().all(f)
    // }

    // pub fn for_any_local(&self, f: impl Fn(&T) -> bool) -> bool {
    //     self.locals().iter().any(f)
    // }

    pub fn find_last(&self, f: impl Fn(&T) -> bool) -> Option<&T>
    where
        Prelude: Find<T>,
    {
        if let Some(t) = self.stack.iter().rev().find(|item| f(*item)) {
            return Some(t);
        }
        self.prelude.find_last(f)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.stack.iter_mut()
    }
}

impl<T> Deref for LocalStack<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.stack
    }
}

impl<T> std::ops::Index<usize> for LocalStack<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.stack[index]
    }
}

impl<T> std::ops::IndexMut<usize> for LocalStack<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.stack[index]
    }
}
