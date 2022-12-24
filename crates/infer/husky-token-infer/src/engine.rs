use crate::*;
use husky_defn::DefnSheet;

pub(crate) struct TokenInferEngine<'a> {
    db: &'a dyn TokenInferDb,
    token_sheet: &'a TokenSheet,
    defn_sheet: &'a DefnSheet,
    sheet: TokenInferSheet,
}

impl<'a> TokenInferEngine<'a> {
    pub(crate) fn new(db: &'a dyn TokenInferDb, module_path: ModulePath) -> VfsResult<Self> {
        let token_sheet = &db.token_sheet(module_path)?;
        Ok(Self {
            db,
            token_sheet,
            // if token sheet is ok, expr sheet must be ok
            defn_sheet: db.defn_sheet(module_path).unwrap(),
            sheet: TokenInferSheet::new(token_sheet),
        })
    }

    pub(crate) fn infer_all(mut self) -> TokenInferSheet {
        for defn in self.defn_sheet.defns().iter() {
            todo!()
        }
        self.sheet
    }
}
