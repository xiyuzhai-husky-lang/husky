use crate::*;

use common::*;

impl ItemToFold<()> for Indent {
    fn value(&self) -> () {
        ()
    }

    fn indent(&self) -> Indent {
        0
    }
}

#[test]
fn fold_items1() {
    let items: Vec<Indent> = vec![0, 4, 0].into();
    let fold_items: FoldedList<()> = items.into();
    should_eq!(fold_items.nodes[1].next_sibling, None);
}

#[test]
fn fold_items2() {
    let items: Vec<Indent> = vec![0, 4, 0, 4, 4].into();
    let fold_items: FoldedList<()> = items.into();
    should!(fold_items.fold_iter(1).next().unwrap().children.is_none());
    should_eq!(fold_items.nodes[3].next_sibling, Some(4));
}

pub struct TrivialTransformer {
    fold_outputs: FoldedList<()>,
}

impl<'a> Transformer<(), FoldedList<()>, ()> for TrivialTransformer {
    fn _enter_block(&mut self) {}

    fn _exit_block(&mut self) {}

    fn transform(
        &mut self,
        _indent: Indent,
        _input: &(),
        _enter_block: impl FnOnce(&mut Self),
    ) -> () {
    }

    fn folded_output_mut(&mut self) -> &mut FoldedList<()> {
        &mut self.fold_outputs
    }
}

#[test]
fn transform() {
    let items: Vec<Indent> = vec![0, 4, 0, 4, 4].into();
    let fold_items: FoldedList<()> = items.into();
    let mut transformer = TrivialTransformer {
        fold_outputs: FoldedList::<()>::new(),
    };
    should!(fold_items.fold_iter(2).next().unwrap().children.is_some());
    for i in 0..fold_items.len() {
        let mut iter = fold_items.fold_iter(i);
        test_print!(i, iter, iter.next());
    }
    transformer.transform_all(fold_items.fold_iter(0));
    should_eq!(transformer.fold_outputs.len(), 5);
}
