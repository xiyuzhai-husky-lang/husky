use crate::*;

use common::*;

impl ItemToFold<()> for u16 {
    fn value(&self) -> () {
        ()
    }

    fn indent(&self) -> u16 {
        *self
    }
}

#[test]
fn fold_items1() {
    let items: Vec<u16> = vec![0, 4, 0].into();
    let folded_items: FoldedList<()> = items.into();
    should_be!(folded_items.nodes[1].next_sibling, None);
}

#[test]
fn fold_items2() {
    let items: Vec<u16> = vec![0, 4, 0, 4, 4].into();
    let folded_items: FoldedList<()> = items.into();
    should!(folded_items
        .folded_iter(1)
        .next()
        .unwrap()
        .children
        .is_none());
    should_be!(folded_items.nodes[3].next_sibling, Some(4));
}

pub struct TrivialTransformer {
    folded_outputs: FoldedList<()>,
}

impl<'a> Transformer<(), FoldedList<()>, ()> for TrivialTransformer {
    fn enter(&mut self) {}

    fn exit(&mut self) {}

    fn post_exit(&mut self, task: ()) {}

    fn transform(&mut self, indent: Indent, input: &()) -> () {}

    fn folded_outputs_mut(&mut self) -> &mut FoldedList<()> {
        &mut self.folded_outputs
    }

    fn designate_task(&self, output: &()) -> Option<()> {
        None
    }
}

#[test]
fn transform() {
    let items: Vec<u16> = vec![0, 4, 0, 4, 4].into();
    let folded_items: FoldedList<()> = items.into();
    let mut transformer = TrivialTransformer {
        folded_outputs: FoldedList::<()>::new(),
    };
    should!(folded_items
        .folded_iter(2)
        .next()
        .unwrap()
        .children
        .is_some());
    for i in 0..folded_items.len() {
        let mut iter = folded_items.folded_iter(i);
        p!(i, iter, iter.next());
    }
    transformer.transform_all(folded_items.folded_iter(0));
    should_be!(transformer.folded_outputs.len(), 5);
    p!(transformer.folded_outputs);
}
