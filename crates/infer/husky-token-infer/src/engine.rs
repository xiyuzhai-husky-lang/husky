use crate::*;

pub(crate) struct TokenInferEngine<'a> {
    db: &'a dyn TokenInferDb,
    token_sheet: &'a TokenSheet,
}

impl<'a> TokenInferEngine<'a> {
    pub(crate) fn new(db: &'a dyn TokenInferDb, module_path: ModulePath) -> VfsResult<Self> {
        Ok(Self {
            db,
            token_sheet: db.token_sheet(module_path)?,
        })
    }

    pub(crate) fn infer_all(mut self) -> TokenInferSheet {
        todo!()
    }
}
