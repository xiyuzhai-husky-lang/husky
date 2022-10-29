mod pool;

use pool::*;
use std::{borrow::Borrow, cell::Cell, marker::PhantomData};

pub struct RefArena<T, const N: usize> {
    pools: Cell<Vec<RefArenaPool<T, N>>>,
}

impl<T, const PoolCapacity: usize> RefArena<T, PoolCapacity> {
    pub fn new() -> Self {
        assert!(PoolCapacity > 0);
        Self {
            pools: Cell::new(vec![RefArenaPool::new()]),
        }
    }

    pub fn alloc<'a>(&'a self, t: T) -> &'a T {
        let data: &mut Vec<_> = unsafe { &mut *self.pools.as_ptr() };
        let ptr = match data.last_mut().unwrap().alloc(t) {
            Ok(ptr) => ptr,
            Err(RefArenaPoolError::RefArenaIsFull(t)) => {
                data.push(RefArenaPool::new());
                unsafe { data.last_mut().unwrap().alloc(t).unwrap_unchecked() }
            }
        };
        unsafe { wild_utils::arb_ref(&*ptr) }
    }

    pub fn reset(&mut self) {
        self.pools.get_mut().clear();
    }
}

#[test]
fn it_works() {
    let mut arena = RefArena::<i32, 2>::new();
    let a = arena.alloc(1);
    assert_eq!(a, &1);
    let b = arena.alloc(2);
    assert_eq!(b, &2);
    let c = arena.alloc(3);
    assert_eq!(a, &1);
    assert_eq!(b, &2);
    assert_eq!(c, &3);
    assert_eq!(arena.pools.get_mut().len(), 2);
}

#[test]
fn it_works2() {
    let mut arena = RefArena::<i32, 2>::new();
    for i in 0..10000 {
        assert_eq!(arena.alloc(i), &i)
    }
}
