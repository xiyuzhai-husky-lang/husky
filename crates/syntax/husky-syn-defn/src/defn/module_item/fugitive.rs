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
#[salsa::derive_debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum FugitiveSynNodeDefn {
    Fn(FnSynNodeDefn),
    // Function(FunctionDefn),
    Val(ValSynNodeDefn),
    Gn(GnNodeSynDefn),
    // AliasType(TypeAliasDefn)
}

impl FugitiveSynNodeDefn {
    pub fn syn_node_decl(self, db: &dyn SynDefnDb) -> FugitiveNodeDecl {
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

    pub fn expr_region(self, db: &dyn SynDefnDb) -> SynExprRegion {
        match self {
            FugitiveSynNodeDefn::Fn(defn) => defn.expr_region(db),
            FugitiveSynNodeDefn::Val(defn) => defn.expr_region(db),
            FugitiveSynNodeDefn::Gn(defn) => defn.expr_region(db),
        }
    }
}

impl HasSynNodeDefn for FugitiveSynNodePath {
    type NodeDefn = FugitiveSynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::NodeDefn {
        fugitive_syn_node_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn fugitive_syn_node_defn(
    db: &dyn SynDefnDb,
    syn_node_path: FugitiveSynNodePath,
) -> FugitiveSynNodeDefn {
    match syn_node_path.syn_node_decl(db) {
        FugitiveNodeDecl::Fn(syn_node_decl) => {
            FnSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        FugitiveNodeDecl::Val(syn_node_decl) => {
            ValSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        FugitiveNodeDecl::Gn(syn_node_decl) => {
            GnNodeSynDefn::new(db, syn_node_path, syn_node_decl).into()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum FugitiveDefn {
    Fn(FnDefn),
    // Function(FunctionDefn),
    Val(ValSynDefn),
    Gn(GnDefn),
    // AliasType(TypeAliasDefn)
}

impl FugitiveDefn {
    pub fn decl(self, db: &dyn SynDefnDb) -> FugitiveDecl {
        match self {
            FugitiveDefn::Fn(defn) => defn.decl(db).into(),
            FugitiveDefn::Val(defn) => defn.decl(db).into(),
            FugitiveDefn::Gn(defn) => defn.decl(db).into(),
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
    pub fn expr_region(self, db: &dyn SynDefnDb) -> SynExprRegion {
        match self {
            FugitiveDefn::Fn(defn) => defn.expr_region(db),
            FugitiveDefn::Val(defn) => defn.expr_region(db),
            FugitiveDefn::Gn(defn) => defn.expr_region(db),
        }
    }
}

impl HasDefn for FugitivePath {
    type Defn = FugitiveDefn;

    fn defn(self, db: &dyn SynDefnDb) -> DefnResult<Self::Defn> {
        fugitive_defn(db, self)
    }
}

#[salsa::tracked(jar= SynDefnJar)]
pub(crate) fn fugitive_defn(db: &dyn SynDefnDb, path: FugitivePath) -> DefnResult<FugitiveDefn> {
    Ok(match path.decl(db)? {
        FugitiveDecl::Fn(decl) => FnDefn::new(db, path, decl).into(),
        FugitiveDecl::Val(decl) => ValSynDefn::new(db, path, decl).into(),
        FugitiveDecl::Gn(decl) => GnDefn::new(db, path, decl).into(),
    })
}
