use crate::*;
use husky_entity_path::ItemPath;

impl Linkage {
    pub fn path_and_instantiation<'db>(
        self,
        db: &'db salsa::Db,
    ) -> Option<(ItemPath, &LinInstantiation)> {
        Some(match *self.data(db) {
            LinkageData::MajorRitchieEager {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinkageData::MajorRitchieLazy {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinkageData::MajorVal {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinkageData::MemoizedField {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinkageData::MethodRitchie {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinkageData::AssocRitchie {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinkageData::UnveilAssocFn {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinkageData::TypeConstructor {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinkageData::TypeVariantConstructor {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinkageData::StructField { self_ty, field } => todo!(),
            LinkageData::Index => return None,
            LinkageData::VecConstructor { element_ty } => todo!(),
            LinkageData::TypeDefault { ty } => todo!(),
            LinkageData::EnumU8ToJsonValue { ty_path } => todo!(),
        })
    }
}
