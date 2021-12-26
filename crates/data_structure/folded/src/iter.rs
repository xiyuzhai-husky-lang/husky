use std::marker::PhantomData;

use crate::*;

pub struct FoldedIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldedStorage<Value, Storage>,
{
    pub(crate) storage: &'a Storage,
    pub next: Option<usize>,
    phantom: PhantomData<Value>,
}

impl<'a, Value, Storage> FoldedIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldedStorage<Value, Storage>,
{
    pub(crate) fn new(storage: &'a Storage, next: Option<usize>) -> Self {
        Self {
            storage,
            next,
            phantom: PhantomData,
        }
    }
}

impl<'a, Value, Storage> FoldedIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldedStorage<Value, Storage>,
{
    pub fn new_iter(&self, next: Option<usize>) -> Self {
        Self {
            storage: self.storage,
            next,
            phantom: PhantomData,
        }
    }

    pub fn children(&self) -> Self {
        if let Some(next) = self.next {
            if next + 1 >= self.storage.len() {
                return self.new_iter(None);
            }
            if let Some(next_sibling) = self.storage.next_sibling(next) {
                if next + 1 >= next_sibling {
                    return self.new_iter(None);
                }
            }
            self.new_iter(Some(next + 1))
        } else {
            self.new_iter(None)
        }
    }
}

impl<'a, Value: 'a, Storage> Iterator for FoldedIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldedStorage<Value, Storage>,
{
    type Item = (usize, &'a Value);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next {
            self.next = self.storage.next_sibling(next);
            Some((next, self.storage.value(next)))
        } else {
            None
        }
    }
}
