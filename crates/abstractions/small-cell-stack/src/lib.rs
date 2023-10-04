#![feature(generic_const_exprs)]
mod local_key;

pub use self::local_key::*;

use smallvec::{Array, SmallVec};
use std::{cell::UnsafeCell, thread::LocalKey};

pub struct SmallCellStack<A: Array>(UnsafeCell<SmallVec<A>>);

impl<A: Array> SmallCellStack<A> {
    // gives the last item copied
    pub fn read(&self) -> Option<A::Item>
    where
        A::Item: Copy,
    {
        unsafe {
            let inner = &*self.0.get();
            inner.last().copied()
        }
    }

    pub fn push(&self, item: A::Item) {
        unsafe {
            let inner = &mut *self.0.get();
            inner.push(item)
        }
    }

    pub fn pop(&self) -> Option<A::Item> {
        unsafe {
            let inner = &mut *self.0.get();
            inner.pop()
        }
    }
}

mod sealed {
    pub trait Sealed {}
}
