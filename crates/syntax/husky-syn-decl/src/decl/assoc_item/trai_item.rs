mod assoc_fn;
mod assoc_ty;
mod assoc_val;
mod memo_field;
mod method_fn;

use husky_entity_kind::TraitItemKind;

pub use self::assoc_fn::*;
pub use self::assoc_ty::*;
pub use self::assoc_val::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TraitItemSynNodeDecl {
    AssocFn(TraitAssocFnSynNodeDecl),
    MethodFn(TraitMethodFnSynNodeDecl),
    AssocType(TraitAssocTypeSynNodeDecl),
    AssocVal(TraitAssocValSynNodeDecl),
}

impl TraitItemSynNodeDecl {
    pub fn syn_node_path(self, db: &::salsa::Db) -> TraitItemSynNodePath {
        match self {
            TraitItemSynNodeDecl::AssocFn(decl) => decl.syn_node_path(db),
            TraitItemSynNodeDecl::MethodFn(decl) => decl.syn_node_path(db),
            TraitItemSynNodeDecl::AssocType(decl) => decl.syn_node_path(db),
            TraitItemSynNodeDecl::AssocVal(decl) => decl.syn_node_path(db),
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            TraitItemSynNodeDecl::AssocFn(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TraitItemSynNodeDecl::MethodFn(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TraitItemSynNodeDecl::AssocType(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TraitItemSynNodeDecl::AssocVal(syn_node_decl) => syn_node_decl.syn_expr_region(db),
        }
    }

    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        match self {
            TraitItemSynNodeDecl::AssocFn(slf) => slf.errors(db),
            TraitItemSynNodeDecl::MethodFn(slf) => slf.errors(db),
            TraitItemSynNodeDecl::AssocType(slf) => slf.errors(db),
            TraitItemSynNodeDecl::AssocVal(slf) => slf.errors(db),
        }
    }
}

impl HasSynNodeDecl for TraitItemSynNodePath {
    type NodeDecl = TraitItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        trai_item_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
fn trai_item_syn_node_decl(
    db: &::salsa::Db,
    syn_node_path: TraitItemSynNodePath,
) -> TraitItemSynNodeDecl {
    let parser = DeclParser::new(db, syn_node_path.into());
    parser.parse_trai_item_syn_node_decl(syn_node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_trai_item_syn_node_decl(
        &self,
        syn_node_path: TraitItemSynNodePath,
    ) -> TraitItemSynNodeDecl {
        match syn_node_path.item_kind(self.db()) {
            TraitItemKind::MemoizedField => todo!(),
            TraitItemKind::MethodRitchie(_) => {
                self.parse_trai_method_fn_node_decl(syn_node_path).into()
            }
            TraitItemKind::AssocType => self.parse_trai_assoc_ty_node_decl(syn_node_path).into(),
            TraitItemKind::AssocVal => self.parse_trai_assoc_val_node_decl(syn_node_path).into(),
            TraitItemKind::AssocRitchie(_) => todo!(),
            TraitItemKind::AssocFormal => todo!(),
            TraitItemKind::AssocConst => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TraitItemSynDecl {
    AssocFn(TraitAssocFnSynDecl),
    MethodFn(TraitMethodFnSynDecl),
    AssocType(TraitAssocTypeSynDecl),
    AssocVal(TraitAssocValSynDecl),
}

/// # constructor
impl TraitItemSynDecl {
    fn from_node_decl(
        db: &::salsa::Db,
        path: TraitItemPath,
        syn_node_decl: TraitItemSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(match syn_node_decl {
            TraitItemSynNodeDecl::AssocFn(syn_node_decl) => {
                TraitAssocFnSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TraitItemSynNodeDecl::MethodFn(syn_node_decl) => {
                TraitMethodFnSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TraitItemSynNodeDecl::AssocType(syn_node_decl) => {
                TraitAssocTypeSynDecl::from_node_decl(path, syn_node_decl, db)?.into()
            }
            TraitItemSynNodeDecl::AssocVal(_) => todo!(),
        })
    }
}

/// # getters
impl TraitItemSynDecl {
    pub fn path(self, db: &::salsa::Db) -> TraitItemPath {
        match self {
            TraitItemSynDecl::AssocFn(slf) => slf.path(db),
            TraitItemSynDecl::MethodFn(slf) => slf.path(db),
            TraitItemSynDecl::AssocType(slf) => slf.path(db),
            TraitItemSynDecl::AssocVal(slf) => slf.path(db),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a [TemplateSynParameterData] {
        match self {
            TraitItemSynDecl::AssocFn(slf) => slf.template_parameters(db),
            TraitItemSynDecl::MethodFn(slf) => slf.template_parameters(db),
            TraitItemSynDecl::AssocType(slf) => slf.template_parameters(db),
            TraitItemSynDecl::AssocVal(_slf) => &[],
        }
    }

    pub fn parenate_parameters<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> Option<&'a [ParenateParameterSyndicate]> {
        match self {
            TraitItemSynDecl::AssocFn(syn_decl) => Some(syn_decl.parenate_parameters(db)),
            TraitItemSynDecl::MethodFn(syn_decl) => Some(syn_decl.parenate_parameters(db)),
            TraitItemSynDecl::AssocType(_) => None,
            TraitItemSynDecl::AssocVal(_) => None,
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            TraitItemSynDecl::AssocFn(slf) => slf.syn_expr_region(db),
            TraitItemSynDecl::MethodFn(slf) => slf.syn_expr_region(db),
            TraitItemSynDecl::AssocType(slf) => slf.syn_expr_region(db),
            TraitItemSynDecl::AssocVal(slf) => slf.syn_expr_region(db),
        }
    }
}

impl HasSynDecl for TraitItemPath {
    type Decl = TraitItemSynDecl;

    fn syn_decl(self, db: &::salsa::Db) -> DeclResult<Self::Decl> {
        trai_item_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_item_syn_decl(
    db: &::salsa::Db,
    path: TraitItemPath,
) -> DeclResult<TraitItemSynDecl> {
    let syn_node_decl = path.syn_node_path(db).syn_node_decl(db);
    TraitItemSynDecl::from_node_decl(db, path, syn_node_decl)
}
