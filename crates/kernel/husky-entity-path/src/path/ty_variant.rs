use salsa::Db;

use crate::*;

#[salsa::debug_with_db]
#[salsa::as_id(jar = EntityPathJar)]
#[salsa::deref_id]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TypeVariantPath(ItemPathId);

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TypeVariantPathData {
    pub parent_ty_path: TypePath,
    pub ident: Ident,
}

impl TypeVariantPath {
    pub fn new(parent_ty_path: TypePath, ident: Ident, db: &::salsa::Db) -> Self {
        Self(ItemPathId::new(
            db,
            ItemPathData::TypeVariant(TypeVariantPathData {
                parent_ty_path,
                ident,
            }),
        ))
    }

    pub fn data(self, db: &::salsa::Db) -> TypeVariantPathData {
        match self.0.data(db) {
            ItemPathData::TypeVariant(data) => data,
            _ => unreachable!(),
        }
    }

    pub fn parent_ty_path(self, db: &::salsa::Db) -> TypePath {
        self.data(db).parent_ty_path
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.data(db).ident
    }

    #[inline(never)]
    pub fn show_aux(self, f: &mut std::fmt::Formatter<'_>, db: &::salsa::Db) -> std::fmt::Result {
        self.data(db).show_aux(f, db)
    }
}

impl TypeVariantPathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> TypeVariantPath {
        TypeVariantPath(id)
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        self.parent_ty_path.toolchain(db)
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.parent_ty_path.module_path(db)
    }

    #[inline(never)]
    pub fn show_aux(self, f: &mut std::fmt::Formatter<'_>, db: &::salsa::Db) -> std::fmt::Result {
        self.parent_ty_path.show_aux(f, db)?;
        f.write_str("::")?;
        f.write_str(self.ident.data(db))
    }
}

impl salsa::DisplayWithDb for TypeVariantPath {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_aux(f, db)
    }
}
