use super::*;
use alt_option::*;
use std::marker::PhantomData;

pub struct GacDfsSequentialEngine<'sess, Inner>
where
    Inner: IsGacSequentialEngineInner<'sess>,
{
    inner: Inner,
    _phantom: PhantomData<&'sess ()>,
}

impl<'sess, Inner: IsGacSequentialEngineInner<'sess>> GacDfsSequentialEngine<'sess, Inner> {
    pub fn new(inner: Inner) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }
}

impl<'sess, Inner> GacDfsSequentialEngine<'sess, Inner>
where
    Inner: IsGacSequentialEngineInner<'sess>,
{
    pub fn search(mut self, input: Inner::Input) -> Option<Inner::Output> {
        let state = self.inner.initial_state(input);
        self.search_nodes(state, self.inner.roots()).into()
    }

    fn search_nodes(
        &mut self,
        state: Inner::State,
        nodes: impl IntoIterator<Item = Inner::Node>,
    ) -> AltOption<Inner::Output> {
        for node in nodes {
            self.search_node(state, node)?;
        }
        AltNone
    }

    fn search_node(&mut self, state: Inner::State, node: Inner::Node) -> AltOption<Inner::Output> {
        match self.inner.process(node, state) {
            Left(states) => {
                for state in states {
                    self.search_nodes(state, self.inner.children(node))?;
                }
                AltNone
            }
            Right(output) => AltSome(output),
        }
    }
}
