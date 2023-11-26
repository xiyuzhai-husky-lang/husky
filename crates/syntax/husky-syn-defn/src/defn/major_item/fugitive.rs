mod r#fn;
mod gn;
mod type_alias;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::type_alias::*;
pub use self::val::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
#[enum_class::from_variants]
pub enum FugitiveSynNodeDefn {
    FunctionFn(FnSynNodeDefn),
    // Function(FunctionDefn),
    Val(ValSynNodeDefn),
    Gn(GnSynNodeDefn),
    // AliasType(TypeAliasDefn)
}

impl FugitiveSynNodeDefn {
    pub fn syn_node_decl(self, db: &::salsa::Db) -> FugitiveSynNodeDecl {
        match self {
            FugitiveSynNodeDefn::FunctionFn(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
            FugitiveSynNodeDefn::Val(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            FugitiveSynNodeDefn::Gn(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
        }
    }

    pub fn syn_node_path(self, _db: &::salsa::Db) -> FugitiveSynNodePath {
        todo!()
        // match self {
        //     FugitiveDefn::Fn(defn) => defn.path(db),
        //     FugitiveDefn::Val(defn) => defn.path(db),
        //     FugitiveDefn::Gn(defn) => defn.path(db),
        // }
    }

    pub fn body_with_syn_expr_region(
        self,
        db: &::salsa::Db,
    ) -> Option<(SynExprIdx, SynExprRegion)> {
        match self {
            FugitiveSynNodeDefn::FunctionFn(defn) => defn.body_with_syn_expr_region(db),
            FugitiveSynNodeDefn::Val(defn) => defn.body_with_syn_expr_region(db),
            FugitiveSynNodeDefn::Gn(defn) => defn.body_with_syn_expr_region(db),
        }
    }
}

impl HasSynNodeDefn for FugitiveSynNodePath {
    type SynNodeDefn = FugitiveSynNodeDefn;

    fn syn_node_defn(self, db: &::salsa::Db) -> Self::SynNodeDefn {
        fugitive_syn_node_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn fugitive_syn_node_defn(
    db: &::salsa::Db,
    syn_node_path: FugitiveSynNodePath,
) -> FugitiveSynNodeDefn {
    match syn_node_path.syn_node_decl(db) {
        FugitiveSynNodeDecl::FunctionFn(syn_node_decl) => {
            FnSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        FugitiveSynNodeDecl::Val(syn_node_decl) => {
            ValSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        FugitiveSynNodeDecl::FunctionGn(syn_node_decl) => {
            GnSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
#[enum_class::from_variants]
pub enum FugitiveSynDefn {
    FunctionFn(FnSynDefn),
    // Function(FunctionDefn),
    Val(ValSynDefn),
    FunctionGn(GnSynDefn),
    // AliasType(TypeAliasDefn)
}

impl FugitiveSynDefn {
    pub fn decl(self, db: &::salsa::Db) -> FugitiveSynDecl {
        match self {
            FugitiveSynDefn::FunctionFn(defn) => defn.decl(db).into(),
            FugitiveSynDefn::Val(defn) => defn.decl(db).into(),
            FugitiveSynDefn::FunctionGn(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> FugitivePath {
        match self {
            FugitiveSynDefn::FunctionFn(defn) => defn.path(db),
            FugitiveSynDefn::Val(defn) => defn.path(db),
            FugitiveSynDefn::FunctionGn(defn) => defn.path(db),
        }
    }

    pub fn body_with_syn_expr_region(
        self,
        db: &::salsa::Db,
    ) -> Option<(SynExprIdx, SynExprRegion)> {
        match self {
            FugitiveSynDefn::FunctionFn(defn) => defn.body_with_syn_expr_region(db),
            FugitiveSynDefn::Val(defn) => defn.body_with_syn_expr_region(db),
            FugitiveSynDefn::FunctionGn(defn) => defn.body_with_syn_expr_region(db),
        }
    }
}

impl HasSynDefn for FugitivePath {
    type SynDefn = FugitiveSynDefn;

    fn syn_defn(self, db: &::salsa::Db) -> SynDefnResult<Self::SynDefn> {
        fugitive_syn_defn(db, self)
    }
}

// #[salsa::tracked(jar= SynDefnJar)]
pub(crate) fn fugitive_syn_defn(
    db: &::salsa::Db,
    path: FugitivePath,
) -> SynDefnResult<FugitiveSynDefn> {
    Ok(match path.syn_decl(db)? {
        FugitiveSynDecl::FunctionFn(decl) => FnSynDefn::new(db, path, decl).into(),
        FugitiveSynDecl::Val(decl) => ValSynDefn::new(db, path, decl).into(),
        FugitiveSynDecl::FunctionGn(decl) => GnSynDefn::new(db, path, decl).into(),
    })
}
