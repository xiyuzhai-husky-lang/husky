use std::marker::PhantomData;

use crate::*;

pub struct FoldedIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldedContainer<Value>,
{
    pub(crate) storage: &'a Storage,
    // pub indent: Indent,
    pub next: Option<usize>,
    phantom: PhantomData<Value>,
}

impl<'a, Value, Storage> std::fmt::Debug for FoldedIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldedContainer<Value>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("FoldedIter{{ next: {:?} }}", &self.next))
    }
}

impl<'a, Value, Storage> FoldedIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldedContainer<Value>,
{
    pub(crate) fn new(storage: &'a Storage, indent: Indent, next: Option<usize>) -> Self {
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
    Storage: FoldedContainer<Value>,
{
    pub fn next_level_iter(&self, next: Option<usize>) -> Self {
        Self {
            storage: self.storage,
            next,
            phantom: PhantomData,
        }
    }
}

impl<'a, Value: 'a, Storage> Iterator for FoldedIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldedContainer<Value>,
{
    type Item = (usize, Indent, &'a Value, Option<Self>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(idx) = self.next {
            let next_children = if idx + 1 >= self.storage.len() {
                None
            } else if self.storage.indent(idx + 1) > self.storage.indent(idx) {
                Some(self.next_level_iter(Some(idx + 1)))
            } else {
                None
            };
            self.next = self.storage.next_sibling(idx);
            #[cfg(test)]
            if let Some(raw) = self.next {
                assert!(raw < self.storage.len());
            }
            Some((
                idx,
                self.storage.indent(idx),
                self.storage.value(idx),
                next_children,
            ))
        } else {
            None
        }
    }
}
