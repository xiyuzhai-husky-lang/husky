use husky_vfs::CratePath;
use vec_like::VecSet;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VarianceId {
    path: EntityPath,
    idx: u8,
}

impl VarianceId {
    pub(super) fn new(
        db: &dyn RawTypeDb,
        crate_path: CratePath,
        path: EntityPath,
        idx: u8,
    ) -> Option<Self> {
        (crate_path == path.crate_path(db)).then_some(Self { path, idx })
    }
}

pub(crate) fn entity_variance_crate_dependencies(
    db: &dyn RawTypeDb,
    id: VarianceId,
) -> VarianceResultRef<&[VarianceId]> {
    let _raw_term_menu = db.raw_term_menu(id.path.toolchain(db)).as_ref().unwrap();
    match id.path {
        EntityPath::Module(_) => todo!(),
        EntityPath::ModuleItem(path) => match path {
            ModuleItemPath::Type(path) => {
                raw_ty_entity_variance_crate_dependencies(db, path, id.idx)
                    .as_ref()
                    .map(|t| t.as_ref())
            }
            ModuleItemPath::Trait(path) => {
                trai_entity_variance_crate_dependencies(db, path, id.idx)
                    .as_ref()
                    .map(|t| t.as_ref())
            }
            ModuleItemPath::Form(path) => form_entity_variance_crate_dependencies(db, path, id.idx)
                .as_ref()
                .map(|t| t.as_ref()),
        },
        EntityPath::AssociatedItem(_) => todo!(),
        EntityPath::Variant(_) => todo!(),
    }
}

#[salsa::tracked(jar = RawTypeJar, return_ref)]
pub(crate) fn raw_ty_entity_variance_crate_dependencies(
    db: &dyn RawTypeDb,
    path: TypePath,
    _idx: u8,
) -> VarianceResult<VecSet<VarianceId>> {
    let _raw_term_menu = db.raw_term_menu(path.toolchain(db)).unwrap();
    let decl = match path.decl(db) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedVarianceError::DeclError.into()),
    };
    let signature = match db.ty_signature_from_decl(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    match signature {
        TypeSignature::Enum(_) => todo!(),
        TypeSignature::RegularStruct(_) => todo!(),
        TypeSignature::UnitStruct(_) => todo!(),
        TypeSignature::TupleStruct(_) => todo!(),
        TypeSignature::Record(_) => todo!(),
        TypeSignature::Inductive(_) => todo!(),
        TypeSignature::Structure(_) => todo!(),
        TypeSignature::Foreign(_) => (),
        TypeSignature::Union(_) => todo!(),
    }
    todo!()
}

#[salsa::tracked(jar = RawTypeJar, return_ref)]
pub(crate) fn trai_entity_variance_crate_dependencies(
    _db: &dyn RawTypeDb,
    _path: TraitPath,
    _idx: u8,
) -> VarianceResult<VecSet<VarianceId>> {
    todo!()
}

#[salsa::tracked(jar = RawTypeJar, return_ref)]
pub(crate) fn form_entity_variance_crate_dependencies(
    db: &dyn RawTypeDb,
    path: FormPath,
    _idx: u8,
) -> VarianceResult<VecSet<VarianceId>> {
    let decl = match db.form_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedVarianceError::DeclError.into()),
    };
    let _signature = match db.form_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    todo!()
}
