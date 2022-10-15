use std::{cell::RefCell, marker::PhantomData};

use thiserror::Error;

#[derive(Clone, PartialEq, Eq)]
pub struct RefArena<T, const CAPACITY: usize> {
    data: RefCell<Vec<T>>,
}

impl<T, const CAPACITY: usize> RefArena<T, CAPACITY> {
    pub fn new() -> Self {
        let mut data = vec![];
        data.reserve(CAPACITY);
        Self {
            data: RefCell::new(data),
        }
    }

    pub fn alloc<'a>(&'a self, t: T) -> RefArenaResult<ArenaRef<'a, T>> {
        let data: &mut Vec<_> = &mut *self.data.borrow_mut();
        if data.len() == CAPACITY {
            return Err(RefArenaError::RefArenaIsFull);
        }
        data.push(t);
        Ok(ArenaRef(unsafe {
            wild_utils::arb_ref(data.last().unwrap())
        }))
    }
}

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum RefArenaError {
    #[error("ref arena is full")]
    RefArenaIsFull,
}

pub type RefArenaResult<T> = Result<T, RefArenaError>;

pub struct ArenaRef<'a, T>(&'a T);

impl<'a, T> std::ops::Deref for ArenaRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[test]
fn it_works() {
    todo!()
}
