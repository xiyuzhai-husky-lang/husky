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
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum FugitiveNodeDefn {
    Fn(FnNodeDefn),
    // Function(FunctionDefn),
    Val(ValNodeDefn),
    Gn(GnNodeDefn),
    // AliasType(TypeAliasDefn)
}

impl HasNodeDefn for FugitiveNodePath {
    type NodeDefn = FugitiveNodeDefn;

    fn node_defn(self, db: &dyn DefnDb) -> Self::NodeDefn {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum FugitiveDefn {
    Fn(FnDefn),
    // Function(FunctionDefn),
    Val(ValDefn),
    Gn(GnDefn),
    // AliasType(TypeAliasDefn)
}

impl FugitiveDefn {
    pub fn decl(self, db: &dyn DefnDb) -> FugitiveDecl {
        match self {
            FugitiveDefn::Fn(defn) => defn.decl(db).into(),
            FugitiveDefn::Val(defn) => defn.decl(db).into(),
            FugitiveDefn::Gn(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn DefnDb) -> FugitivePath {
        todo!()
        // match self {
        //     FugitiveDefn::Fn(defn) => defn.path(db),
        //     FugitiveDefn::Val(defn) => defn.path(db),
        //     FugitiveDefn::Gn(defn) => defn.path(db),
        // }
    }
    pub fn expr_region(self, db: &dyn DefnDb) -> ExprRegion {
        match self {
            FugitiveDefn::Fn(defn) => defn.expr_region(db),
            FugitiveDefn::Val(defn) => defn.expr_region(db),
            FugitiveDefn::Gn(defn) => defn.expr_region(db),
        }
    }
}

impl HasDefn for FugitivePath {
    type Defn = FugitiveDefn;

    fn defn(self, db: &dyn DefnDb) -> DefnResult<Self::Defn> {
        fugitive_defn(db, self)
    }
}

#[salsa::tracked(jar= DefnJar)]
pub(crate) fn fugitive_defn(db: &dyn DefnDb, path: FugitivePath) -> DefnResult<FugitiveDefn> {
    todo!()
    // match self {
    //     FugitivePath::Fn(path) => path.defn(db).into(),
    //     FugitivePath::Val(decl) => decl.defn(db).into(),
    //     FugitivePath::Gn(decl) => decl.defn(db).into(),
    // }
}
