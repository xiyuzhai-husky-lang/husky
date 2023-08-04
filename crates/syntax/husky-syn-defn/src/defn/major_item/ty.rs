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
#[salsa::debug_with_db(db = SynDefnDb)]
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
    pub fn syn_node_decl(self, db: &dyn SynDefnDb) -> TypeSynNodeDecl {
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

    pub fn path(self, db: &dyn SynDefnDb) -> TypePath {
        todo!()
        // match self {
        //     TypeDefn::Enum(defn) => defn.path(db),
        //     TypeDefn::Inductive(defn) => defn.path(db),
        //     TypeDefn::Record(defn) => defn.path(db),
        //     TypeDefn::UnitStruct(defn) => defn.path(db),
        //     TypeDefn::TupleStruct(defn) => defn.path(db),
        //     TypeDefn::PropsStruct(defn) => defn.path(db),
        //     TypeDefn::Structure(defn) => defn.path(db),
        //     TypeDefn::Extern(defn) => defn.path(db),
        //     TypeDefn::Union(defn) => defn.path(db),
        // }
    }
}

impl HasSynNodeDefn for TypeSynNodePath {
    type SynNodeDefn = TypeSynNodeDefn;

    fn syn_node_defn(self, db: &dyn SynDefnDb) -> Self::SynNodeDefn {
        ty_syn_node_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn ty_syn_node_defn(
    db: &dyn SynDefnDb,
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
#[salsa::debug_with_db(db = SynDefnDb)]
#[enum_class::from_variants]
pub enum TypeDefn {
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

impl TypeDefn {
    pub fn decl(self, db: &dyn SynDefnDb) -> TypeSynDecl {
        match self {
            TypeDefn::Enum(defn) => defn.decl(db).into(),
            TypeDefn::Inductive(defn) => defn.decl(db).into(),
            TypeDefn::Record(defn) => defn.decl(db).into(),
            TypeDefn::UnitStruct(defn) => defn.decl(db).into(),
            TypeDefn::TupleStruct(defn) => defn.decl(db).into(),
            TypeDefn::PropsStruct(defn) => defn.decl(db).into(),
            TypeDefn::Structure(defn) => defn.decl(db).into(),
            TypeDefn::Extern(defn) => defn.decl(db).into(),
            TypeDefn::Union(defn) => defn.decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn SynDefnDb) -> TypePath {
        todo!()
        // match self {
        //     TypeDefn::Enum(defn) => defn.path(db),
        //     TypeDefn::Inductive(defn) => defn.path(db),
        //     TypeDefn::Record(defn) => defn.path(db),
        //     TypeDefn::UnitStruct(defn) => defn.path(db),
        //     TypeDefn::TupleStruct(defn) => defn.path(db),
        //     TypeDefn::PropsStruct(defn) => defn.path(db),
        //     TypeDefn::Structure(defn) => defn.path(db),
        //     TypeDefn::Extern(defn) => defn.path(db),
        //     TypeDefn::Union(defn) => defn.path(db),
        // }
    }
}

impl HasSynDefn for TypePath {
    type SynDefn = TypeDefn;

    fn syn_defn(self, db: &dyn SynDefnDb) -> SynDefnResult<Self::SynDefn> {
        ty_syn_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn ty_syn_defn(db: &dyn SynDefnDb, path: TypePath) -> SynDefnResult<TypeDefn> {
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
