use std::sync::Arc;

use print_utils::p;

use crate::*;

impl ItemToFold<()> for Indent {
    fn value(&self) -> () {
        ()
    }

    fn indent(&self) -> Indent {
        *self
    }
}

#[test]
fn fold_items1() {
    use check_utils::*;
    let items: Vec<Indent> = vec![0, 4, 0].into();
    let fold_items: FoldableList<()> = items.into();
    p!(fold_items.nodes);
    should_eq!(fold_items.nodes[1].folding_end, FoldingEnd::Elder(2));
}

#[test]
fn fold_items2() {
    use check_utils::*;
    let items: Vec<Indent> = vec![0, 4, 0, 4, 4].into();
    let fold_items: FoldableList<()> = items.into();
    should!(fold_items
        .iter_from(1)
        .next()
        .unwrap()
        .opt_children
        .is_none());
    should_eq!(fold_items.nodes[3].folding_end, FoldingEnd::Sibling(4));
}

pub struct TrivialTransformer {
    fold_inputs: Arc<FoldableList<()>>,
    fold_outputs: FoldableList<()>,
}

impl<'a> Transformer<(), FoldableList<()>, ()> for TrivialTransformer {
    fn _enter_block(&mut self) {}

    fn _exit_block(&mut self) {}

    fn transform(
        &mut self,
        _indent: Indent,
        _input: &(),
        _enter_block: impl FnOnce(&mut Self),
    ) -> () {
    }

    fn foldable_outputs_mut(&mut self) -> &mut FoldableList<()> {
        &mut self.fold_outputs
    }

    fn foldable_inputs(&self) -> &FoldableList<()> {
        &self.fold_inputs
    }

    fn misplaced(&self) -> () {
        todo!()
    }
}

#[test]
fn transform() {
    use check_utils::*;
    use print_utils::*;
    let items: Vec<Indent> = vec![0, 4, 0, 4, 4].into();
    let fold_inputs: Arc<FoldableList<()>> = Arc::new(items.into());
    let mut transformer = TrivialTransformer {
        fold_inputs: fold_inputs.clone(),
        fold_outputs: FoldableList::<()>::new(),
    };
    should!(fold_inputs
        .iter_from(2)
        .next()
        .unwrap()
        .opt_children
        .is_some());
    transformer.transform_all_recr(fold_inputs.iter());
    should_eq!(transformer.fold_outputs.len(), 5);
}
