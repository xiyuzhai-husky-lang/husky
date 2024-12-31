//! linear means single-threaded
pub mod dfs;

use either::*;
use std::collections::VecDeque;

pub trait IsGacSequentialEngineInner<'sess> {
    type Node: Copy + Eq + std::hash::Hash + 'sess;
    type Input;
    type State: Copy + 'sess;
    type Output;
    type Error: 'sess;

    fn root(&self) -> Self::Node;
    fn children(&self, node: Self::Node) -> &'sess [Self::Node];
    fn initial_state(&self, input: Self::Input) -> Self::State;
    fn process(
        &mut self,
        node: Self::Node,
        input: &Self::State,
    ) -> Either<Vec<Self::State>, Self::Error>;
}
