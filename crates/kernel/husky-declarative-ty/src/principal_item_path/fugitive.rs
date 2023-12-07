use husky_vfs::Toolchain;
use smallvec::ToSmallVec;

use super::*;

// #[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn fugitive_path_declarative_ty(
    db: &::salsa::Db,
    path: FugitivePath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    let Ok(variances) = form_item_variances(db, path) else {
        todo!()
    };
    // ad hoc
    let variances = &variances;
    let declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    match signature {
        FugitiveDeclarativeSignatureTemplate::FunctionFn(signature) => {
            fn_path_declarative_ty(db, path.toolchain(db), variances, signature)
        }
        FugitiveDeclarativeSignatureTemplate::FunctionGn(signature) => {
            gn_path_declarative_ty(db, path.toolchain(db), variances, signature)
        }
        FugitiveDeclarativeSignatureTemplate::Val(signature) => {
            val_path_declarative_ty(db, signature, declarative_term_menu)
        }
        FugitiveDeclarativeSignatureTemplate::TypeAlias(_) => todo!(),
    }
}

pub(crate) fn fn_path_declarative_ty(
    db: &::salsa::Db,
    toolchain: Toolchain,
    variances: &[Variance],
    signature: FnFugitiveDeclarativeSignatureTemplate,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    let parenate_parameters = signature.parenate_parameters(db).data().to_smallvec();
    let return_declarative_ty = signature.return_ty(db);
    curry_from_template_parameters(
        db,
        toolchain,
        CurryKind::Implicit,
        variances,
        signature.template_parameters(db),
        DeclarativeTermRitchie::new(
            db,
            RitchieTypeKind::Fn.into(),
            parenate_parameters,
            return_declarative_ty,
        ),
    )
}

pub(crate) fn gn_path_declarative_ty(
    db: &::salsa::Db,
    toolchain: Toolchain,
    variances: &[Variance],
    signature: GnFugitiveDeclarativeSignatureTemplate,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    use smallvec::ToSmallVec;
    let param_declarative_tys = signature.parenate_parameters(db).data().to_smallvec();
    let return_declarative_ty = signature.return_ty(db);
    curry_from_template_parameters(
        db,
        toolchain,
        CurryKind::Implicit,
        variances,
        signature.template_parameters(db),
        DeclarativeTermRitchie::new(
            db,
            RitchieTypeKind::Fn.into(),
            param_declarative_tys,
            return_declarative_ty,
        ),
    )
}

pub(crate) fn val_path_declarative_ty(
    db: &::salsa::Db,
    signature: ValFugitiveDeclarativeSignatureTemplate,
    _declarative_term_menu: &DeclarativeTermMenu,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    Ok(signature.initialization_ty(db).leashed_ty(db))
}
