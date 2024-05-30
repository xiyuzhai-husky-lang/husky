use super::*;
use husky_dec_signature::signature::major_item::ty::{
    props_struct::{PropsStructDecTemplate, PropsStructFieldDecTemplate},
    tuple_struct::{TupleStructDecTemplate, TupleStructFieldDecTemplate},
    TypeDecTemplate,
};
use husky_entity_kind::ritchie::RitchieItemKind;
use husky_entity_path::path::major_item::ty::TypePath;

#[salsa::tracked(jar = DecTypeJar)]
pub fn ty_instance_constructor_path_declarative_ty(
    db: &::salsa::Db,
    path: TypePath,
) -> DeclarativeTypeResult<DecTerm> {
    let _declarative_term_menu = db.dec_term_menu(path.toolchain(db)).unwrap();
    let signature = match path.dec_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    let Ok(variances) = ty_path_variances(db, path) else {
        todo!()
    };
    match signature {
        TypeDecTemplate::Enum(_) => Err(OriginalDeclarativeTypeError::EnumNoConstructor)?,
        TypeDecTemplate::PropsStruct(signature) => {
            Ok(props_struct_ty_instance_constructor_path_declarative_ty(
                db, path, variances, signature,
            )?)
        }
        TypeDecTemplate::UnitStruct(_) => todo!(),
        TypeDecTemplate::TupleStruct(signature) => Ok(
            tuple_struct_ty_constructor_path_declarative_ty(db, path, variances, signature)?,
        ),
        TypeDecTemplate::Inductive(_) => {
            Err(OriginalDeclarativeTypeError::InductiveTypeHasNoConstructor)?
        }
        TypeDecTemplate::Structure(_) => todo!(),
        TypeDecTemplate::Extern(_) => {
            Err(OriginalDeclarativeTypeError::ExternTypeHasNoConstructor)?
        }
        TypeDecTemplate::Union(_) => todo!(),
    }
}

fn props_struct_ty_instance_constructor_path_declarative_ty(
    db: &::salsa::Db,
    path: TypePath,
    variances: &[Variance],
    tmpl: PropsStructDecTemplate,
) -> DeclarativeTypeResult<DecTerm> {
    let template_parameters = &tmpl.template_parameters(db);
    let self_ty = tmpl.self_ty(db);
    let parameter_tys = tmpl
        .fields(db)
        .iter()
        .copied()
        .filter_map(PropsStructFieldDecTemplate::into_ritchie_parameter_contracted_ty)
        .collect();
    let instance_constructor_ty =
        DecRitchie::new(db, RitchieItemKind::Fn.into(), parameter_tys, self_ty);
    curry_from_template_parameters(
        db,
        path.toolchain(db),
        CurryKind::Implicit,
        variances,
        template_parameters,
        instance_constructor_ty,
    )
}

fn tuple_struct_ty_constructor_path_declarative_ty(
    db: &::salsa::Db,
    path: TypePath,
    variances: &[Variance],
    signature: TupleStructDecTemplate,
) -> DeclarativeTypeResult<DecTerm> {
    let template_parameters = &signature.template_parameters(db);
    let self_ty = signature.self_ty(db);
    let parameter_tys = signature
        .fields(db)
        .iter()
        .copied()
        .map(TupleStructFieldDecTemplate::into_ritchie_parameter_contracted_ty)
        .collect();
    let constructor_ty = DecRitchie::new(db, RitchieItemKind::Fn.into(), parameter_tys, self_ty);
    curry_from_template_parameters(
        db,
        path.toolchain(db),
        CurryKind::Implicit,
        variances,
        template_parameters,
        constructor_ty,
    )
}
