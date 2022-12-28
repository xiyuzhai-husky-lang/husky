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
                Defn::Trait(defn) => self.infer_trai(defn),
                Defn::Form(defn) => self.infer_form(defn),
                Defn::TypeItem(_) => todo!(),
                Defn::TraitItem(_) => todo!(),
            }
        }
        self.sheet
    }

    fn infer_ty(&mut self, defn: TypeDefn) {
        match defn {
            TypeDefn::Enum(defn) => self.infer_enum_ty(defn),
            TypeDefn::Inductive(defn) => self.infer_inductive_ty(defn),
            TypeDefn::Record(defn) => self.infer_record_ty(defn),
            TypeDefn::Struct(defn) => self.infer_struct_ty(defn),
            TypeDefn::Structure(defn) => self.infer_structure_ty(defn),
            TypeDefn::Alias(defn) => self.infer_alias_ty(defn),
        }
    }

    fn infer_enum_ty(&mut self, defn: EnumTypeDefn) {
        // todo!()
    }

    fn infer_inductive_ty(&mut self, defn: InductiveTypeDefn) {
        // todo!()
    }

    fn infer_record_ty(&mut self, defn: RecordTypeDefn) {
        // todo!()
    }

    fn infer_struct_ty(&mut self, defn: StructTypeDefn) {
        // todo!()
    }

    fn infer_structure_ty(&mut self, defn: StructureTypeDefn) {
        // todo!()
    }

    fn infer_alias_ty(&mut self, defn: AliasTypeDefn) {
        // todo!()
    }

    fn infer_trai(&mut self, defn: TraitDefn) {
        //todo!()
    }

    fn infer_form(&mut self, defn: FormDefn) {
        // todo!()
    }
}
