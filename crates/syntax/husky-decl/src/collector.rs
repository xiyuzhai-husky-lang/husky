use crate::*;
use husky_ast::{Ast, AstIdx, AstIdxRange, AstSheet};
use husky_entity_path::EntityPath;
use husky_entity_taxonomy::{EntityKind, ModuleItemKind, TypeKind};
use husky_entity_tree::{EntitySymbol, EntityTreeSheet};
use husky_print_utils::p;
use husky_token::{TokenIdentifier, TokenSheet};
use parsec::ParseFrom;
use vec_like::VecPairMap;

pub(crate) struct DeclCollector<'a> {
    db: &'a dyn DeclDb,
    token_sheet: &'a TokenSheet,
    ast_sheet: &'a AstSheet,
    entity_tree_sheet: &'a EntityTreeSheet,
}

impl<'a> DeclCollector<'a> {
    pub(crate) fn new(db: &'a dyn DeclDb, module_path: ModulePath) -> VfsResult<Self> {
        Ok(Self {
            db,
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
                } => decls.insert(((*path).into(), self.parse_decl(*ast_idx))),
            }
        }
        for associated_item in self.entity_tree_sheet.associated_items().iter() {
            todo!()
        }
        DeclSheet::new(decls)
    }

    fn parse_decl(&mut self, ast_idx: AstIdx) -> DeclResult<Decl> {
        match self.ast_sheet[ast_idx] {
            Ast::Defn {
                token_group_idx,
                ref body,
                accessibility,
                entity_kind,
                entity_path,
                ident,
                is_generic,
                body_kind,
            } => match entity_kind {
                EntityKind::ModuleItem(module_item_kind) => match module_item_kind {
                    ModuleItemKind::Type(type_kind) => match type_kind {
                        TypeKind::Enum => todo!(),
                        TypeKind::Inductive => todo!(),
                        TypeKind::Record => todo!(),
                        TypeKind::Struct => todo!(),
                        TypeKind::Structure => todo!(),
                        TypeKind::Form => self.parse_form_decl(token_group_idx, body),
                    },
                    ModuleItemKind::Trait => todo!(),
                    ModuleItemKind::Form => todo!(),
                },
                EntityKind::Module | EntityKind::AssociatedItem | EntityKind::EnumVariant => {
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

    fn parse_form_decl(
        &self,
        token_group_idx: husky_token::TokenGroupIdx,
        body: &AstIdxRange,
    ) -> DeclResult<Decl> {
        let mut token_iter = self.token_sheet.token_group_token_iter(token_group_idx);
        let a = TokenIdentifier::parse_from(&mut token_iter)?;
        p!(a);
        todo!()
    }
}
