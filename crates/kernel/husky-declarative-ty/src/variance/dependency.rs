use husky_vfs::CratePath;
use vec_like::VecSet;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VarianceId {
    path: ItemPath,
    idx: u8,
}

impl VarianceId {
    pub(super) fn new(
        db: &::salsa::Db,
        crate_path: CratePath,
        path: ItemPath,
        idx: u8,
    ) -> Option<Self> {
        (crate_path == path.crate_path(db)).then_some(Self { path, idx })
    }
}

pub(crate) fn item_variance_crate_dependencies(
    db: &::salsa::Db,
    id: VarianceId,
) -> VarianceResultRef<&[VarianceId]> {
    let _declarative_term_menu = db
        .declarative_term_menu(id.path.toolchain(db))
        .as_ref()
        .unwrap();
    match id.path {
        ItemPath::Submodule(_, _) => todo!(),
        ItemPath::MajorItem(path) => match path {
            MajorItemPath::Type(path) => {
                declarative_ty_item_variance_crate_dependencies(db, path, id.idx)
                    .as_ref()
                    .map(|t| t.as_ref())
            }
            MajorItemPath::Trait(path) => trai_item_variance_crate_dependencies(db, path, id.idx)
                .as_ref()
                .map(|t| t.as_ref()),
            MajorItemPath::Fugitive(path) => {
                todo!()
                // form_item_variance_crate_dependencies(db, path, id.idx)
                //     .as_ref()
                //     .map(|t| t.as_ref())
            }
        },
        ItemPath::AssociatedItem(_) => todo!(),
        ItemPath::TypeVariant(_, _) => todo!(),
        ItemPath::ImplBlock(_) => todo!(),
        ItemPath::Attr(_, _) => todo!(),
    }
}

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub(crate) fn declarative_ty_item_variance_crate_dependencies(
    db: &::salsa::Db,
    path: TypePath,
    _idx: u8,
) -> VarianceResult<VecSet<VarianceId>> {
    let _declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    let signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    match signature {
        TypeDeclarativeSignatureTemplate::Enum(_) => todo!(),
        TypeDeclarativeSignatureTemplate::PropsStruct(_) => todo!(),
        TypeDeclarativeSignatureTemplate::UnitStruct(_) => todo!(),
        TypeDeclarativeSignatureTemplate::TupleStruct(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Record(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Inductive(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Structure(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Extern(_) => (),
        TypeDeclarativeSignatureTemplate::Union(_) => todo!(),
    }
    todo!()
}

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub(crate) fn trai_item_variance_crate_dependencies(
    _db: &::salsa::Db,
    _path: TraitPath,
    _idx: u8,
) -> VarianceResult<VecSet<VarianceId>> {
    todo!()
}

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub(crate) fn form_item_variance_crate_dependencies(
    db: &::salsa::Db,
    path: FugitivePath,
    _idx: u8,
) -> VarianceResult<VecSet<VarianceId>> {
    let _signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    todo!()
}
