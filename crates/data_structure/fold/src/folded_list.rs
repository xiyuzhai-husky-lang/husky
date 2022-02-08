use std::{fmt::Debug, marker::PhantomData};

use common::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FoldedList<T> {
    pub nodes: Vec<FoldedNode<T>>,
}

pub struct FoldIdx<T> {
    raw: usize,
    phantom: PhantomData<T>,
}

impl<T> Debug for FoldIdx<T> {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FoldIdx").field("raw", &self.raw).finish()
    }
}

impl<T> Clone for FoldIdx<T> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            phantom: self.phantom.clone(),
        }
    }
}

impl<T> Copy for FoldIdx<T> {}

impl<T> From<usize> for FoldIdx<T> {
    fn from(raw: usize) -> Self {
        Self {
            raw,
            phantom: PhantomData,
        }
    }
}

impl<T> FoldedList<T> {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    pub fn nodes(&self) -> &[FoldedNode<T>] {
        &self.nodes
    }

    pub fn append(&mut self, indent: Indent, value: T, next_sibling: Option<usize>) {
        self.nodes.push(FoldedNode {
            indent,
            value,
            next_sibling,
        });
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct FoldedNode<T> {
    pub(crate) indent: Indent,
    pub value: T,
    pub(crate) next_sibling: Option<usize>,
}

impl<T> Debug for FoldedNode<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "FoldedNode{{indent: {:?}, value: {:?}, next_sibling: {:?}}}",
            &self.indent, &self.value, &self.next_sibling
        ))
    }
}

impl<T> FoldedNode<T> {
    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<Item, T> From<Vec<Item>> for FoldedList<T>
where
    Item: ItemToFold<T>,
{
    fn from(items: Vec<Item>) -> Self {
        let mut nodes = Vec::new();
        nodes.reserve(items.len());

        for i in 0..items.len() {
            let mut j = i + 1;
            let indent = items[i].indent();
            let next_sibling = loop {
                if j < items.len() {
                    let indent_below = items[j].indent();
                    if indent_below == indent {
                        break Some(j);
                    } else if indent_below < indent {
                        break None;
                    } else {
                        j += 1;
                    }
                } else {
                    break None;
                }
            };
            nodes.push(FoldedNode {
                indent: items[i].indent(),
                value: items[i].value(),
                next_sibling,
            })
        }

        Self { nodes }
    }
}

pub trait ItemToFold<Key> {
    fn value(&self) -> Key;
    fn indent(&self) -> Indent;
}

impl<T> FoldStorage<T> for FoldedList<T>
where
    T: Debug,
{
    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn next_sibling(&self, index: usize) -> Option<usize> {
        self.nodes[index].next_sibling
    }

    fn value(&self, index: usize) -> &T {
        &self.nodes[index].value
    }

    fn this(&self) -> &FoldedList<T> {
        self
    }

    fn indent(&self, index: usize) -> Indent {
        self.nodes[index].indent
    }
}
