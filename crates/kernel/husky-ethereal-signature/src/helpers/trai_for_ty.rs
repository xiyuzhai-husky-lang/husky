use super::*;
use either::*;
use husky_entity_syn_tree::{
    helpers::trai_side_derive_any_trai_for_ty_impl_block_paths_map,
    helpers::trai_side_path_leading_trai_for_ty_impl_block_paths_map,
    helpers::ty_side_trai_for_ty_impl_block_paths_map,
};
use husky_term_prelude::{RitchieKind, RitchieTypeKind};

pub struct TraitForTypeImplBlockEtherealSignatureTemplates<'a> {
    trai_side_derive_any: &'a [TraitForTypeImplBlockEtherealSignatureTemplate],
    trai_side_path_leading: &'a [TraitForTypeImplBlockEtherealSignatureTemplate],
    ty_side: &'a [TraitForTypeImplBlockEtherealSignatureTemplate],
}

impl<'a> TraitForTypeImplBlockEtherealSignatureTemplates<'a> {
    pub fn iter(self) -> impl Iterator<Item = TraitForTypeImplBlockEtherealSignatureTemplate> + 'a {
        (self
            .trai_side_derive_any
            .iter()
            .chain(self.trai_side_path_leading.iter())
            .chain(self.ty_side.iter()))
        .copied()
    }
}

pub fn trai_path_for_ty_path_impl_block_ethereal_signature_templates<'a>(
    db: &'a ::salsa::Db,
    trai_path: TraitPath,
    ty_path: TypePath,
) -> EtherealSignatureResult<TraitForTypeImplBlockEtherealSignatureTemplates<'a>> {
    let does_ty_path_derive_trai_path = ty_path
        .derive_attr_ethereal_signature_templates(db, trai_path)?
        .is_some();
    Ok(TraitForTypeImplBlockEtherealSignatureTemplates {
        trai_side_derive_any: if does_ty_path_derive_trai_path {
            trai_side_derive_any_ethereal_signature_templates(db, trai_path).as_ref()?
        } else {
            &[]
        },
        // todo: ty_kind
        trai_side_path_leading: trai_side_path_leading_ethereal_signature_templates(
            db, trai_path, ty_path,
        )?
        .unwrap_or(&[]),
        ty_side: ty_side_trai_for_ty_impl_block_signature_templates(db, trai_path, ty_path)
            .into_result_option()?
            .unwrap_or(&[]),
    })
}

pub fn trai_path_for_ty_term_impl_block_ethereal_signature_builders<'a>(
    db: &'a ::salsa::Db,
    trai_path: TraitPath,
    ty_term: EtherealTerm,
) -> EtherealSignatureResult<SmallVec<[TraitForTypeImplBlockEtherealSignatureBuilder; 2]>> {
    let application_expansion = ty_term.application_expansion(db);
    let arguments = application_expansion.arguments(db);
    let TermFunctionReduced::TypeOntology(ty_path) = application_expansion.function() else {
        unreachable!()
    };
    let mut builders: SmallVec<[TraitForTypeImplBlockEtherealSignatureBuilder; 2]> = smallvec![];
    for template in
        trai_path_for_ty_path_impl_block_ethereal_signature_templates(db, trai_path, ty_path)?
            .iter()
    {
        match template.instantiate_ty(db, arguments, ty_term) {
            JustOk(builder) => builders.push(builder),
            JustErr(_) => todo!(),
            Nothing => todo!(),
        }
    }
    Ok(builders)
}

// todo: check argument ty trai satisfaction
pub fn trai_path_for_ty_term_impl_block_ethereal_signature_builder_exists<'a>(
    db: &'a ::salsa::Db,
    trai_path: TraitPath,
    ty_term: EtherealTerm,
) -> EtherealSignatureResult<bool> {
    match ty_term {
        EtherealTerm::Symbol(_) => return Ok(false), // ad hoc
        EtherealTerm::Rune(_) => todo!(),
        EtherealTerm::Ritchie(ritchie) => match ritchie.ritchie_kind(db) {
            RitchieKind::Type(ritchie_ty_kind) => match ritchie_ty_kind {
                RitchieTypeKind::Fn => match trai_path.refine(db) {
                    Left(prelude_trai_path) => match prelude_trai_path {
                        PreludeTraitPath::Add => todo!(),
                        PreludeTraitPath::AddAssign => todo!(),
                        PreludeTraitPath::BitAnd => todo!(),
                        PreludeTraitPath::BitAndAssign => todo!(),
                        PreludeTraitPath::BitOr => todo!(),
                        PreludeTraitPath::BitOrAssign => todo!(),
                        PreludeTraitPath::BitXor => todo!(),
                        PreludeTraitPath::BitXorAssign => todo!(),
                        PreludeTraitPath::Div => todo!(),
                        PreludeTraitPath::DivAssign => todo!(),
                        PreludeTraitPath::IntIndex => todo!(),
                        PreludeTraitPath::Mul => todo!(),
                        PreludeTraitPath::MulAssign => todo!(),
                        PreludeTraitPath::Neg => todo!(),
                        PreludeTraitPath::Not => todo!(),
                        PreludeTraitPath::Unveil => todo!(),
                        PreludeTraitPath::Clone => return Ok(true),
                        PreludeTraitPath::Copy => return Ok(true),
                        PreludeTraitPath::Default => return Ok(false),
                        PreludeTraitPath::Visualize => todo!(),
                    },
                    Right(_) => todo!(),
                },
                RitchieTypeKind::Gn => todo!(),
            },
            RitchieKind::Trait(_) => todo!(),
        },
        EtherealTerm::Subitem(_) => todo!(),
        EtherealTerm::AsTraitSubitem(_) => todo!(),
        _ => (),
    }
    let application_expansion = ty_term.application_expansion(db);
    let arguments = application_expansion.arguments(db);
    let TermFunctionReduced::TypeOntology(ty_path) = application_expansion.function() else {
        unreachable!()
    };
    for template in
        trai_path_for_ty_path_impl_block_ethereal_signature_templates(db, trai_path, ty_path)?
            .iter()
    {
        match template.instantiate_ty(db, arguments, ty_term) {
            JustOk(builder) => return Ok(true),
            JustErr(e) => return Err(e),
            Nothing => continue,
        }
    }
    Ok(false)
}

// todo: cache this
pub fn is_ty_term_always_copyable(
    ty_term: EtherealTerm,
    db: &::salsa::Db,
) -> EtherealSignatureResult<Option<bool>> {
    let Some(item_path_menu) = ty_term.item_path_menu(db) else {
        return Ok(None);
    };
    let copy_trai = item_path_menu.copy_trai_path();
    trai_path_for_ty_term_impl_block_ethereal_signature_builder_exists(db, copy_trai, ty_term)
        .map(Some)
}

// trait side

#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
fn trai_side_derive_any_ethereal_signature_templates(
    db: &::salsa::Db,
    trai_path: TraitPath,
) -> EtherealSignatureResult<SmallVec<[TraitForTypeImplBlockEtherealSignatureTemplate; 2]>> {
    trai_side_derive_any_trai_for_ty_impl_block_paths_map(db, trai_path)
        .iter()
        .map(|path| path.ethereal_signature_template(db))
        .collect()
}

fn trai_side_path_leading_ethereal_signature_templates(
    db: &::salsa::Db,
    trai_path: TraitPath,
    ty_path: TypePath,
) -> EtherealSignatureResult<Option<&[TraitForTypeImplBlockEtherealSignatureTemplate]>> {
    match trai_side_path_leading_trai_for_ty_impl_block_ethereal_signature_templates_map(
        db, trai_path,
    )
    .get_value(ty_path)
    {
        Some(Ok(v)) => Ok(Some(v)),
        Some(Err(e)) => Err(*e),
        None => Ok(None),
    }
}

#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
fn trai_side_path_leading_trai_for_ty_impl_block_ethereal_signature_templates_map(
    db: &::salsa::Db,
    trai_path: TraitPath,
) -> SmallVecPairMap<
    TypePath,
    EtherealSignatureResult<SmallVec<[TraitForTypeImplBlockEtherealSignatureTemplate; 2]>>,
    8,
> {
    trai_side_path_leading_trai_for_ty_impl_block_paths_map(db, trai_path).map_collect(|paths| {
        paths
            .iter()
            .map(|path| path.ethereal_signature_template(db))
            .collect()
    })
}

// type side

pub fn ty_side_trai_for_ty_impl_block_signature_templates(
    db: &::salsa::Db,
    trai_path: TraitPath,
    ty_path: TypePath,
) -> EtherealSignatureMaybeResult<&[TraitForTypeImplBlockEtherealSignatureTemplate]> {
    match ty_side_impl_block_signature_templates_map(db, ty_path).get_value(trai_path) {
        Some(result) => match result {
            Ok(templates) => JustOk(templates),
            Err(e) => JustErr(*e),
        },
        None => Nothing,
    }
}

#[salsa::tracked(jar = EtherealSignatureJar, return_ref)]
fn ty_side_impl_block_signature_templates_map(
    db: &::salsa::Db,
    ty_path: TypePath,
) -> SmallVecPairMap<TraitPath, EtherealSignatureResult<TraitForTypeImplBlockSignatureTemplates>, 2>
{
    ty_side_trai_for_ty_impl_block_paths_map(db, ty_path).map_collect(|paths| {
        paths
            .iter()
            .map(|path| path.ethereal_signature_template(db))
            .collect()
    })
}
