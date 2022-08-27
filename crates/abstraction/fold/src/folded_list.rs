use std::{fmt::Debug, marker::PhantomData};

use husky_check_utils::should_eq;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FoldableList<T> {
    pub nodes: Vec<FoldedNode<T>>,
}

pub struct FoldIdx<T> {
    raw: usize,
    phantom: PhantomData<T>,
}

impl<T> Debug for FoldIdx<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl<T> FoldableList<T> {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    pub fn nodes(&self) -> &[FoldedNode<T>] {
        &self.nodes
    }

    pub fn append(&mut self, indent: Indent, value: T, folding_end: FoldingEnd) {
        self.nodes.push(FoldedNode {
            indent,
            value,
            folding_end,
        });
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct FoldedNode<T> {
    pub indent: Indent,
    pub value: T,
    pub folding_end: FoldingEnd,
}

impl<T> Debug for FoldedNode<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "FoldedNode{{indent: {:?}, value: {:?}, next_sibling: {:?}}}",
            &self.indent, &self.value, &self.folding_end
        ))
    }
}

impl<T> FoldedNode<T> {
    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<Item, T> From<Vec<Item>> for FoldableList<T>
where
    Item: ItemToFold<T>,
{
    fn from(items: Vec<Item>) -> Self {
        let mut nodes = Vec::new();
        nodes.reserve(items.len());

        for i in 0..items.len() {
            let mut j = i + 1;
            let indent = items[i].indent();
            let folding_end = loop {
                if j < items.len() {
                    let indent_below = items[j].indent();
                    if indent_below == indent {
                        break FoldingEnd::Sibling(j);
                    } else if indent_below < indent {
                        break FoldingEnd::Elder(j);
                    } else {
                        j += 1;
                    }
                } else {
                    break FoldingEnd::EOF;
                }
            };
            nodes.push(FoldedNode {
                indent: items[i].indent(),
                value: items[i].value(),
                folding_end,
            })
        }

        should_eq!(nodes.len(), items.len());

        Self { nodes }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldingEnd {
    Sibling(usize),
    Elder(usize),
    EOF,
}

pub trait ItemToFold<Key> {
    fn value(&self) -> Key;
    fn indent(&self) -> Indent;
}

impl<T> FoldableStorage for FoldableList<T>
where
    T: Debug,
{
    type Value = T;
    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn folding_end(&self, index: usize) -> FoldingEnd {
        self.nodes[index].folding_end
    }

    fn value(&self, index: usize) -> &T {
        &self.nodes[index].value
    }

    fn this(&self) -> &FoldableList<T> {
        self
    }

    fn indent(&self, index: usize) -> Indent {
        self.nodes[index].indent
    }
}
