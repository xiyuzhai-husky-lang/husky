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
pub enum AssociatedItemDecl {
    TypeItem(TypeItemDecl),
    TraitItem(TraitItemDecl),
    TraitForTypeItem(TraitForTypeItemDecl),
}

impl AssociatedItemDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.ast_idx(db),
            AssociatedItemDecl::TraitItem(decl) => decl.ast_idx(db),
            AssociatedItemDecl::TraitForTypeItem(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDecl] {
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

    pub fn path(self, db: &dyn DeclDb) -> Option<AssociatedItemPath> {
        match self {
            AssociatedItemDecl::TypeItem(decl) => decl.path(db).map(|path| path.into()),
            AssociatedItemDecl::TraitItem(decl) => Some(decl.path(db).into()),
            AssociatedItemDecl::TraitForTypeItem(decl) => decl.path(db).map(|path| path.into()),
        }
    }
}

#[salsa::tracked(jar = DeclJar,return_ref)]
pub(crate) fn associated_item_decl(
    db: &dyn DeclDb,
    associated_item: AssociatedItem,
) -> DeclResult<AssociatedItemDecl> {
    let parser = DeclParseContext::new(db, associated_item.module_path(db))?;
    parser.parse_associated_item_decl(associated_item)
}

impl<'a> DeclParseContext<'a> {
    fn parse_associated_item_decl(
        &self,
        associated_item: AssociatedItem,
    ) -> DeclResult<AssociatedItemDecl> {
        let ast_idx = associated_item.ast_idx(self.db());
        Ok(match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                entity_kind:
                    EntityKind::AssociatedItem {
                        associated_item_kind,
                    },
                saved_stream_state,
                ..
            } => match associated_item_kind {
                AssociatedItemKind::TraitItem(_) => todo!(),
                AssociatedItemKind::TypeItem(ty_item_kind) => self.parse_ty_item_decl(
                    ty_item_kind,
                    ast_idx,
                    token_group_idx,
                    associated_item,
                    saved_stream_state,
                )?,
                AssociatedItemKind::TraitForTypeItem(trai_for_ty_item_kind) => {
                    AssociatedItemDecl::TraitForTypeItem(match trai_for_ty_item_kind {
                        TraitItemKind::MethodFn => self
                            .parse_trai_for_ty_method_decl(
                                ast_idx,
                                token_group_idx,
                                associated_item,
                                saved_stream_state,
                            )?
                            .into(),
                        TraitItemKind::AssociatedType => todo!(),
                    })
                }
            },
            _ => unreachable!(),
        })
    }
}

pub trait HasItemDecls {
    type ItemDecl;

    fn item_decls<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> EntityTreeBundleResultRef<'a, &'a [(Ident, Result<TypeItemDecl, ()>)]>;
}

impl HasDecl for AssociatedItem {
    type Decl = AssociatedItemDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        associated_item_decl(db, self).as_ref().copied()
    }
}
