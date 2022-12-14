use husky_package_path::PackagePathData;

use crate::*;

#[salsa::tracked(jar = EntityPathJar)]
pub(crate) fn entity_package(db: &dyn EntityPathDb, entity: EntityPath) -> PackagePath {
    match db.dt_entity_path(entity) {
        EntityPathData::Crate { package, kind } => package,
        EntityPathData::Childpath { parent, ident } => db.entity_package(parent),
    }
}

#[salsa::tracked(jar = EntityPathJar)]
pub(crate) fn is_builtin_entity(db: &dyn EntityPathDb, entity: EntityPath) -> bool {
    let package = db.entity_package(entity);
    match db.package_path_data(package) {
        PackagePathData::Builtin { ident, toolchain } => true,
        PackagePathData::Global { version } => todo!(),
        PackagePathData::Local(_) => false,
        PackagePathData::Git(_) => todo!(),
    }
}
