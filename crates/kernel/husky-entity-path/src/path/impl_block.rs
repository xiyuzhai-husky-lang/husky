use super::*;
use salsa::Db;
use vec_like::VecPairMap;

#[salsa::debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ImplBlockPath {
    TypeImplBlock(TypeImplBlockPath),
    TraitForTypeImplBlock(TraitForTypeImplBlockPath),
}

#[test]
fn item_path_size_works() {
    assert_eq!(
        std::mem::size_of::<ImplBlockPath>(),
        std::mem::size_of::<[u32; 2]>()
    )
}

impl std::ops::Deref for ImplBlockPath {
    type Target = ItemPathId;

    fn deref(&self) -> &Self::Target {
        unsafe { &std::mem::transmute::<_, &(u32, ItemPathId)>(self).1 }
    }
}

#[salsa::debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ImplBlockPathData {
    TypeImplBlock(TypeImplBlockPathData),
    TraitForTypeImplBlock(TraitForTypeImplBlockPathData),
}

impl ImplBlockPathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> ImplBlockPath {
        match self {
            ImplBlockPathData::TypeImplBlock(slf) => slf.item_path(id).into(),
            ImplBlockPathData::TraitForTypeImplBlock(slf) => slf.item_path(id).into(),
        }
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        match self {
            ImplBlockPathData::TypeImplBlock(data) => data.module_path(),
            ImplBlockPathData::TraitForTypeImplBlock(data) => data.module_path(),
        }
    }
}

#[salsa::debug_with_db]
#[salsa::as_id(jar = EntityPathJar)]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeImplBlockPath(ItemPathId);

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TypeImplBlockPathData {
    module_path: ModulePath,
    ty_path: TypePath,
    disambiguator: u8,
}

impl TypeImplBlockPath {
    pub fn data(self, db: &::salsa::Db) -> TypeImplBlockPathData {
        match self.0.data(db) {
            ItemPathData::ImplBlock(ImplBlockPathData::TypeImplBlock(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn ty_path(self, db: &::salsa::Db) -> TypePath {
        self.data(db).ty_path
    }

    pub(crate) fn show_aux(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.data(db).show_aux(f, db)
    }
}

impl TypeImplBlockPathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> TypeImplBlockPath {
        TypeImplBlockPath(id)
    }

    pub fn module_path(self) -> ModulePath {
        self.module_path
    }

    pub fn ty_path(self) -> TypePath {
        self.ty_path
    }

    pub fn disambiguator(self) -> u8 {
        self.disambiguator
    }

    pub(crate) fn show_aux(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.ty_path.show_aux(f, db)?;
        use std::fmt::Display;
        f.write_str("(")?;
        self.disambiguator.fmt(f);
        f.write_str(")")
    }
}

impl From<TypeImplBlockPath> for ItemPath {
    fn from(path: TypeImplBlockPath) -> Self {
        ItemPath::ImplBlock(path.into())
    }
}

impl TypeImplBlockPath {
    pub fn new(
        db: &::salsa::Db,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        ty_path: TypePath,
    ) -> Self {
        TypeImplBlockPath(ItemPathId::new(
            db,
            ItemPathData::ImplBlock(ImplBlockPathData::TypeImplBlock(TypeImplBlockPathData {
                module_path,
                ty_path,
                disambiguator: registry
                    .issue_disambiguitor(module_path, ImplBlockKind::Type { ty_path }),
            })),
        ))
    }
}

impl TypeImplBlockPathData {
    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        self.module_path.toolchain(db)
    }
}

#[salsa::as_id(jar = EntityPathJar)]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraitForTypeImplBlockPath(ItemPathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub struct TraitForTypeImplBlockPathData {
    module_path: ModulePath,
    trai_path: TraitPath,
    ty_sketch: TypeSketch,
    disambiguator: u8,
}

impl From<TraitForTypeImplBlockPath> for ItemPath {
    fn from(path: TraitForTypeImplBlockPath) -> Self {
        ItemPath::ImplBlock(path.into())
    }
}

impl TraitForTypeImplBlockPath {
    pub fn data(self, db: &::salsa::Db) -> TraitForTypeImplBlockPathData {
        match self.0.data(db) {
            ItemPathData::ImplBlock(ImplBlockPathData::TraitForTypeImplBlock(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn trai_path(self, db: &::salsa::Db) -> TraitPath {
        self.data(db).trai_path
    }

    pub fn ty_sketch(self, db: &::salsa::Db) -> TypeSketch {
        self.data(db).ty_sketch
    }

    #[inline(never)]
    fn show(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> Result<(), std::fmt::Error> {
        use salsa::DebugWithDb;
        f.debug_struct("TraitForTypeImplBlock")
            .field("data", &self.data(db).debug(db))
            .finish()
    }
}

impl TraitForTypeImplBlockPathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> TraitForTypeImplBlockPath {
        TraitForTypeImplBlockPath(id)
    }

    pub fn module_path(self) -> ModulePath {
        self.module_path
    }

    pub fn trai_path(self) -> TraitPath {
        self.trai_path
    }

    pub fn ty_sketch(self) -> TypeSketch {
        self.ty_sketch
    }

    pub fn disambiguator(self) -> u8 {
        self.disambiguator
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub enum TypeSketch {
    DeriveAny,
    // { ty_kind: Option<TypeKind> }
    Path(TypePath),
}

impl TraitForTypeImplBlockPath {
    pub fn new(
        db: &::salsa::Db,
        registry: &mut ImplBlockRegistry,
        module_path: ModulePath,
        trai_path: TraitPath,
        ty_sketch: TypeSketch,
    ) -> Self {
        TraitForTypeImplBlockPath(ItemPathId::new(
            db,
            ItemPathData::ImplBlock(ImplBlockPathData::TraitForTypeImplBlock(
                TraitForTypeImplBlockPathData {
                    module_path,
                    trai_path,
                    ty_sketch,
                    disambiguator: registry.issue_disambiguitor(
                        module_path,
                        ImplBlockKind::TraitForType {
                            ty_sketch,
                            trai_path,
                        },
                    ),
                },
            )),
        ))
    }
}

impl salsa::DebugWithDb for TraitForTypeImplBlockPath {
    fn debug_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show(f, db)
    }
}

#[derive(Default)]
pub struct ImplBlockRegistry {
    next_disambiguitors: VecPairMap<(ModulePath, ImplBlockKind), u8>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub enum ImplBlockKind {
    Type {
        ty_path: TypePath,
    },
    TraitForType {
        ty_sketch: TypeSketch,
        trai_path: TraitPath,
    },
}

impl ImplBlockRegistry {
    fn issue_disambiguitor(
        &mut self,
        module_path: ModulePath,
        impl_block_kind: ImplBlockKind,
    ) -> u8 {
        let next_disambiguitor = self
            .next_disambiguitors
            .get_value_mut_or_insert_default((module_path, impl_block_kind));
        let new_disambiguitor = *next_disambiguitor;
        *next_disambiguitor += 1;
        new_disambiguitor
    }
}
