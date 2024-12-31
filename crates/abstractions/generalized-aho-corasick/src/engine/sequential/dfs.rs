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
        let root = self.inner.root();
        let state = self.inner.initial_state(input);
        self.search_aux(&state, root).into()
    }

    fn search_aux(&mut self, state: &Inner::State, node: Inner::Node) -> AltOption<Inner::Output> {
        match self.inner.process(node, state) {
            Left(states) => {
                for state in states {
                    for child in self.inner.children(node) {
                        self.search_aux(&state, child)?;
                    }
                }
                AltNone
            }
            Right(output) => AltSome(output),
        }
    }
}
