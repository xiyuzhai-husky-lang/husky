use super::*;
use crate::path::{LinkagePath, LinkagePathData};

#[salsa::tracked(jar = LinkagePathJar)]
fn linkage_path_dependencies(db: &dyn LinkagePathDb, linkage_path: LinkagePath) {
    match linkage_path.data(db) {
        LinkagePathData::Coersion {} => todo!(),
        LinkagePathData::Item {
            path,
            template_arguments,
        } => todo!(),
        LinkagePathData::Field => todo!(),
        LinkagePathData::Index => todo!(),
        LinkagePathData::Method => todo!(),
    }
}
