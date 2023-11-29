use super::*;
use crate::linkage::{Linkage, LinkageData};

#[salsa::tracked(jar = LinkageJar)]
fn linkage_path_dependencies(db: &::salsa::Db, linkage_path: Linkage) {
    match linkage_path.data(db) {
        LinkageData::Coersion {} => todo!(),
        LinkageData::PathLeading {
            path,
            ref instantiation,
        } => todo!(),
        LinkageData::PropsStructField => todo!(),
        LinkageData::MemoizedField => todo!(),
        LinkageData::Index => todo!(),
        LinkageData::Method => todo!(),
    }
}
