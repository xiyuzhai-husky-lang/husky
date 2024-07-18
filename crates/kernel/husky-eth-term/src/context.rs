use crate::EthTerm;

#[salsa::interned]
pub struct EthTermContextItd {
    pub task_ty: Option<EthTerm>,
}

impl EthTermContextItd {
    #[deprecated(note = "we should probably use a better notion")]
    pub fn new_generic(db: &::salsa::Db) -> Self {
        let task_ty = None;
        Self::new(db, task_ty)
    }
}
