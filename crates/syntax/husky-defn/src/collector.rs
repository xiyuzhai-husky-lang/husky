use crate::*;
use husky_ast::{Ast, AstSheet};
use husky_entity_tree::{CratePrelude, EntityTreeResult};
use husky_token::TokenSheet;
use vec_like::VecPairMap;

pub(crate) struct DefnCollector<'a> {
    db: &'a dyn DefnDb,
    crate_prelude: CratePrelude<'a>,
    token_sheet: &'a TokenSheet,
    ast_sheet: &'a AstSheet,
    decl_sheet: &'a DeclSheet,
}

impl<'a> DefnCollector<'a> {
    pub(crate) fn new(db: &'a dyn DefnDb, module_path: ModulePath) -> EntityTreeResult<Self> {
        let crate_prelude = db.crate_prelude(module_path.crate_path(db))?;
        Ok(Self {
            db,
            crate_prelude,
            token_sheet: db.token_sheet(module_path)?,
            ast_sheet: db.ast_sheet(module_path)?,
            decl_sheet: db.decl_sheet(module_path)?,
        })
    }

    pub(crate) fn collect_all(mut self) -> DefnSheet {
        let mut defns: VecPairMap<EntityPath, Defn> = Default::default();
        for (entity_path, decl) in self.decl_sheet.decls().iter() {
            if let Ok(decl) = decl {
                defns
                    .insert_new((*entity_path, self.parse_defn(*decl)))
                    .unwrap()
            }
        }
        DefnSheet::new(defns)
    }

    fn parse_defn(&self, decl: Decl) -> Defn {
        match decl {
            Decl::Type(decl) => self.parse_ty_defn(decl).into(),
            Decl::Form(decl) => self.parse_form_defn(decl).into(),
            Decl::Trait(decl) => self.parse_trai_defn(decl).into(),
            Decl::TypeItem(_) => todo!(),
            Decl::TraitItem(_) => todo!(),
            Decl::Variant(_) => todo!(),
        }
    }

    fn parse_ty_defn(&self, decl: TypeDecl) -> TypeDefn {
        match decl {
            TypeDecl::Enum(decl) => self.parse_enum_ty_defn(decl).into(),
            TypeDecl::PropsStruct(decl) => self.parse_props_struct_ty_defn(decl).into(),
            TypeDecl::TupleStruct(decl) => self.parse_tuple_struct_ty_defn(decl).into(),
            TypeDecl::UnitStruct(decl) => self.parse_unit_struct_ty_defn(decl).into(),
            TypeDecl::Record(_) => todo!(),
            TypeDecl::Inductive(decl) => self.parse_inductive_ty_defn(decl).into(),
            TypeDecl::Structure(decl) => self.parse_structure_ty_defn(decl).into(),
            TypeDecl::Foreign(decl) => self.parse_alien_ty_defn(decl).into(),
            TypeDecl::Union(_) => todo!(),
        }
    }

    fn parse_trai_defn(&self, decl: TraitDecl) -> TraitDefn {
        let path = decl.path(self.db);
        let mut parser = self.expr_parser(path.into());
        TraitDefn::new(self.db, path, decl, parser.finish())
    }

    fn parse_enum_ty_defn(&self, decl: EnumTypeDecl) -> EnumTypeDefn {
        let path = decl.path(self.db);
        let mut parser = self.expr_parser(path.into());
        EnumTypeDefn::new(self.db, path, decl, parser.finish())
    }

    fn parse_props_struct_ty_defn(&self, decl: PropsStructTypeDecl) -> PropsStructTypeDefn {
        let path = decl.path(self.db);
        let mut parser = self.expr_parser(path.into());
        PropsStructTypeDefn::new(self.db, path, decl, parser.finish())
    }

    fn parse_tuple_struct_ty_defn(&self, decl: TupleStructTypeDecl) -> TupleStructTypeDefn {
        let path = decl.path(self.db);
        let mut parser = self.expr_parser(path.into());
        TupleStructTypeDefn::new(self.db, path, decl, parser.finish())
    }

    fn parse_unit_struct_ty_defn(&self, decl: UnitStructTypeDecl) -> UnitStructTypeDefn {
        let path = decl.path(self.db);
        let mut parser = self.expr_parser(path.into());
        UnitStructTypeDefn::new(self.db, path, decl, parser.finish())
    }

    fn parse_inductive_ty_defn(&self, decl: InductiveTypeDecl) -> InductiveTypeDefn {
        let path = decl.path(self.db);
        let mut parser = self.expr_parser(path.into());
        InductiveTypeDefn::new(self.db, path, decl, parser.finish())
    }

    fn parse_structure_ty_defn(&self, decl: StructureTypeDecl) -> StructureTypeDefn {
        let path = decl.path(self.db);
        let mut parser = self.expr_parser(path.into());
        StructureTypeDefn::new(self.db, path, decl, parser.finish())
    }

    fn parse_alien_ty_defn(&self, decl: AlienTypeDecl) -> AlienTypeDefn {
        let path = decl.path(self.db);
        let mut parser = self.expr_parser(path.into());
        AlienTypeDefn::new(self.db, path, decl, parser.finish())
    }

    fn parse_form_defn(&self, decl: FormDecl) -> FormDefn {
        match decl {
            FormDecl::Function(decl) => self.parse_function_defn(decl).into(),
            FormDecl::Feature(decl) => self.parse_feature_defn(decl).into(),
            FormDecl::Morphism(_) => todo!(),
            FormDecl::Value(_) => todo!(),
        }
    }

    fn parse_function_defn(&self, decl: FunctionDecl) -> FunctionDefn {
        let path = decl.path(self.db);
        let mut parser = self.expr_parser(path.into());
        let ast_idx = decl.ast_idx(self.db);
        let ast = &self.ast_sheet[ast_idx];
        let body = match ast {
            Ast::Defn { body, .. } => parser.parse_block(body).ok_or(DefnError::MissingBody),
            _ => unreachable!(),
        };
        FunctionDefn::new(self.db, path, decl, parser.finish(), body)
    }

    fn parse_feature_defn(&self, decl: FeatureDecl) -> FeatureDefn {
        let path = decl.path(self.db);
        let mut parser = self.expr_parser(path.into());
        let ast_idx = decl.ast_idx(self.db);
        let ast = &self.ast_sheet[ast_idx];
        let body = match ast {
            Ast::Defn { body, .. } => parser.parse_block(body).ok_or(DefnError::MissingBody),
            _ => unreachable!(),
        };
        FeatureDefn::new(self.db, path, decl, parser.finish(), body)
    }
    fn expr_parser(&self, entity_path: EntityPath) -> ExprParser<'a> {
        ExprParser::new(
            self.db,
            self.token_sheet,
            Some(self.ast_sheet),
            self.crate_prelude,
        )
    }
}
