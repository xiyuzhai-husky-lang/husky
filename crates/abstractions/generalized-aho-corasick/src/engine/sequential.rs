//! linear means single-threaded
pub mod dfs;

use either::*;
use std::collections::VecDeque;

pub trait IsGacSequentialEngineInner<'sess> {
    type Node: Copy + Eq + std::hash::Hash + 'sess;
    type Input;
    type State: Copy + 'sess;
    type Output;

    fn roots(&self) -> impl IntoIterator<Item = Self::Node> + 'sess;
    fn children(&self, node: Self::Node) -> impl IntoIterator<Item = Self::Node> + 'sess;
    fn initial_state(&self, input: Self::Input) -> Self::State;
    /// Processes the current node with the given input state, potentially modifying internal state.
    ///
    /// This method allows the implementation to perform side effects on its internal state while processing,
    /// as long as these modifications don't affect the correctness of the algorithm. For example:
    /// - Caching intermediate results for performance optimization
    /// - Building up internal proof context in theorem provers
    /// - Maintaining statistics or debugging information
    ///
    /// # Returns
    /// - `Left`: New states to explore (continuation)
    /// - `Right`: Final output when a match is found
    fn process(
        &mut self,
        node: Self::Node,
        state: Self::State,
    ) -> Either<impl IntoIterator<Item = Self::State> + 'sess, Self::Output>;
}
