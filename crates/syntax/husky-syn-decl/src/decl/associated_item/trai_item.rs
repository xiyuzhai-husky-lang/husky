mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb, jar = SynDeclJar)]
#[enum_class::from_variants]
pub enum TraitItemSynNodeDecl {
    AssociatedFn(TraitAssociatedFnSynNodeDecl),
    MethodFn(TraitMethodFnSynNodeDecl),
    AssociatedType(TraitAssociatedTypeSynNodeDecl),
    AssociatedVal(TraitAssociatedValSynNodeDecl),
}

impl TraitItemSynNodeDecl {
    pub fn syn_node_path(self, db: &::salsa::Db) -> TraitItemSynNodePath {
        match self {
            TraitItemSynNodeDecl::AssociatedFn(decl) => decl.syn_node_path(db),
            TraitItemSynNodeDecl::MethodFn(decl) => decl.syn_node_path(db),
            TraitItemSynNodeDecl::AssociatedType(decl) => decl.syn_node_path(db),
            TraitItemSynNodeDecl::AssociatedVal(decl) => decl.syn_node_path(db),
        }
    }

    pub fn template_parameters<'a>(self, _db: &'a ::salsa::Db) -> &'a [TemplateSynParameterData] {
        match self {
            TraitItemSynNodeDecl::AssociatedFn(_) => todo!(),
            TraitItemSynNodeDecl::MethodFn(_) => todo!(),
            TraitItemSynNodeDecl::AssociatedType(_) => todo!(),
            TraitItemSynNodeDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            TraitItemSynNodeDecl::AssociatedFn(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TraitItemSynNodeDecl::MethodFn(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TraitItemSynNodeDecl::AssociatedType(syn_node_decl) => {
                syn_node_decl.syn_expr_region(db)
            }
            TraitItemSynNodeDecl::AssociatedVal(syn_node_decl) => syn_node_decl.syn_expr_region(db),
        }
    }

    pub fn errors(self, _db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        match self {
            TraitItemSynNodeDecl::AssociatedFn(_) => todo!(),
            TraitItemSynNodeDecl::MethodFn(_) => todo!(),
            TraitItemSynNodeDecl::AssociatedType(_) => todo!(),
            TraitItemSynNodeDecl::AssociatedVal(_) => todo!(),
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
            TraitItemKind::MethodFn => self.parse_trai_method_fn_node_decl(syn_node_path).into(),
            TraitItemKind::AssociatedType => self
                .parse_trai_associated_ty_node_decl(syn_node_path)
                .into(),
            TraitItemKind::AssociatedVal => todo!(),
            TraitItemKind::AssociatedFn => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb, jar = SynDeclJar)]
#[enum_class::from_variants]
pub enum TraitItemSynDecl {
    AssociatedFn(TraitAssociatedFnSynDecl),
    MethodFn(TraitMethodFnSynDecl),
    AssociatedType(TraitAssociatedTypeSynDecl),
    AssociatedVal(TraitAssociatedValSynDecl),
}

impl TraitItemSynDecl {
    pub fn path(self, _db: &::salsa::Db) -> TraitItemPath {
        match self {
            TraitItemSynDecl::AssociatedFn(_) => todo!(),
            TraitItemSynDecl::MethodFn(_) => todo!(),
            TraitItemSynDecl::AssociatedType(_) => todo!(),
            TraitItemSynDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn template_parameters<'a>(self, _db: &'a ::salsa::Db) -> &'a [TemplateSynParameterData] {
        match self {
            TraitItemSynDecl::AssociatedFn(_) => todo!(),
            TraitItemSynDecl::MethodFn(_) => todo!(),
            TraitItemSynDecl::AssociatedType(_) => todo!(),
            TraitItemSynDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn parenate_parameters<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> Option<&'a [ParenateSynParameterData]> {
        match self {
            TraitItemSynDecl::AssociatedFn(syn_decl) => Some(syn_decl.parenate_parameters(db)),
            TraitItemSynDecl::MethodFn(syn_decl) => Some(syn_decl.parenate_parameters(db)),
            TraitItemSynDecl::AssociatedType(_) => None,
            TraitItemSynDecl::AssociatedVal(_) => None,
        }
    }

    pub fn syn_expr_region(self, _db: &::salsa::Db) -> SynExprRegion {
        match self {
            TraitItemSynDecl::AssociatedFn(_) => todo!(),
            TraitItemSynDecl::MethodFn(_) => todo!(),
            TraitItemSynDecl::AssociatedType(_) => todo!(),
            TraitItemSynDecl::AssociatedVal(_) => todo!(),
        }
    }
}

impl HasSynDecl for TraitItemPath {
    type Decl = TraitItemSynDecl;

    fn syn_decl(self, _db: &::salsa::Db) -> DeclResult<Self::Decl> {
        todo!()
    }
}
