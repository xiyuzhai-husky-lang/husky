use husky_dec_signature::signature::{major_item::ty::TypeDecTemplate, HasDecTemplate};
use husky_entity_path::path::{
    major_item::{form::MajorFormPath, trai::TraitPath, ty::TypePath, MajorItemPath},
    ItemPath,
};
use husky_vfs::path::crate_path::CratePath;
use vec_like::VecSet;

use super::*;

/// the path that represent the variance for a specific template argument of a path
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VariancePath {
    path: ItemPath,
    idx: u8,
}

impl VariancePath {
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
    variance_path: VariancePath,
) -> VarianceResultRef<&[VariancePath]> {
    let _dec_term_menu = db
        .dec_term_menu(variance_path.path.toolchain(db))
        .as_ref()
        .unwrap();
    match variance_path.path {
        ItemPath::Submodule(_, _) => todo!(),
        ItemPath::MajorItem(path) => match path {
            MajorItemPath::Type(path) => {
                declarative_ty_item_variance_crate_dependencies(db, path, variance_path.idx)
                    .as_ref()
                    .map(|t| t.as_ref())
            }
            MajorItemPath::Trait(path) => {
                trai_item_variance_crate_dependencies(db, path, variance_path.idx)
                    .as_ref()
                    .map(|t| t.as_ref())
            }
            MajorItemPath::Form(_path) => {
                todo!()
                // form_item_variance_crate_dependencies(db, path, id.idx)
                //     .as_ref()
                //     .map(|t| t.as_ref())
            }
        },
        ItemPath::AssocItem(_) => todo!(),
        ItemPath::TypeVariant(_, _) => todo!(),
        ItemPath::ImplBlock(_) => todo!(),
        ItemPath::Attr(_, _) => todo!(),
        ItemPath::Chunk(_, _) => todo!(),
    }
}

#[salsa::tracked(jar = DecTypeJar, return_ref)]
pub(crate) fn declarative_ty_item_variance_crate_dependencies(
    db: &::salsa::Db,
    path: TypePath,
    _idx: u8,
) -> VarianceResult<VecSet<VariancePath>> {
    let _dec_term_menu = db.dec_term_menu(path.toolchain(db)).unwrap();
    let signature = match path.dec_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    match signature {
        TypeDecTemplate::Enum(_) => todo!(),
        TypeDecTemplate::PropsStruct(_) => todo!(),
        TypeDecTemplate::UnitStruct(_) => todo!(),
        TypeDecTemplate::TupleStruct(_) => todo!(),
        TypeDecTemplate::Inductive(_) => todo!(),
        TypeDecTemplate::Structure(_) => todo!(),
        TypeDecTemplate::Extern(_) => (),
        TypeDecTemplate::Union(_) => todo!(),
    }
    todo!()
}

#[salsa::tracked(jar = DecTypeJar, return_ref)]
pub(crate) fn trai_item_variance_crate_dependencies(
    _db: &::salsa::Db,
    _path: TraitPath,
    _idx: u8,
) -> VarianceResult<VecSet<VariancePath>> {
    todo!()
}

#[salsa::tracked(jar = DecTypeJar, return_ref)]
pub(crate) fn form_item_variance_crate_dependencies(
    db: &::salsa::Db,
    path: MajorFormPath,
    _idx: u8,
) -> VarianceResult<VecSet<VariancePath>> {
    let _signature = match path.dec_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    todo!()
}
