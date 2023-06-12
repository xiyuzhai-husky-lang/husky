use super::*;
use vec_like::VecPairMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum ImplBlockPath {
    Type(TypeImplBlockPath),
    TraitForType(TraitForTypeImplBlockPath),
    IllFormed(IllFormedImplBlockPath),
}

impl ImplBlockPath {
    pub fn module(self, db: &dyn EntityPathDb) -> ModulePath {
        match self {
            ImplBlockPath::Type(path) => path.module_path(db),
            ImplBlockPath::TraitForType(path) => path.module_path(db),
            ImplBlockPath::IllFormed(path) => path.module_path(db),
        }
    }
}

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar, constructor = new_inner)]
pub struct TypeImplBlockPath {
    pub module_path: ModulePath,
    pub ty_path: TypePath,
    pub disambiguator: u8,
}

impl TypeImplBlockPath {
    pub fn new(
        db: &dyn EntityPathDb,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        ty_path: TypePath,
    ) -> Self {
        TypeImplBlockPath::new_inner(
            db,
            module_path,
            ty_path,
            registry.issue_disambiguitor(module_path, ImplBlockKind::Type { ty_path }),
        )
    }
}

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar, constructor = new_inner)]
pub struct TraitForTypeImplBlockPath {
    pub module_path: ModulePath,
    pub trai_path: TraitPath,
    pub ty_path: TypePath,
    pub disambiguator: u8,
}

impl TraitForTypeImplBlockPath {
    pub fn new(
        db: &dyn EntityPathDb,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        trai_path: TraitPath,
        ty_path: TypePath,
    ) -> Self {
        TraitForTypeImplBlockPath::new_inner(
            db,
            module_path,
            trai_path,
            ty_path,
            registry.issue_disambiguitor(
                module_path,
                ImplBlockKind::TraitForType { ty_path, trai_path },
            ),
        )
    }
}

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar, constructor = new_inner)]
pub struct IllFormedImplBlockPath {
    pub module_path: ModulePath,
    pub disambiguator: u8,
}

impl IllFormedImplBlockPath {
    pub fn new(
        db: &dyn EntityPathDb,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
    ) -> Self {
        IllFormedImplBlockPath::new_inner(
            db,
            module_path,
            registry.issue_disambiguitor(module_path, ImplBlockKind::Err),
        )
    }
}

#[derive(Default)]
pub struct ImplBlockRegistry {
    next_disambiguitors: VecPairMap<(ModulePath, ImplBlockKind), u8>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
pub enum ImplBlockKind {
    Type {
        ty_path: TypePath,
    },
    TraitForType {
        ty_path: TypePath,
        trai_path: TraitPath,
    },
    Err,
}

impl ImplBlockRegistry {
    fn issue_disambiguitor(
        &mut self,
        module_path: ModulePath,
        impl_block_kind: ImplBlockKind,
    ) -> u8 {
        let next_disambiguitor = self
            .next_disambiguitors
            .get_mut_or_insert_default((module_path, impl_block_kind));
        let new_disambiguitor = *next_disambiguitor;
        *next_disambiguitor += 1;
        new_disambiguitor
    }
}
