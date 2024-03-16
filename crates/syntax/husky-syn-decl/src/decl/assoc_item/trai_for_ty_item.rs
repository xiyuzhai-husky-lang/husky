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
pub enum TraitForTypeItemSynNodeDecl {
    AssocFn(TraitForTypeAssocFnSynNodeDecl),
    MethodFn(TraitForTypeMethodFnSynNodeDecl),
    AssocType(TraitForTypeAssocTypeSynNodeDecl),
    AssocVal(TraitForTypeAssocValSynNodeDecl),
}

impl From<TraitForTypeItemSynNodeDecl> for ItemSynNodeDecl {
    fn from(decl: TraitForTypeItemSynNodeDecl) -> Self {
        ItemSynNodeDecl::AssocItem(decl.into())
    }
}

impl TraitForTypeItemSynNodeDecl {
    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            TraitForTypeItemSynNodeDecl::AssocFn(slf) => slf.syn_expr_region(db),
            TraitForTypeItemSynNodeDecl::MethodFn(slf) => slf.syn_expr_region(db),
            TraitForTypeItemSynNodeDecl::AssocType(slf) => slf.syn_expr_region(db),
            TraitForTypeItemSynNodeDecl::AssocVal(slf) => slf.syn_expr_region(db),
        }
    }

    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        match self {
            TraitForTypeItemSynNodeDecl::AssocFn(slf) => slf.errors(db),
            TraitForTypeItemSynNodeDecl::MethodFn(slf) => slf.errors(db),
            TraitForTypeItemSynNodeDecl::AssocType(slf) => slf.errors(db),
            TraitForTypeItemSynNodeDecl::AssocVal(slf) => slf.errors(db),
        }
    }
}

impl HasSynNodeDecl for TraitForTypeItemSynNodePath {
    type NodeDecl = TraitForTypeItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        trai_for_ty_item_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_for_ty_item_syn_node_decl(
    db: &::salsa::Db,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> TraitForTypeItemSynNodeDecl {
    let parser = DeclParser::new(db, syn_node_path.into());
    parser.parse_trai_for_ty_item_syn_node_decl()
}

impl<'a> DeclParser<'a> {
    fn parse_trai_for_ty_item_syn_node_decl(&self) -> TraitForTypeItemSynNodeDecl {
        let db = self.db();
        let ItemSynNodePath::AssocItem(AssocItemSynNodePath::TraitForTypeItem(syn_node_path)) =
            self.syn_node_path()
        else {
            unreachable!()
        };
        match syn_node_path.data(db).item_kind(db) {
            TraitItemKind::MemoizedField => todo!(),
            TraitItemKind::MethodRitchie(_) => self
                .parse_trai_for_ty_method_fn_node_decl(syn_node_path)
                .into(),
            TraitItemKind::AssocType => self
                .parse_trai_for_ty_assoc_ty_node_decl(syn_node_path)
                .into(),
            TraitItemKind::AssocVal => todo!(),
            TraitItemKind::AssocRitchie(_) => self
                .parse_trai_for_ty_assoc_fn_node_decl(syn_node_path)
                .into(),
            TraitItemKind::AssocFormal => todo!(),
            TraitItemKind::AssocConst => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TraitForTypeItemSynDecl {
    AssocFn(TraitForTypeAssocFnSynDecl),
    MethodFn(TraitForTypeMethodFnSynDecl),
    AssocType(TraitForTypeAssocTypeSynDecl),
    AssocVal(TraitForTypeAssocValSynDecl),
}

impl From<TraitForTypeItemSynDecl> for SynDecl {
    fn from(decl: TraitForTypeItemSynDecl) -> Self {
        SynDecl::AssocItem(decl.into())
    }
}

/// # constructor
impl TraitForTypeItemSynDecl {
    fn from_node_decl(
        db: &::salsa::Db,
        path: TraitForTypeItemPath,
        syn_node_decl: TraitForTypeItemSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(match syn_node_decl {
            TraitForTypeItemSynNodeDecl::AssocFn(syn_node_decl) => {
                TraitForTypeAssocFnSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TraitForTypeItemSynNodeDecl::MethodFn(syn_node_decl) => {
                TraitForTypeMethodFnSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TraitForTypeItemSynNodeDecl::AssocType(syn_node_decl) => {
                TraitForTypeAssocTypeSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            TraitForTypeItemSynNodeDecl::AssocVal(_) => todo!(),
        })
    }
}

/// # getters
impl TraitForTypeItemSynDecl {
    pub fn path(self, db: &::salsa::Db) -> TraitForTypeItemPath {
        match self {
            TraitForTypeItemSynDecl::AssocFn(decl) => decl.path(db),
            TraitForTypeItemSynDecl::MethodFn(decl) => decl.path(db),
            TraitForTypeItemSynDecl::AssocType(decl) => decl.path(db),
            TraitForTypeItemSynDecl::AssocVal(decl) => decl.path(db),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a [TemplateSynParameterData] {
        match self {
            TraitForTypeItemSynDecl::AssocFn(decl) => decl.template_parameters(db),
            TraitForTypeItemSynDecl::MethodFn(decl) => decl.template_parameters(db),
            TraitForTypeItemSynDecl::AssocType(decl) => decl.template_parameters(db),
            TraitForTypeItemSynDecl::AssocVal(_) => &[],
        }
    }

    pub fn parenate_parameters<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> Option<&'a [ParenateParameterSyndicate]> {
        match self {
            TraitForTypeItemSynDecl::AssocFn(syn_decl) => Some(syn_decl.parenate_parameters(db)),
            TraitForTypeItemSynDecl::MethodFn(syn_decl) => Some(syn_decl.parenate_parameters(db)),
            TraitForTypeItemSynDecl::AssocType(_) => None,
            TraitForTypeItemSynDecl::AssocVal(_) => None,
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            TraitForTypeItemSynDecl::AssocFn(decl) => decl.syn_expr_region(db),
            TraitForTypeItemSynDecl::MethodFn(decl) => decl.syn_expr_region(db),
            TraitForTypeItemSynDecl::AssocType(decl) => decl.syn_expr_region(db),
            TraitForTypeItemSynDecl::AssocVal(decl) => decl.syn_expr_region(db),
        }
    }
}

impl HasSynDecl for TraitForTypeItemPath {
    type Decl = TraitForTypeItemSynDecl;

    fn syn_decl(self, db: &::salsa::Db) -> DeclResult<Self::Decl> {
        trai_for_ty_item_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_for_ty_item_syn_decl(
    db: &::salsa::Db,
    path: TraitForTypeItemPath,
) -> DeclResult<TraitForTypeItemSynDecl> {
    let syn_node_decl = path.syn_node_path(db).syn_node_decl(db);
    TraitForTypeItemSynDecl::from_node_decl(db, path, syn_node_decl)
}
