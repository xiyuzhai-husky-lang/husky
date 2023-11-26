mod r#enum;
mod r#extern;
mod inductive;
mod props_struct;
mod record;
mod structure;
mod tuple_struct;
mod union;
mod unit_struct;

pub use self::inductive::*;
pub use self::props_struct::*;
pub use self::r#enum::*;
pub use self::r#extern::*;
pub use self::record::*;
pub use self::structure::*;
pub use self::tuple_struct::*;
pub use self::union::*;
pub use self::unit_struct::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
#[enum_class::from_variants]
pub enum TypeSynNodeDefn {
    Enum(EnumTypeSynNodeDefn),
    Inductive(InductiveTypeSynNodeDefn),
    Record(RecordTypeSynNodeDefn),
    PropsStruct(PropsStructTypeSynNodeDefn),
    TupleStruct(TupleStructTypeSynNodeDefn),
    UnitStruct(UnitStructTypeSynNodeDefn),
    Structure(StructureTypeSynNodeDefn),
    Extern(ExternTypeSynNodeDefn),
    Union(UnionTypeSynNodeDefn),
}

impl TypeSynNodeDefn {
    pub fn syn_node_decl(self, db: &::salsa::Db) -> TypeSynNodeDecl {
        match self {
            TypeSynNodeDefn::Enum(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            TypeSynNodeDefn::Inductive(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            TypeSynNodeDefn::Record(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            TypeSynNodeDefn::UnitStruct(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            TypeSynNodeDefn::TupleStruct(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            TypeSynNodeDefn::PropsStruct(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            TypeSynNodeDefn::Structure(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            TypeSynNodeDefn::Extern(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            TypeSynNodeDefn::Union(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
        }
    }

    pub fn syn_node_path(self, db: &::salsa::Db) -> TypeSynNodePath {
        match self {
            TypeSynNodeDefn::Enum(defn) => defn.syn_node_path(db),
            TypeSynNodeDefn::Inductive(defn) => defn.syn_node_path(db),
            TypeSynNodeDefn::Record(defn) => defn.syn_node_path(db),
            TypeSynNodeDefn::UnitStruct(defn) => defn.syn_node_path(db),
            TypeSynNodeDefn::TupleStruct(defn) => defn.syn_node_path(db),
            TypeSynNodeDefn::PropsStruct(defn) => defn.syn_node_path(db),
            TypeSynNodeDefn::Structure(defn) => defn.syn_node_path(db),
            TypeSynNodeDefn::Extern(defn) => defn.syn_node_path(db),
            TypeSynNodeDefn::Union(defn) => defn.syn_node_path(db),
        }
    }
}

impl HasSynNodeDefn for TypeSynNodePath {
    type SynNodeDefn = TypeSynNodeDefn;

    fn syn_node_defn(self, db: &::salsa::Db) -> Self::SynNodeDefn {
        ty_syn_node_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn ty_syn_node_defn(
    db: &::salsa::Db,
    syn_node_path: TypeSynNodePath,
) -> TypeSynNodeDefn {
    match syn_node_path.syn_node_decl(db) {
        TypeSynNodeDecl::Enum(syn_node_decl) => {
            EnumTypeSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        TypeSynNodeDecl::PropsStruct(syn_node_decl) => {
            PropsStructTypeSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        TypeSynNodeDecl::TupleStruct(syn_node_decl) => {
            TupleStructTypeSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        TypeSynNodeDecl::UnitStruct(syn_node_decl) => {
            UnitStructTypeSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        TypeSynNodeDecl::Record(syn_node_decl) => {
            RecordTypeSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        TypeSynNodeDecl::Inductive(syn_node_decl) => {
            InductiveTypeSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        TypeSynNodeDecl::Structure(syn_node_decl) => {
            StructureTypeSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        TypeSynNodeDecl::Extern(syn_node_decl) => {
            ExternTypeSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        TypeSynNodeDecl::Union(syn_node_decl) => {
            UnionTypeSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
#[enum_class::from_variants]
pub enum TypeSynDefn {
    Enum(EnumTypeSynDefn),
    Inductive(InductiveTypeSynDefn),
    Record(RecordTypeSynDefn),
    PropsStruct(PropsStructTypeSynDefn),
    TupleStruct(TupleStructTypeSynDefn),
    UnitStruct(UnitStructTypeSynDefn),
    Structure(StructureTypeSynDefn),
    Extern(ExternTypeSynDefn),
    Union(UnionTypeSynDefn),
}

impl TypeSynDefn {
    pub fn decl(self, db: &::salsa::Db) -> TypeSynDecl {
        match self {
            TypeSynDefn::Enum(defn) => defn.decl(db).into(),
            TypeSynDefn::Inductive(defn) => defn.decl(db).into(),
            TypeSynDefn::Record(defn) => defn.decl(db).into(),
            TypeSynDefn::UnitStruct(defn) => defn.decl(db).into(),
            TypeSynDefn::TupleStruct(defn) => defn.decl(db).into(),
            TypeSynDefn::PropsStruct(defn) => defn.decl(db).into(),
            TypeSynDefn::Structure(defn) => defn.decl(db).into(),
            TypeSynDefn::Extern(defn) => defn.decl(db).into(),
            TypeSynDefn::Union(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> TypePath {
        match self {
            TypeSynDefn::Enum(defn) => defn.path(db),
            TypeSynDefn::Inductive(defn) => defn.path(db),
            TypeSynDefn::Record(defn) => defn.path(db),
            TypeSynDefn::UnitStruct(defn) => defn.path(db),
            TypeSynDefn::TupleStruct(defn) => defn.path(db),
            TypeSynDefn::PropsStruct(defn) => defn.path(db),
            TypeSynDefn::Structure(defn) => defn.path(db),
            TypeSynDefn::Extern(defn) => defn.path(db),
            TypeSynDefn::Union(defn) => defn.path(db),
        }
    }
}

impl HasSynDefn for TypePath {
    type SynDefn = TypeSynDefn;

    fn syn_defn(self, db: &::salsa::Db) -> SynDefnResult<Self::SynDefn> {
        ty_syn_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn ty_syn_defn(db: &::salsa::Db, path: TypePath) -> SynDefnResult<TypeSynDefn> {
    Ok(match path.syn_decl(db)? {
        TypeSynDecl::Enum(decl) => EnumTypeSynDefn::new(db, path, decl).into(),
        TypeSynDecl::PropsStruct(decl) => PropsStructTypeSynDefn::new(db, path, decl).into(),
        TypeSynDecl::TupleStruct(decl) => TupleStructTypeSynDefn::new(db, path, decl).into(),
        TypeSynDecl::UnitStruct(decl) => UnitStructTypeSynDefn::new(db, path, decl).into(),
        TypeSynDecl::Record(decl) => RecordTypeSynDefn::new(db, path, decl).into(),
        TypeSynDecl::Inductive(decl) => InductiveTypeSynDefn::new(db, path, decl).into(),
        TypeSynDecl::Structure(decl) => StructureTypeSynDefn::new(db, path, decl).into(),
        TypeSynDecl::Extern(decl) => ExternTypeSynDefn::new(db, path, decl).into(),
        TypeSynDecl::Union(decl) => UnionTypeSynDefn::new(db, path, decl).into(),
    })
}
