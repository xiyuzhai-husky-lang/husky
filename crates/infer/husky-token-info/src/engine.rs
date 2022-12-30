use crate::*;
use husky_ast::{Ast, AstSheet};
use husky_defn::*;

pub(crate) struct TokenInferEngine<'a> {
    db: &'a dyn TokenInfoDb,
    token_sheet: &'a TokenSheet,
    ast_sheet: &'a AstSheet,
    defn_sheet: &'a DefnSheet,
    sheet: TokenInfoSheet,
}

impl<'a> TokenInferEngine<'a> {
    pub(crate) fn new(db: &'a dyn TokenInfoDb, module_path: ModulePath) -> EntityTreeResult<Self> {
        let token_sheet = &db.token_sheet(module_path)?;
        Ok(Self {
            db,
            token_sheet,
            defn_sheet: db.defn_sheet(module_path)?,
            ast_sheet: db.ast_sheet(module_path)?,
            sheet: TokenInfoSheet::new(token_sheet),
        })
    }

    pub(crate) fn infer_all(mut self) -> TokenInfoSheet {
        for defn in self.defn_sheet.defns() {
            let ast_idx = defn.ast_idx(self.db);
            match self.ast_sheet[ast_idx] {
                Ast::Defn {
                    token_group_idx,
                    ref body,
                    accessibility,
                    entity_kind,
                    entity_path,
                    ident_token,
                    is_generic,
                    body_kind,
                    saved_stream_state,
                } => self
                    .sheet
                    .add(ident_token.token_idx(), TokenInfo::Entity(entity_kind)),
                _ => unreachable!(),
            }
            match defn {
                Defn::Type(defn) => self.infer_ty(defn),
                Defn::Trait(defn) => self.infer_trai(defn),
                Defn::Form(defn) => self.infer_form(defn),
                Defn::TypeItem(_) => todo!(),
                Defn::TraitItem(_) => todo!(),
                Defn::Variant(_) => todo!(),
            }
        }
        self.sheet
    }

    fn infer_ty(&mut self, defn: TypeDefn) {
        match defn {
            TypeDefn::Enum(defn) => self.infer_enum_ty(defn),
            TypeDefn::Inductive(defn) => self.infer_inductive_ty(defn),
            TypeDefn::Record(defn) => self.infer_record_ty(defn),
            TypeDefn::UnitStruct(defn) => self.infer_unit_struct_ty(defn),
            TypeDefn::TupleStruct(defn) => self.infer_tuple_struct_ty(defn),
            TypeDefn::PropsStruct(defn) => self.infer_props_struct_ty(defn),
            TypeDefn::Structure(defn) => self.infer_structure_ty(defn),
            TypeDefn::Foreign(defn) => self.infer_alias_ty(defn),
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

    fn infer_unit_struct_ty(&mut self, defn: UnitStructTypeDefn) {
        // todo!()
    }

    fn infer_tuple_struct_ty(&mut self, defn: TupleStructTypeDefn) {
        // todo!()
    }

    fn infer_props_struct_ty(&mut self, defn: PropsStructTypeDefn) {
        // todo!()
    }

    fn infer_structure_ty(&mut self, defn: StructureTypeDefn) {
        // todo!()
    }

    fn infer_alias_ty(&mut self, defn: AlienTypeDefn) {
        // todo!()
    }

    fn infer_trai(&mut self, defn: TraitDefn) {
        //todo!()
    }

    fn infer_form(&mut self, defn: FormDefn) {
        // todo!()
    }
}
