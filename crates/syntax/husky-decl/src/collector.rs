use crate::*;
use husky_ast::{Ast, AstIdx, AstIdxRange, AstSheet};
use husky_entity_path::EntityPath;
use husky_entity_taxonomy::{EntityKind, ItemKind, TypeKind};
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
    pub(crate) fn new(db: &'a dyn DeclDb, module_path: ModulePath) -> DeclResult<Self> {
        let crate_prelude = db.crate_prelude(module_path.crate_path(db))?;
        Ok(Self {
            db,
            crate_prelude,
            token_sheet: db.token_sheet(module_path)?,
            ast_sheet: db.ast_sheet(module_path).unwrap(),
            entity_tree_sheet: db.entity_tree_sheet(module_path).unwrap(),
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
                    item_kind: module_item_kind,
                    ..
                } => match module_item_kind {
                    ItemKind::Type(type_kind) => match type_kind {
                        TypeKind::Enum => todo!(),
                        TypeKind::Inductive => todo!(),
                        TypeKind::Record => todo!(),
                        TypeKind::Struct => todo!(),
                        TypeKind::Structure => todo!(),
                        TypeKind::Form => self.parse_form_decl(
                            entity_kind,
                            entity_path,
                            token_group_idx,
                            body,
                            saved_stream_state,
                        ),
                    },
                    ItemKind::Trait => todo!(),
                    ItemKind::Form => todo!(),
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

    // get declaration from tokens
    fn parse_form_decl(
        &self,
        entity_kind: EntityKind,
        entity_path: EntityPath,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIterState,
    ) -> DeclResult<Decl> {
        let mut token_iter = self
            .token_sheet
            .token_group_token_iter(token_group_idx, Some(saved_stream_state));
        let mut expr_arena = ExprArena::default();
        let local_symbol_sheet = LocalSymbolSheet::default();
        if let Some(_) = token_iter.try_eat_special(BinaryOpr::Assign(None).into(), true) {
            let (expr, stop_reason) = parse_expr(
                self.ctx(entity_path, &local_symbol_sheet),
                &mut token_iter,
                &mut expr_arena,
            );
            todo!()
        } else {
            match token_iter.try_eat_special(SpecialToken::Semicolon, true) {
                Some(_) => {
                    if !token_iter.is_empty() {
                        todo!()
                    }
                    todo!()
                }
                None => todo!(),
            }
        }
    }

    fn ctx<'b>(
        &'b self,
        entity_path: EntityPath,
        local_symbol_sheet: &'a LocalSymbolSheet,
    ) -> SymbolContext<'b> {
        SymbolContext::new(self.db, entity_path, self.crate_prelude, local_symbol_sheet)
    }
}
