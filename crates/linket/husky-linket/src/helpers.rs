mod vm_only;

use crate::*;
use husky_entity_path::{path::ItemPath, region::RegionPath};
use husky_place::PlaceRegistry;
use template_argument::{qual::LinQual, ty::LinType, LinTemplateArgument};

impl Linket {
    pub fn path_and_instantiation_for_definition<'db>(
        self,
        db: &'db salsa::Db,
    ) -> Option<(ItemPath, &LinInstantiation)> {
        Some(match *self.data(db) {
            LinketData::MajorRitchie {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinketData::MajorStaticVar {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinketData::MajorVal {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinketData::Memo {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinketData::MethodRitchie {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinketData::AssocRitchie {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinketData::UnveilAssocRitchie {
                path,
                ref instantiation,
            } => (path.into(), instantiation),
            LinketData::StructConstructor { .. }
            | LinketData::EnumVariantConstructor { .. }
            | LinketData::EnumVariantDiscriminator { .. }
            | LinketData::StructDestructor { .. }
            | LinketData::EnumVariantDestructor { .. }
            | LinketData::StructField { .. }
            | LinketData::EnumVariantField { .. }
            | LinketData::Index
            | LinketData::VecConstructor { .. }
            | LinketData::TypeDefault { .. }
            | LinketData::EnumUnitToJsonValue { .. } => return None,
        })
    }

    pub fn place_registry(self, db: &::salsa::Db) -> Option<&PlaceRegistry> {
        use husky_sem_expr::helpers::path::sem_expr_region_from_region_path;

        let (path, _) = self.path_and_instantiation_for_definition(db)?;
        Some(
            sem_expr_region_from_region_path(RegionPath::ItemDefn(path), db)?
                .data(db)
                .place_registry(),
        )
    }
}
