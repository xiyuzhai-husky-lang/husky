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
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum TypeNodeDefn {
    Enum(EnumTypeNodeDefn),
    Inductive(InductiveTypeNodeDefn),
    Record(RecordTypeNodeDefn),
    PropsStruct(PropsStructTypeNodeDefn),
    TupleStruct(TupleStructTypeNodeDefn),
    UnitStruct(UnitStructTypeNodeDefn),
    Structure(StructureTypeNodeDefn),
    Extern(ExternTypeNodeDefn),
    Union(UnionTypeNodeDefn),
}

impl TypeNodeDefn {
    pub fn node_decl(self, db: &dyn DefnDb) -> TypeNodeDecl {
        match self {
            TypeNodeDefn::Enum(node_defn) => node_defn.node_decl(db).into(),
            TypeNodeDefn::Inductive(node_defn) => node_defn.node_decl(db).into(),
            TypeNodeDefn::Record(node_defn) => node_defn.node_decl(db).into(),
            TypeNodeDefn::UnitStruct(node_defn) => node_defn.node_decl(db).into(),
            TypeNodeDefn::TupleStruct(node_defn) => node_defn.node_decl(db).into(),
            TypeNodeDefn::PropsStruct(node_defn) => node_defn.node_decl(db).into(),
            TypeNodeDefn::Structure(node_defn) => node_defn.node_decl(db).into(),
            TypeNodeDefn::Extern(node_defn) => node_defn.node_decl(db).into(),
            TypeNodeDefn::Union(node_defn) => node_defn.node_decl(db).into(),
        }
    }

    pub fn path(self, db: &dyn DefnDb) -> TypePath {
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

impl HasNodeDefn for TypeNodePath {
    type NodeDefn = TypeNodeDefn;

    fn node_defn(self, db: &dyn DefnDb) -> Self::NodeDefn {
        ty_node_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn ty_node_defn(db: &dyn DefnDb, node_path: TypeNodePath) -> TypeNodeDefn {
    match node_path.node_decl(db) {
        TypeNodeDecl::Enum(node_decl) => EnumTypeNodeDefn::new(db, node_path, node_decl).into(),
        TypeNodeDecl::PropsStruct(node_decl) => {
            PropsStructTypeNodeDefn::new(db, node_path, node_decl).into()
        }
        TypeNodeDecl::TupleStruct(node_decl) => {
            TupleStructTypeNodeDefn::new(db, node_path, node_decl).into()
        }
        TypeNodeDecl::UnitStruct(node_decl) => {
            UnitStructTypeNodeDefn::new(db, node_path, node_decl).into()
        }
        TypeNodeDecl::Record(node_decl) => RecordTypeNodeDefn::new(db, node_path, node_decl).into(),
        TypeNodeDecl::Inductive(node_decl) => {
            InductiveTypeNodeDefn::new(db, node_path, node_decl).into()
        }
        TypeNodeDecl::Structure(node_decl) => {
            StructureTypeNodeDefn::new(db, node_path, node_decl).into()
        }
        TypeNodeDecl::Extern(node_decl) => ExternTypeNodeDefn::new(db, node_path, node_decl).into(),
        TypeNodeDecl::Union(node_decl) => UnionTypeNodeDefn::new(db, node_path, node_decl).into(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum TypeDefn {
    Enum(EnumTypeDefn),
    Inductive(InductiveTypeDefn),
    Record(RecordTypeDefn),
    PropsStruct(PropsStructTypeDefn),
    TupleStruct(TupleStructTypeDefn),
    UnitStruct(UnitStructTypeDefn),
    Structure(StructureTypeDefn),
    Extern(ExternTypeDefn),
    Union(UnionTypeDefn),
}

impl TypeDefn {
    pub fn decl(self, db: &dyn DefnDb) -> TypeDecl {
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

    pub fn path(self, db: &dyn DefnDb) -> TypePath {
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

impl HasDefn for TypePath {
    type Defn = TypeDefn;

    fn defn(self, db: &dyn DefnDb) -> DefnResult<Self::Defn> {
        ty_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn ty_defn(db: &dyn DefnDb, path: TypePath) -> DefnResult<TypeDefn> {
    Ok(match path.decl(db)? {
        TypeDecl::Enum(decl) => EnumTypeDefn::new(db, path, decl).into(),
        TypeDecl::PropsStruct(decl) => PropsStructTypeDefn::new(db, path, decl).into(),
        TypeDecl::TupleStruct(decl) => TupleStructTypeDefn::new(db, path, decl).into(),
        TypeDecl::UnitStruct(decl) => UnitStructTypeDefn::new(db, path, decl).into(),
        TypeDecl::Record(decl) => RecordTypeDefn::new(db, path, decl).into(),
        TypeDecl::Inductive(decl) => InductiveTypeDefn::new(db, path, decl).into(),
        TypeDecl::Structure(decl) => StructureTypeDefn::new(db, path, decl).into(),
        TypeDecl::Extern(decl) => ExternTypeDefn::new(db, path, decl).into(),
        TypeDecl::Union(decl) => UnionTypeDefn::new(db, path, decl).into(),
    })
}
