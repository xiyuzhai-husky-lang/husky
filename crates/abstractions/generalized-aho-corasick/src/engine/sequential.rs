//! linear means single-threaded
pub mod dfs;

use either::*;
use std::collections::VecDeque;

pub trait IsGacSequentialEngineInner<'sess> {
    type Node: Copy + Eq + std::hash::Hash + 'sess;
    type Input;
    type State: Copy + 'sess;
    type Output;

    fn root(&self) -> Self::Node;
    fn children(&self, node: Self::Node) -> impl IntoIterator<Item = Self::Node> + 'sess;
    fn initial_state(&self, input: Self::Input) -> Self::State;
    fn process(
        &mut self,
        node: Self::Node,
        input: &Self::State,
    ) -> Either<impl IntoIterator<Item = Self::State> + 'sess, Self::Output>;
}
