use crate::*;
use husky_defn::DefnSheet;

pub(crate) struct TokenInferEngine<'a> {
    db: &'a dyn TokenInferDb,
    token_sheet: &'a TokenSheet,
    defn_sheet: &'a DefnSheet,
}

impl<'a> TokenInferEngine<'a> {
    pub(crate) fn new(db: &'a dyn TokenInferDb, module_path: ModulePath) -> VfsResult<Self> {
        Ok(Self {
            db,
            token_sheet: db.token_sheet(module_path)?,
            // if token sheet is ok, expr sheet must be ok
            defn_sheet: db.defn_sheet(module_path).unwrap(),
        })
    }

    pub(crate) fn infer_all(mut self) -> TokenInferSheet {
        todo!()
    }
}
