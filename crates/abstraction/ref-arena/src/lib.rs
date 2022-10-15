use std::{cell::Cell, marker::PhantomData};

use thiserror::Error;

pub struct RefArena<T> {
    data: Cell<Vec<T>>,
}

impl<T> RefArena<T> {
    pub fn new(capacity: usize) -> Self {
        let mut data = vec![];
        data.reserve_exact(capacity);
        assert_eq!(data.capacity(), 2);
        Self {
            data: Cell::new(data),
        }
    }

    pub fn alloc<'a>(&'a self, t: T) -> RefArenaResult<&'a T> {
        let data: &mut Vec<_> = unsafe { &mut *self.data.as_ptr() };
        assert_eq!(data.capacity(), 2);
        if data.len() == data.capacity() {
            return Err(RefArenaError::RefArenaIsFull);
        }
        data.push(t);
        Ok(unsafe { wild_utils::arb_ref(data.last().unwrap()) })
    }

    pub fn reset(&mut self) {
        self.data.get_mut().clear();
    }
}

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum RefArenaError {
    #[error("ref arena is full")]
    RefArenaIsFull,
}

pub type RefArenaResult<T> = Result<T, RefArenaError>;

#[test]
fn it_works() {
    let arena = RefArena::<i32>::new(2);
    let a = arena.alloc(1);
    assert_eq!(a, Ok(&1));
    let b = arena.alloc(2);
    assert_eq!(b, Ok(&2));
    let c = arena.alloc(3);
    assert_eq!(a, Ok(&1));
    assert_eq!(b, Ok(&2));
    assert_eq!(c, Err(RefArenaError::RefArenaIsFull));
}
