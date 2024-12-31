use super::*;
use alt_option::AltOption;
use rustc_hash::FxHashMap;
use std::marker::PhantomData;

pub struct GacDfsSearchEngine<'sess, Inner>
where
    Inner: IsGacSequentialEngineInner<'sess>,
{
    inner: Inner,
    _phantom: PhantomData<&'sess ()>,
}

impl<'sess, Inner: IsGacSequentialEngineInner<'sess>> GacDfsSearchEngine<'sess, Inner> {
    pub fn new(inner: Inner) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }
}

impl<'sess, Inner> GacDfsSearchEngine<'sess, Inner>
where
    Inner: IsGacSequentialEngineInner<'sess>,
{
    pub fn search(mut self, input: Inner::Input) -> Option<Inner::Output> {
        let root = self.inner.root();
        let state = self.inner.initial_state(input);
        self.search_aux(&state, root).into()
    }

    fn search_aux(&mut self, state: &Inner::State, node: Inner::Node) -> AltOption<Inner::Output> {
        match self.inner.process(node, state) {
            Left(states) => {
                for state in states {
                    for &child in self.inner.children(node) {
                        self.search_aux(&state, child)?;
                    }
                }
                AltOption::AltNone
            }
            Right(_) => todo!(),
        }
    }
}
