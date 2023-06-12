mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;
use husky_entity_taxonomy::{AssociatedItemKind, EntityKind, TraitItemKind, TypeItemKind};
use husky_word::Ident;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum AssociatedItemRawDecl {
    TypeItem(TypeItemRawDecl),
    TraitItem(TraitItemRawDecl),
    TraitForTypeItem(TraitForTypeItemRawDecl),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum AssociatedItemDecl {
    TypeItem(TypeItemDecl),
    TraitItem(TraitItemDecl),
    TraitForTypeItem(TraitForTypeItemDecl),
}

impl AssociatedItemDecl {
    pub fn node_path(self, db: &dyn DeclDb) -> AssociatedItemNodePath {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.node_path(db).into(),
            AssociatedItemDecl::TraitItem(decl) => decl.node_path(db).into(),
            AssociatedItemDecl::TraitForTypeItem(decl) => decl.node_path(db).into(),
        }
    }

    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.ast_idx(db),
            AssociatedItemDecl::TraitItem(decl) => decl.ast_idx(db),
            AssociatedItemDecl::TraitForTypeItem(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.implicit_parameters(db),
            AssociatedItemDecl::TraitItem(decl) => decl.implicit_parameters(db),
            AssociatedItemDecl::TraitForTypeItem(_) => todo!(),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.expr_region(db),
            AssociatedItemDecl::TraitItem(decl) => decl.expr_region(db),
            AssociatedItemDecl::TraitForTypeItem(decl) => decl.expr_region(db),
        }
    }
}

// #[salsa::tracked(jar = DeclJar, return_ref)]
// pub(crate) fn associated_item_decl(
//     db: &dyn DeclDb,
//     node: AssociatedItemNode,
// ) -> DeclResult<AssociatedItemDecl> {
//     let parser = DeclParseContext::new(db, node.module_path(db))?;
//     parser.parse_associated_item_decl(node)
// }

impl<'a> DeclParseContext<'a> {
    // fn parse_associated_item_decl(
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
    //                 .parse_ty_item_decl(
    //                     ty_item_kind,
    //                     ast_idx,
    //                     token_group_idx,
    //                     node,
    //                     saved_stream_state,
    //                 )?
    //                 .into(),
    //             AssociatedItemKind::TraitForTypeItem(trai_item_kind) => self
    //                 .parse_trai_for_ty_item_decl(
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

    fn item_decls_map<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> EntityTreeBundleResultRef<'a, &'a [(Ident, Result<Self::ItemDecls, ()>)]>;
}

pub trait HasItemDecls {
    type ItemDecls;

    fn item_decls<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, &'a Self::ItemDecls>;
}

impl HasDecl for AssociatedItemNode {
    type Decl = AssociatedItemDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        todo!()
        // associated_item_decl(db, self).as_ref().copied()
    }
}
