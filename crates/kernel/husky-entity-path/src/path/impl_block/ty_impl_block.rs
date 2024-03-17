use super::*;

#[salsa::derive_debug_with_db]
#[salsa::as_id(jar = EntityPathJar)]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeImplBlockPath(ItemPathId);

#[salsa::derive_debug_with_db]
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
        self.disambiguator.fmt(f)?;
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
