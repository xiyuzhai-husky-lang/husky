use crate::*;
use husky_ast::{Ast, AstIdx, AstIdxRange, AstSheet};
use husky_entity_path::EntityPath;
use husky_entity_taxonomy::{EntityKind, FormKind, ModuleItemKind, TypeKind};
use husky_entity_tree::{CratePrelude, EntitySymbol, EntityTreeSheet};
use husky_expr::{parse_expr, ExprArena, ExprParsingStopReason};
use husky_opn_syntax::BinaryOpr;
use husky_print_utils::p;
use husky_symbol::{LocalSymbolSheet, SymbolContext};
use husky_token::{SpecialToken, TokenGroupIdx, TokenIdentifier, TokenIterState, TokenSheet};
use parsec::ParseFrom;
use vec_like::VecPairMap;

pub(crate) struct DeclCollector<'a> {
    db: &'a dyn DeclDb,
    crate_prelude: CratePrelude<'a>,
    token_sheet: &'a TokenSheet,
    ast_sheet: &'a AstSheet,
    entity_tree_sheet: &'a EntityTreeSheet,
}

impl<'a> DeclCollector<'a> {
    pub(crate) fn new(db: &'a dyn DeclDb, module_path: ModulePath) -> EntityTreeResult<Self> {
        let crate_prelude = db.crate_prelude(module_path.crate_path(db))?;
        Ok(Self {
            db,
            crate_prelude,
            token_sheet: db.token_sheet(module_path)?,
            ast_sheet: db.ast_sheet(module_path)?,
            entity_tree_sheet: db.entity_tree_sheet(module_path)?,
        })
    }

    pub(crate) fn collect_all(mut self) -> DeclSheet {
        let mut decls: VecPairMap<EntityPath, DeclResult<Decl>> = Default::default();
        for entity_symbol in self.entity_tree_sheet.module_symbols().iter() {
            match entity_symbol {
                EntitySymbol::CrateRoot { .. } => unreachable!(),
                EntitySymbol::Submodule { .. } | EntitySymbol::EntityUse { .. } => (),
                EntitySymbol::ModuleItem {
                    ident,
                    accessibility,
                    path,
                    ast_idx,
                } => decls.insert(((*path).into(), self.parse_decl((*path).into(), *ast_idx))),
            }
        }
        for associated_item in self.entity_tree_sheet.associated_items().iter() {
            todo!()
        }
        DeclSheet::new(decls)
    }

    fn parse_decl(&mut self, entity_path: EntityPath, ast_idx: AstIdx) -> DeclResult<Decl> {
        match self.ast_sheet[ast_idx] {
            Ast::Defn {
                token_group_idx,
                ref body,
                accessibility,
                entity_kind,
                entity_path: _,
                ident,
                is_generic,
                body_kind,
                saved_stream_state,
            } => match entity_kind {
                EntityKind::ModuleItem {
                    module_item_kind, ..
                } => match module_item_kind {
                    ModuleItemKind::Type(type_kind) => self.parse_ty_decl(
                        type_kind,
                        entity_path.module_item_path().unwrap(),
                        entity_kind,
                        token_group_idx,
                        body,
                        saved_stream_state,
                    ),
                    ModuleItemKind::Form(form_kind) => self.parse_form_decl(
                        form_kind,
                        entity_path.module_item_path().unwrap(),
                        entity_kind,
                        token_group_idx,
                        body,
                        saved_stream_state,
                    ),
                    ModuleItemKind::Trait => {
                        self.parse_trai_decl(entity_path.module_item_path().unwrap())
                    }
                },
                EntityKind::Module | EntityKind::AssociatedItem { .. } | EntityKind::Variant => {
                    unreachable!()
                }
            },
            Ast::Impl { .. }
            | Ast::Err { .. }
            | Ast::Use { .. }
            | Ast::Comment { .. }
            | Ast::Decor { .. }
            | Ast::Stmt { .. }
            | Ast::IfElseStmts { .. }
            | Ast::MatchStmts { .. }
            | Ast::ModuleItemVariant { .. }
            | Ast::Main { .. }
            | Ast::Config { .. } => unreachable!(),
        }
    }

    fn parse_ty_decl(
        &mut self,
        type_kind: TypeKind,
        module_item_path: ModuleItemPath,
        entity_kind: EntityKind,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIterState,
    ) -> Result<Decl, DeclError> {
        match type_kind {
            TypeKind::Enum => self.parse_enum_type_decl(module_item_path),
            TypeKind::Inductive => self.parse_inductive_type_decl(module_item_path),
            TypeKind::Record => todo!(),
            TypeKind::Struct => self.parse_struct_type_decl(module_item_path),
            TypeKind::Structure => self.parse_structure_type_decl(module_item_path),
            TypeKind::Foreign => self.parse_foreign_type_decl(
                entity_kind,
                module_item_path,
                token_group_idx,
                body,
                saved_stream_state,
            ),
        }
    }

    fn parse_enum_type_decl(&self, module_item_path: ModuleItemPath) -> DeclResult<Decl> {
        Ok(Decl::Type(
            EnumTypeDecl::new(self.db, module_item_path).into(),
        ))
    }

    fn parse_trai_decl(&self, module_item_path: ModuleItemPath) -> DeclResult<Decl> {
        Ok(Decl::Trait(TraitDecl::new(self.db, module_item_path)))
    }

    fn parse_inductive_type_decl(&self, module_item_path: ModuleItemPath) -> DeclResult<Decl> {
        Ok(Decl::Type(
            InductiveTypeDecl::new(self.db, module_item_path).into(),
        ))
    }

    fn parse_struct_type_decl(&self, module_item_path: ModuleItemPath) -> DeclResult<Decl> {
        Ok(Decl::Type(
            StructTypeDecl::new(self.db, module_item_path, /* ad hoc */ vec![]).into(),
        ))
    }

    fn parse_structure_type_decl(&self, module_item_path: ModuleItemPath) -> DeclResult<Decl> {
        Ok(Decl::Type(
            StructureTypeDecl::new(self.db, module_item_path).into(),
        ))
    }

    // get declaration from tokens
    fn parse_foreign_type_decl(
        &self,
        entity_kind: EntityKind,
        module_item_path: ModuleItemPath,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIterState,
    ) -> DeclResult<Decl> {
        let mut token_iter = self
            .token_sheet
            .token_group_token_iter(token_group_idx, Some(saved_stream_state));
        let mut expr_arena = ExprArena::default();
        let local_symbol_sheet = LocalSymbolSheet::default();
        // if let Some(_) = token_iter.try_eat_special(BinaryOpr::Assign(None).into(), true) {
        //     todo!()
        // } else {
        //     match token_iter.try_eat_special(SpecialToken::Semicolon, true) {
        //         Some(_) => {
        //             if !token_iter.is_empty() {
        //                 todo!()
        //             }
        //             todo!()
        //         }
        //         None => todo!(),
        //     }
        // }
        Ok(Decl::Type(
            AliasTypeDecl::new(self.db, module_item_path).into(),
        ))
    }

    fn parse_form_decl(
        &mut self,
        form_kind: FormKind,
        module_item_path: ModuleItemPath,
        entity_kind: EntityKind,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIterState,
    ) -> Result<Decl, DeclError> {
        match form_kind {
            FormKind::Feature => self.parse_feature_decl(module_item_path),
            FormKind::Function => self.parse_function_decl(module_item_path),
            FormKind::Value => todo!(),
            FormKind::TypeAlias => todo!(),
        }
    }

    fn parse_feature_decl(&self, module_item_path: ModuleItemPath) -> Result<Decl, DeclError> {
        Ok(Decl::Form(
            FeatureDecl::new(self.db, module_item_path).into(),
        ))
    }

    fn parse_function_decl(&self, module_item_path: ModuleItemPath) -> Result<Decl, DeclError> {
        Ok(Decl::Form(
            FunctionDecl::new(self.db, module_item_path).into(),
        ))
    }

    fn ctx<'b>(
        &'b self,
        entity_path: EntityPath,
        local_symbol_sheet: &'a LocalSymbolSheet,
    ) -> SymbolContext<'b> {
        SymbolContext::new(self.db, entity_path, self.crate_prelude, local_symbol_sheet)
    }
}
