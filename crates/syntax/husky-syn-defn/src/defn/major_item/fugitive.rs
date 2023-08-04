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
#[salsa::debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum FugitiveSynNodeDefn {
    Fn(FnSynNodeDefn),
    // Function(FunctionDefn),
    Val(ValSynNodeDefn),
    Gn(GnSynNodeDefn),
    // AliasType(TypeAliasDefn)
}

impl FugitiveSynNodeDefn {
    pub fn syn_node_decl(self, db: &dyn SynDefnDb) -> FugitiveSynNodeDecl {
        match self {
            FugitiveSynNodeDefn::Fn(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            FugitiveSynNodeDefn::Val(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            FugitiveSynNodeDefn::Gn(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
        }
    }

    pub fn syn_node_path(self, db: &dyn SynDefnDb) -> FugitiveSynNodePath {
        todo!()
        // match self {
        //     FugitiveDefn::Fn(defn) => defn.path(db),
        //     FugitiveDefn::Val(defn) => defn.path(db),
        //     FugitiveDefn::Gn(defn) => defn.path(db),
        // }
    }

    pub fn syn_expr_region(self, db: &dyn SynDefnDb) -> SynExprRegion {
        match self {
            FugitiveSynNodeDefn::Fn(defn) => defn.syn_expr_region(db),
            FugitiveSynNodeDefn::Val(defn) => defn.syn_expr_region(db),
            FugitiveSynNodeDefn::Gn(defn) => defn.syn_expr_region(db),
        }
    }
}

impl HasSynNodeDefn for FugitiveSynNodePath {
    type SynNodeDefn = FugitiveSynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::SynNodeDefn {
        fugitive_syn_node_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn fugitive_syn_node_defn(
    db: &dyn SynDefnDb,
    syn_node_path: FugitiveSynNodePath,
) -> FugitiveSynNodeDefn {
    match syn_node_path.syn_node_decl(db) {
        FugitiveSynNodeDecl::Fn(syn_node_decl) => {
            FnSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        FugitiveSynNodeDecl::Val(syn_node_decl) => {
            ValSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        FugitiveSynNodeDecl::Gn(syn_node_decl) => {
            GnSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum FugitiveSynDefn {
    Fn(FnSynDefn),
    // Function(FunctionDefn),
    Val(ValSynDefn),
    Gn(GnSynDefn),
    // AliasType(TypeAliasDefn)
}

impl FugitiveSynDefn {
    pub fn decl(self, db: &dyn SynDefnDb) -> FugitiveSynDecl {
        match self {
            FugitiveSynDefn::Fn(defn) => defn.decl(db).into(),
            FugitiveSynDefn::Val(defn) => defn.decl(db).into(),
            FugitiveSynDefn::Gn(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn SynDefnDb) -> FugitivePath {
        todo!()
        // match self {
        //     FugitiveDefn::Fn(defn) => defn.path(db),
        //     FugitiveDefn::Val(defn) => defn.path(db),
        //     FugitiveDefn::Gn(defn) => defn.path(db),
        // }
    }
    pub fn syn_expr_region(self, db: &dyn SynDefnDb) -> SynExprRegion {
        match self {
            FugitiveSynDefn::Fn(defn) => defn.syn_expr_region(db),
            FugitiveSynDefn::Val(defn) => defn.syn_expr_region(db),
            FugitiveSynDefn::Gn(defn) => defn.syn_expr_region(db),
        }
    }
}

impl HasSynDefn for FugitivePath {
    type SynDefn = FugitiveSynDefn;

    fn syn_defn(self, db: &dyn SynDefnDb) -> SynDefnResult<Self::SynDefn> {
        fugitive_syn_defn(db, self)
    }
}

#[salsa::tracked(jar= SynDefnJar)]
pub(crate) fn fugitive_syn_defn(
    db: &dyn SynDefnDb,
    path: FugitivePath,
) -> SynDefnResult<FugitiveSynDefn> {
    Ok(match path.syn_decl(db)? {
        FugitiveSynDecl::Fn(decl) => FnSynDefn::new(db, path, decl).into(),
        FugitiveSynDecl::Val(decl) => ValSynDefn::new(db, path, decl).into(),
        FugitiveSynDecl::Gn(decl) => GnSynDefn::new(db, path, decl).into(),
    })
}
