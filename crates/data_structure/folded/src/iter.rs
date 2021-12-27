use std::marker::PhantomData;

use crate::*;

pub struct FoldedIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldedStorage<Value>,
{
    pub(crate) storage: &'a Storage,
    pub next: Option<usize>,
    phantom: PhantomData<Value>,
}

impl<'a, Value, Storage> std::fmt::Debug for FoldedIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldedStorage<Value>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("FoldedIter{{ next: {:?} }}", &self.next))
    }
}

impl<'a, Value, Storage> FoldedIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldedStorage<Value>,
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
    Storage: FoldedStorage<Value>,
{
    pub fn new_iter(&self, next: Option<usize>) -> Self {
        Self {
            storage: self.storage,
            next,
            phantom: PhantomData,
        }
    }

    fn next_children(&self) -> Option<Self> {
        if let Some(next) = self.next {
            if next + 1 >= self.storage.len() {
                return None;
            }
            if let Some(next_sibling) = self.storage.next_sibling(next) {
                if next + 1 >= next_sibling {
                    return None;
                }
            } else {
                return None;
            }
            Some(self.new_iter(Some(next + 1)))
        } else {
            None
        }
    }
}

impl<'a, Value: 'a, Storage> Iterator for FoldedIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldedStorage<Value>,
{
    type Item = (usize, &'a Value, Option<Self>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next {
            let next_children = self.next_children();
            self.next = self.storage.next_sibling(next);
            Some((next, self.storage.value(next), next_children))
        } else {
            None
        }
    }
}
