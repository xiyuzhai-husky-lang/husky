use super::*;

#[floated(constructor = pub new)]
pub struct VdBsqNumChain<'sess> {
    pub leader: VdBsqNumTerm<'sess>,
    #[return_ref]
    pub followers: Vec<(VdMirFunc, VdBsqNumTerm<'sess>)>,
}

impl<'sess> From<VdBsqNumChain<'sess>> for VdBsqTerm<'sess> {
    fn from(chain: VdBsqNumChain<'sess>) -> Self {
        VdBsqTerm::Prop(chain.into())
    }
}
