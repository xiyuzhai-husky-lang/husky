use crate::*;
use husky_entity_path::ItemPath;

impl Linkage {
    pub fn path_and_instantiation_for_definition<'db>(
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
            LinkageData::StructConstructor { .. }
            | LinkageData::EnumVariantConstructor { .. }
            | LinkageData::EnumVariantDiscriminator { .. }
            | LinkageData::StructDestructor { .. }
            | LinkageData::EnumVariantDestructor { .. }
            | LinkageData::StructField { .. }
            | LinkageData::EnumVariantField { .. }
            | LinkageData::Index
            | LinkageData::VecConstructor { .. }
            | LinkageData::TypeDefault { .. }
            | LinkageData::EnumU8ToJsonValue { .. } => return None,
        })
    }
}
