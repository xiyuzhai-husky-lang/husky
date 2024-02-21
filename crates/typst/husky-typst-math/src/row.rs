use crate::TypstMathContext;
use crate::{
    fragment::{FrameFragment, MathFragment},
    TypstStyleChain,
};

#[derive(Debug, Default, Clone)]
pub struct MathRow(Vec<MathFragment>);

impl MathRow {
    pub fn new(fragments: Vec<MathFragment>) -> Self {
        todo!()
    }

    pub fn into_fragment(self, ctx: &TypstMathContext, styles: TypstStyleChain) -> MathFragment {
        if self.0.len() == 1 {
            self.0.into_iter().next().unwrap()
        } else {
            FrameFragment.into()
        }
    }
}
