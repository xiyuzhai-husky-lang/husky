use std::marker::PhantomData;

#[derive(Debug, thiserror::Error)]
pub enum VdBaseqElaborationError<'sess> {
    #[error("phantom data")]
    PhantomData(PhantomData<&'sess ()>),
}

pub type VdBaseqElaborationResult<'sess, T> = Result<T, VdBaseqElaborationError<'sess>>;
