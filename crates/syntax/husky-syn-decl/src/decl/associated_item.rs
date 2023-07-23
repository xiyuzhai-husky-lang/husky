mod ill_formed_item;
mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::ill_formed_item::*;
pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;
use husky_coword::Ident;
use husky_entity_taxonomy::{AssociatedItemKind, EntityKind, TraitItemKind, TypeItemKind};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum AssociatedItemSynNodeDecl {
    TypeItem(TypeItemSynNodeDecl),
    TraitItem(TraitItemSynNodeDecl),
    TraitForTypeItem(TraitForTypeItemSynNodeDecl),
    IllFormedItem(IllFormedItemSynNodeDecl),
}

impl AssociatedItemSynNodeDecl {
    pub fn syn_node_path(self, db: &dyn SynDeclDb) -> AssociatedItemSynNodePath {
        match self {
            AssociatedItemSynNodeDecl::TypeItem(syn_node_decl) => {
                syn_node_decl.syn_node_path(db).into()
            }
            AssociatedItemSynNodeDecl::TraitItem(syn_node_decl) => {
                syn_node_decl.syn_node_path(db).into()
            }
            AssociatedItemSynNodeDecl::TraitForTypeItem(syn_node_decl) => {
                syn_node_decl.syn_node_path(db).into()
            }
            AssociatedItemSynNodeDecl::IllFormedItem(syn_node_decl) => {
                syn_node_decl.syn_node_path(db).into()
            }
        }
    }

    pub fn ast_idx(self, db: &dyn SynDeclDb) -> AstIdx {
        match self {
            AssociatedItemSynNodeDecl::TypeItem(syn_node_decl) => syn_node_decl.ast_idx(db),
            AssociatedItemSynNodeDecl::TraitItem(syn_node_decl) => syn_node_decl.ast_idx(db),
            AssociatedItemSynNodeDecl::TraitForTypeItem(syn_node_decl) => syn_node_decl.ast_idx(db),
            AssociatedItemSynNodeDecl::IllFormedItem(_) => todo!(),
        }
    }

    pub fn generic_parameters<'a>(self, db: &'a dyn SynDeclDb) -> &'a [GenericParameterDecl] {
        match self {
            AssociatedItemSynNodeDecl::TypeItem(syn_node_decl) => {
                syn_node_decl.generic_parameters(db)
            }
            AssociatedItemSynNodeDecl::TraitItem(syn_node_decl) => {
                syn_node_decl.generic_parameters(db)
            }
            AssociatedItemSynNodeDecl::TraitForTypeItem(_) => todo!(),
            AssociatedItemSynNodeDecl::IllFormedItem(_) => todo!(),
        }
    }

    pub fn expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            AssociatedItemSynNodeDecl::TypeItem(syn_node_decl) => syn_node_decl.expr_region(db),
            AssociatedItemSynNodeDecl::TraitItem(syn_node_decl) => syn_node_decl.expr_region(db),
            AssociatedItemSynNodeDecl::TraitForTypeItem(syn_node_decl) => {
                syn_node_decl.expr_region(db)
            }
            AssociatedItemSynNodeDecl::IllFormedItem(_) => todo!(),
        }
    }

    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
        match self {
            AssociatedItemSynNodeDecl::TypeItem(syn_node_decl) => syn_node_decl.errors(db),
            AssociatedItemSynNodeDecl::TraitItem(syn_node_decl) => syn_node_decl.errors(db),
            AssociatedItemSynNodeDecl::TraitForTypeItem(syn_node_decl) => syn_node_decl.errors(db),
            AssociatedItemSynNodeDecl::IllFormedItem(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum AssociatedItemSynDecl {
    TypeItem(TypeItemSynDecl),
    TraitItem(TraitItemSynDecl),
    TraitForTypeItem(TraitForTypeItemSynDecl),
}

impl AssociatedItemSynDecl {
    pub fn path(self, db: &dyn SynDeclDb) -> AssociatedItemPath {
        match self {
            AssociatedItemSynDecl::TypeItem(decl) => decl.path(db).into(),
            AssociatedItemSynDecl::TraitItem(decl) => decl.path(db).into(),
            AssociatedItemSynDecl::TraitForTypeItem(decl) => decl.path(db).into(),
        }
    }

    pub fn generic_parameters<'a>(self, db: &'a dyn SynDeclDb) -> &'a [GenericParameterDecl] {
        match self {
            AssociatedItemSynDecl::TypeItem(decl) => decl.generic_parameters(db),
            AssociatedItemSynDecl::TraitItem(decl) => decl.generic_parameters(db),
            AssociatedItemSynDecl::TraitForTypeItem(_) => todo!(),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            AssociatedItemSynDecl::TypeItem(decl) => decl.expr_region(db),
            AssociatedItemSynDecl::TraitItem(decl) => decl.expr_region(db),
            AssociatedItemSynDecl::TraitForTypeItem(decl) => decl.expr_region(db),
        }
    }
}

impl HasSynDecl for AssociatedItemPath {
    type Decl = AssociatedItemSynDecl;

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        todo!()
        // associated_item_syn_decl(db, self).as_ref().copied()
    }
}

// #[salsa::tracked(jar = SynDeclJar, return_ref)]
// pub(crate) fn associated_item_syn_decl(
//     db: &dyn SynDeclDb,
//     node: AssociatedItemNode,
// ) -> DeclResult<AssociatedItemDecl> {
//     let parser = DeclParseContext::new(db, node.module_path(db))?;
//     parser.parse_associated_item_syn_decl(node)
// }

impl<'a> DeclParser<'a> {
    // fn parse_associated_item_syn_decl(
    //     &self,
    //     node: AssociatedItemNode,
    // ) -> DeclResult<AssociatedItemDecl> {
    //     let ast_idx = node.ast_idx(self.db());
    //     Ok(match self.ast_sheet()[ast_idx] {
    //         Ast::Defn {
    //             token_group_idx,
    //             entity_kind:
    //                 EntityKind::AssociatedItem {
    //                     associated_item_kind,
    //                 },
    //             saved_stream_state,
    //             ..
    //         } => match associated_item_kind {
    //             AssociatedItemKind::TraitItem(_) => todo!(),
    //             AssociatedItemKind::TypeItem(ty_item_kind) => self
    //                 .parse_ty_item_syn_decl(
    //                     ty_item_kind,
    //                     ast_idx,
    //                     token_group_idx,
    //                     node,
    //                     saved_stream_state,
    //                 )?
    //                 .into(),
    //             AssociatedItemKind::TraitForTypeItem(trai_item_kind) => self
    //                 .parse_trai_for_ty_item_syn_decl(
    //                     trai_item_kind,
    //                     ast_idx,
    //                     token_group_idx,
    //                     node,
    //                     saved_stream_state,
    //                 )?
    //                 .into(),
    //         },
    //         _ => unreachable!(),
    //     })
    // }
}

pub trait HasItemDeclsMap {
    type ItemDecls;

    fn item_syn_decls_map<'a>(
        self,
        db: &'a dyn SynDeclDb,
    ) -> EntityTreeBundleResultRef<'a, &'a [(Ident, Result<Self::ItemDecls, ()>)]>;
}

pub trait HasItemDecls {
    type ItemDecls;

    fn item_syn_decls<'a>(self, db: &'a dyn SynDeclDb) -> DeclResult<&'a Self::ItemDecls>;
}
