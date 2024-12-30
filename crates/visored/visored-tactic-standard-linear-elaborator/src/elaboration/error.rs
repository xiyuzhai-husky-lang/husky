use std::marker::PhantomData;

#[derive(Debug, thiserror::Error)]
pub enum VdMirTacticStandardLinearElaborationError<'sess> {
    #[error("phantom data")]
    PhantomData(PhantomData<&'sess ()>),
}

pub type VdMirTacticStandardLinearElaborationResult<'sess, T> =
    Result<T, VdMirTacticStandardLinearElaborationError<'sess>>;
