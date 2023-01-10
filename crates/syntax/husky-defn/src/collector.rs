use crate::*;
use husky_ast::{Ast, AstSheet, AstTokenIdxRangeSheet};
use husky_entity_tree::{CratePrelude, EntityTreeResult, ModuleSymbolContext};
use husky_token::{RangedTokenSheet, TokenSheetData};
use vec_like::VecPairMap;

pub(crate) struct DefnCollector<'a> {
    db: &'a dyn DefnDb,
    module_symbol_context: ModuleSymbolContext<'a>,
    token_sheet_data: &'a TokenSheetData,
    ast_sheet: &'a AstSheet,
    ast_range_sheet: &'a AstTokenIdxRangeSheet,
    decl_sheet: &'a DeclSheet,
}

impl<'a> DefnCollector<'a> {
    pub(crate) fn new(db: &'a dyn DefnDb, module_path: ModulePath) -> EntityTreeResult<Self> {
        let module_symbol_context = db.module_symbol_context(module_path)?;
        Ok(Self {
            db,
            module_symbol_context,
            token_sheet_data: db.token_sheet_data(module_path)?,
            ast_sheet: db.ast_sheet(module_path)?,
            ast_range_sheet: db.ast_range_sheet(module_path)?,
            decl_sheet: db.module_decl_sheet(module_path)?,
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
            Decl::ImplBlock(_) => todo!(),
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
        TraitDefn::new(self.db, path, decl)
    }

    fn parse_enum_ty_defn(&self, decl: EnumTypeDecl) -> EnumTypeDefn {
        let path = decl.path(self.db);
        EnumTypeDefn::new(self.db, path, decl)
    }

    fn parse_props_struct_ty_defn(&self, decl: PropsStructTypeDecl) -> PropsStructTypeDefn {
        let path = decl.path(self.db);
        PropsStructTypeDefn::new(self.db, path, decl)
    }

    fn parse_tuple_struct_ty_defn(&self, decl: TupleStructTypeDecl) -> TupleStructTypeDefn {
        let path = decl.path(self.db);
        TupleStructTypeDefn::new(self.db, path, decl)
    }

    fn parse_unit_struct_ty_defn(&self, decl: UnitStructTypeDecl) -> UnitStructTypeDefn {
        let path = decl.path(self.db);
        UnitStructTypeDefn::new(self.db, path, decl)
    }

    fn parse_inductive_ty_defn(&self, decl: InductiveTypeDecl) -> InductiveTypeDefn {
        let path = decl.path(self.db);
        InductiveTypeDefn::new(self.db, path, decl)
    }

    fn parse_structure_ty_defn(&self, decl: StructureTypeDecl) -> StructureTypeDefn {
        let path = decl.path(self.db);
        StructureTypeDefn::new(self.db, path, decl)
    }

    fn parse_alien_ty_defn(&self, decl: AlienTypeDecl) -> AlienTypeDefn {
        let path = decl.path(self.db);
        AlienTypeDefn::new(self.db, path, decl)
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
        let mut parser = self.module_item_block_expr_parser(
            path.into(),
            Some(decl.expr_sheet(self.db).symbol_sheet(self.db)),
        );
        let ast_idx = decl.ast_idx(self.db);
        let ast = &self.ast_sheet[ast_idx];
        let body = match ast {
            Ast::Defn { body, .. } => parser.parse_block_expr(*body).ok_or(DefnError::MissingBody),
            _ => unreachable!(),
        };
        let expr_sheet = parser.finish();
        FunctionDefn::new(self.db, path, decl, expr_sheet, body)
    }

    fn parse_feature_defn(&self, decl: FeatureDecl) -> FeatureDefn {
        let path = decl.path(self.db);
        let mut parser = self.module_item_block_expr_parser(path.into(), None);
        let ast_idx = decl.ast_idx(self.db);
        let ast = &self.ast_sheet[ast_idx];
        let body = match ast {
            Ast::Defn { body, .. } => parser.parse_block_expr(*body).ok_or(DefnError::MissingBody),
            _ => unreachable!(),
        };
        let expr_sheet = parser.finish();
        FeatureDefn::new(self.db, path, decl, expr_sheet, body)
    }
    fn module_item_block_expr_parser(
        &self,
        entity_path: EntityPath,
        decl_symbol_sheet: Option<&SymbolSheet>,
    ) -> BlockExprParser<'a> {
        let parser = ExprParser::new(
            self.db,
            Some(entity_path),
            self.token_sheet_data,
            SymbolContextMut::new(self.module_symbol_context, decl_symbol_sheet),
        );
        BlockExprParser::new(parser, self.ast_sheet, self.ast_range_sheet)
    }
}
