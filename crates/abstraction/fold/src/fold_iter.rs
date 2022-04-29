use crate::*;
use check_utils::*;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct FoldIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldStorage<Value>,
{
    pub(crate) storage: &'a Storage,
    pub next: Option<usize>,
    phantom: PhantomData<Value>,
}

impl<'a, Value, Storage> std::fmt::Debug for FoldIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldStorage<Value>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("FoldIter{{ next: {:?} }}", &self.next))
    }
}

impl<'a, Value, Storage> FoldIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldStorage<Value>,
{
    pub(crate) fn new(storage: &'a Storage, next: Option<usize>) -> Self {
        if let Some(idx) = next {
            assert!(idx < storage.len())
        }
        Self {
            storage,
            next,
            phantom: PhantomData,
        }
    }
}

impl<'a, Value, Storage> FoldIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldStorage<Value>,
{
    pub fn next_level_iter(&self, next: Option<usize>) -> Self {
        Self {
            storage: self.storage,
            next,
            phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct FoldIterItem<'a, Value: 'a, Storage>
where
    Value: ?Sized,
    Storage: FoldStorage<Value>,
{
    pub idx: usize,
    pub indent: Indent,
    pub value: &'a Value,
    pub children: Option<FoldIter<'a, Value, Storage>>,
}

impl<'a, Value: 'a, Storage> Iterator for FoldIter<'a, Value, Storage>
where
    Value: ?Sized + 'a,
    Storage: FoldStorage<Value>,
{
    type Item = FoldIterItem<'a, Value, Storage>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(idx) = self.next {
            let children = if idx + 1 >= self.storage.len() {
                None
            } else if self.storage.indent(idx + 1) > self.storage.indent(idx) {
                Some(self.next_level_iter(Some(idx + 1)))
            } else {
                None
            };
            self.next = self.storage.next_sibling(idx);
            if let Some(raw) = self.next {
                should!(raw < self.storage.len());
            }
            Some(Self::Item {
                idx,
                indent: self.storage.indent(idx),
                value: self.storage.value(idx),
                children,
            })
        } else {
            None
        }
    }
}
