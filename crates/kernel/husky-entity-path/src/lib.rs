#![feature(trait_upcasting)]
mod ancestry;
mod db;
mod menu;
#[cfg(test)]
mod tests;
mod utils;

pub use db::*;
pub use menu::*;

use ancestry::*;
use husky_package_path::*;
use husky_toolchain::Toolchain;
use husky_word::{Identifier, WordJar};
use optional::Optioned;
use salsa::DbWithJar;
use utils::*;

#[salsa::jar(db = EntityPathDb)]
pub struct EntityPathJar(EntityPath, apparent_ancestry, entity_path_menu);

#[salsa::interned(jar = EntityPathJar)]
pub struct EntityPath {
    pub data: EntityPathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EntityPathData {
    CrateRoot(CratePath),
    Childpath {
        parent: EntityPath,
        ident: Identifier,
    },
}

impl salsa::DebugWithDb<<EntityPathJar as salsa::jar::Jar<'_>>::DynDb> for EntityPathData {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &<EntityPathJar as salsa::jar::Jar<'_>>::DynDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        #[allow(unused_imports)]
        use ::salsa::debug::helper::Fallback;
        match self {
            EntityPathData::CrateRoot(crate_path) => f
                .debug_tuple("CrateRoot")
                .field(&::salsa::debug::helper::SalsaDebug::<
                    CratePath,
                    <PackagePathJar as salsa::jar::Jar<'_>>::DynDb,
                >::salsa_debug(
                    #[allow(clippy::needless_borrow)]
                    crate_path,
                    db,
                    include_all_fields,
                ))
                .finish(),
            EntityPathData::Childpath { parent, ident } => f
                .debug_struct("CrateRoot")
                .field(
                    "parent",
                    &::salsa::debug::helper::SalsaDebug::<
                        EntityPath,
                        <EntityPathJar as salsa::jar::Jar<'_>>::DynDb,
                    >::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        parent,
                        db,
                        include_all_fields,
                    ),
                )
                .field(
                    "ident",
                    &::salsa::debug::helper::SalsaDebug::<
                        Identifier,
                        <WordJar as salsa::jar::Jar<'_>>::DynDb,
                    >::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        ident,
                        db,
                        include_all_fields,
                    ),
                )
                .finish(),
        }
        // let mut debug_struct = &mut f.debug_struct("EntityPath");
        // debug_struct = debug_struct.field("[salsa id]", &self.0.as_u32());
        // debug_struct = debug_struct.field(
        //     "data",
        //     &::salsa::debug::helper::SalsaDebug::<
        //         EntityPathData,
        //         <EntityPathJar as salsa::jar::Jar<'_>>::DynDb,
        //     >::salsa_debug(
        //         #[allow(clippy::needless_borrow)]
        //         &self.data(_db),
        //         _db,
        //         _include_all_fields,
        //     ),
        // );
        // debug_struct.finish()
    }
}

impl EntityPath {
    pub fn root(ident: Identifier) -> Self {
        todo!()
        // Self {
        //     ident,
        //     variant: EntityPathVariant::Crate {
        //         package: PackagePathData::Builtin {
        //             toolchain: Toolchain::new_ad_hoc(),
        //         },
        //         kind: CratePathKind::Library,
        //     },
        // }
    }

    pub fn new_crate_root(
        db: &dyn EntityPathDb,
        package_path: PackagePath,
        crate_kind: CrateKind,
    ) -> Self {
        db.it_entity_path(EntityPathData::CrateRoot(CratePath::new(
            db,
            package_path,
            crate_kind,
        )))
    }

    #[inline(always)]
    pub fn opt_parent(&self) -> Option<EntityPath> {
        todo!()
        // match self.variant {
        //     EntityPathVariant::Crate { .. } => None,
        //     EntityPathVariant::Childpath { parent, .. } => Some(parent),
        // }
    }

    pub(crate) fn child(self, db: &dyn EntityPathDb, ident: &str) -> Option<EntityPath> {
        db.it_child_entity_path(self, ident)
    }

    pub fn show(self, db: &dyn EntityPathDb) -> String {
        match self.data(db) {
            EntityPathData::CrateRoot(crate_path) => {
                format!("crate({:?})", crate_path.package_path(db).data(db))
            }
            EntityPathData::Childpath { parent, ident } => {
                format!("{}::{}", parent.show(db), db.dt_ident(ident))
            }
        }
    }
}
