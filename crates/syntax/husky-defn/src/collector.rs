use crate::*;
use husky_ast::AstSheet;
use husky_token::TokenSheet;
use vec_like::VecPairMap;

pub(crate) struct DefnCollector<'a> {
    db: &'a dyn DefnDb,
    token_sheet: &'a TokenSheet,
    ast_sheet: &'a AstSheet,
    decl_sheet: &'a DeclSheet,
}

impl<'a> DefnCollector<'a> {
    pub(crate) fn new(db: &'a dyn DefnDb, module_path: ModulePath) -> VfsResult<Self> {
        Ok(Self {
            db,
            token_sheet: db.token_sheet(module_path)?,
            ast_sheet: db.ast_sheet(module_path).unwrap(),
            decl_sheet: db.decl_sheet(module_path).unwrap(),
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
            Decl::Form(_) => todo!(),
            Decl::Trait(_) => todo!(),
        }
    }

    fn parse_ty_defn(&self, decl: TypeDecl) -> TypeDefn {
        match decl {
            TypeDecl::Enum(_) => todo!(),
            TypeDecl::Struct(_) => todo!(),
            TypeDecl::Record(_) => todo!(),
            TypeDecl::Inductive(decl) => self.parse_inductive_ty_defn(decl).into(),
            TypeDecl::Structure(decl) => self.parse_structure_ty_defn(decl).into(),
            TypeDecl::Alias(decl) => self.parse_alias_ty_defn(decl).into(),
        }
    }

    fn parse_inductive_ty_defn(&self, decl: InductiveTypeDecl) -> InductiveTypeDefn {
        let mut expr_sheet = ExprSheet::default();
        InductiveTypeDefn::new(self.db, decl.module_item_path(self.db), decl, expr_sheet)
    }

    fn parse_structure_ty_defn(&self, decl: StructureTypeDecl) -> StructureTypeDefn {
        let mut expr_sheet = ExprSheet::default();
        StructureTypeDefn::new(self.db, decl.module_item_path(self.db), decl, expr_sheet)
    }

    fn parse_alias_ty_defn(&self, decl: AliasTypeDecl) -> AliasTypeDefn {
        let mut expr_sheet = ExprSheet::default();
        AliasTypeDefn::new(self.db, decl.module_item_path(self.db), decl, expr_sheet)
    }
}
