use crate::*;
use check_utils::*;
use print_utils::p;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct FoldableIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldableStorage<Value>,
{
    pub(crate) storage: &'a Storage,
    pub next: Option<usize>,
    phantom: PhantomData<Value>,
}

impl<'a, Value, Storage> std::fmt::Debug for FoldableIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldableStorage<Value>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("FoldIter{{ next: {:?} }}", &self.next))
    }
}

impl<'a, Value, Storage> FoldableIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldableStorage<Value>,
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

impl<'a, Value, Storage> FoldableIter<'a, Value, Storage>
where
    Value: ?Sized,
    Storage: FoldableStorage<Value>,
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
    Storage: FoldableStorage<Value>,
{
    pub idx: usize,
    pub indent: Indent,
    pub value: &'a Value,
    pub folding_end: FoldingEnd,
    pub opt_children: Option<FoldableIter<'a, Value, Storage>>,
}

impl<'a, Value: 'a, Storage> Iterator for FoldableIter<'a, Value, Storage>
where
    Value: ?Sized + 'a,
    Storage: FoldableStorage<Value>,
{
    type Item = FoldIterItem<'a, Value, Storage>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(idx) = self.next {
            let opt_children = if idx + 1 >= self.storage.len() {
                None
            } else if self.storage.indent(idx + 1) > self.storage.indent(idx) {
                Some(self.next_level_iter(Some(idx + 1)))
            } else {
                None
            };
            self.next = self.storage.next_sibling_idx(idx);
            if let Some(raw) = self.next {
                should!(raw < self.storage.len());
            }
            Some(Self::Item {
                idx,
                indent: self.storage.indent(idx),
                value: self.storage.value(idx),
                opt_children,
                folding_end: self.storage.folding_end(idx),
            })
        } else {
            None
        }
    }
}
