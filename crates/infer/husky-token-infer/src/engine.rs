use crate::*;
use husky_defn::*;

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
        for defn in self.defn_sheet.defns() {
            match defn {
                Defn::Type(defn) => self.infer_ty(defn),
                Defn::Trait(_) => todo!(),
                Defn::Form(_) => todo!(),
                Defn::TypeItem(_) => todo!(),
                Defn::TraitItem(_) => todo!(),
            }
        }
        self.sheet
    }

    fn infer_ty(&mut self, defn: TypeDefn) {
        match defn {
            TypeDefn::Enum(_) => todo!(),
            TypeDefn::Inductive(_) => todo!(),
            TypeDefn::Record(_) => todo!(),
            TypeDefn::Struct(_) => todo!(),
            TypeDefn::Structure(defn) => self.infer_structure_ty(defn),
            TypeDefn::Alias(defn) => self.infer_alias_ty(defn),
        }
    }

    fn infer_structure_ty(&mut self, defn: StructureTypeDefn) {
        // todo!()
    }

    fn infer_alias_ty(&mut self, defn: AliasTypeDefn) {
        // todo!()
    }
}
