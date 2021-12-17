use std::marker::PhantomData;

use crate::*;

pub struct FoldedIter<'a, Key, Value, Folded>
where
    Value: ?Sized,
{
    pub(crate) folded: &'a Folded,
    pub(crate) next: Option<usize>,
    phantom_key: PhantomData<Key>,
    phantom_value: PhantomData<Value>,
}

impl<'a, Key, Value, ValueStorage> FoldedIter<'a, Key, Value, ValueStorage>
where
    Value: ?Sized,
{
    pub(crate) fn new(storage: &'a ValueStorage, next: Option<usize>) -> Self {
        Self {
            folded: storage,
            next,
            phantom_key: PhantomData,
            phantom_value: PhantomData,
        }
    }
}

impl<'a, Key, Value, ValueStorage> FoldedIter<'a, Key, Value, ValueStorage>
where
    ValueStorage: Folded<Key, Value, ValueStorage>,
    Value: ?Sized,
{
    pub fn new_iter(&self, next: Option<usize>) -> Self {
        Self {
            folded: self.folded,
            next,
            phantom_key: PhantomData,
            phantom_value: PhantomData,
        }
    }

    pub fn children(&self) -> Self {
        if let Some(next) = self.next {
            if next + 1 >= self.folded.nodes().len() {
                return self.new_iter(None);
            }
            let node = &self.folded.nodes()[next];
            if let Some(next_sibling) = node.next_sibling {
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

impl<'a, Key, Value: 'a, ValueStorage> Iterator for FoldedIter<'a, Key, Value, ValueStorage>
where
    ValueStorage: Folded<Key, Value, ValueStorage>,
    Value: ?Sized,
{
    type Item = (usize, &'a Value);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next {
            let node = &self.folded.nodes()[next];
            self.next = node.next_sibling;
            Some((next, self.folded.value(&node.key)))
        } else {
            None
        }
    }
}
