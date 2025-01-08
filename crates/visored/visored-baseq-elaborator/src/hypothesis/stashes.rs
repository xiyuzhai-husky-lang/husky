use std::marker::PhantomData;

pub struct VdBsqHypothesisCaches<'sess> {
    phantom: PhantomData<&'sess ()>,
}
