use smallvec::ToSmallVec;

use super::*;

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn form_path_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    path: FugitivePath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    let Ok(variances) = form_item_variances(db, path) else {
        todo!()
    };
    let declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    match signature {
        FugitiveDeclarativeSignatureTemplate::Fn(signature) => {
            fn_path_declarative_ty(db, variances, signature)
        }
        FugitiveDeclarativeSignatureTemplate::Gn(signature) => {
            gn_path_declarative_ty(db, variances, signature)
        }
        FugitiveDeclarativeSignatureTemplate::Val(signature) => {
            val_path_declarative_ty(db, signature, declarative_term_menu)
        }
        FugitiveDeclarativeSignatureTemplate::AliasType(_) => todo!(),
    }
}

pub(crate) fn fn_path_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    variances: &[Variance],
    signature: FnDeclarativeSignatureTemplate,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let parenic_parameters = signature.parenic_parameters(db).data().to_smallvec();
    let return_declarative_ty = signature.return_ty(db);
    curry_from_template_parameters(
        db,
        CurryKind::Implicit,
        variances,
        signature.template_parameters(db),
        DeclarativeTermRitchie::new(
            db,
            RitchieKind::FnType,
            parenic_parameters,
            return_declarative_ty,
        ),
    )
}

pub(crate) fn gn_path_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    variances: &[Variance],
    signature: GnDeclarativeSignatureTemplate,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    use smallvec::ToSmallVec;
    let param_declarative_tys = signature.parenic_parameters(db).data().to_smallvec();
    let return_declarative_ty = signature.return_ty(db);
    curry_from_template_parameters(
        db,
        CurryKind::Implicit,
        variances,
        signature.template_parameters(db),
        DeclarativeTermRitchie::new(
            db,
            RitchieKind::FnType,
            param_declarative_tys,
            return_declarative_ty,
        ),
    )
}

pub(crate) fn val_path_declarative_ty(
    db: &dyn DeclarativeTypeDb,
    signature: ValDeclarativeSignatureTemplate,
    _declarative_term_menu: &DeclarativeTermMenu,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    Ok(signature.initialization_ty(db).leashed_ty(db))
}
