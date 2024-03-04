use husky_entity_kind::ritchie::RitchieItemKind;
use husky_vfs::Toolchain;
use smallvec::ToSmallVec;

use super::*;

#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn fugitive_path_declarative_ty(
    db: &::salsa::Db,
    path: FugitivePath,
) -> DeclarativeTypeResult<DecTerm> {
    let signature = match path.dec_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    let Ok(variances) = fugitive_path_variances(db, path) else {
        todo!()
    };
    // ad hoc
    let variances = &variances;
    let dec_term_menu = db.dec_term_menu(path.toolchain(db)).unwrap();
    match signature {
        FugitiveDecTemplate::Fn(signature) => {
            fn_path_declarative_ty(db, path.toolchain(db), variances, signature)
        }
        FugitiveDecTemplate::Gn(signature) => {
            gn_path_declarative_ty(db, path.toolchain(db), variances, signature)
        }
        FugitiveDecTemplate::Ki(signature) => val_path_declarative_ty(db, signature, dec_term_menu),
        FugitiveDecTemplate::TypeAlias(_) => todo!(),
    }
}

pub(crate) fn fn_path_declarative_ty(
    db: &::salsa::Db,
    toolchain: Toolchain,
    variances: &[Variance],
    signature: MajorFnDecTemplate,
) -> DeclarativeTypeResult<DecTerm> {
    let parenate_parameters = signature.parenate_parameters(db).data().to_smallvec();
    let return_declarative_ty = signature.return_ty(db);
    curry_from_template_parameters(
        db,
        toolchain,
        CurryKind::Implicit,
        variances,
        signature.template_parameters(db),
        DecRitchie::new(
            db,
            RitchieItemKind::Fn.into(),
            parenate_parameters,
            return_declarative_ty,
        ),
    )
}

pub(crate) fn gn_path_declarative_ty(
    db: &::salsa::Db,
    toolchain: Toolchain,
    variances: &[Variance],
    signature: MajorGnDecTemplate,
) -> DeclarativeTypeResult<DecTerm> {
    let param_declarative_tys = signature.parenate_parameters(db).data().to_smallvec();
    let return_declarative_ty = signature.return_ty(db);
    curry_from_template_parameters(
        db,
        toolchain,
        CurryKind::Implicit,
        variances,
        signature.template_parameters(db),
        DecRitchie::new(
            db,
            RitchieItemKind::Fn.into(),
            param_declarative_tys,
            return_declarative_ty,
        ),
    )
}

pub(crate) fn val_path_declarative_ty(
    db: &::salsa::Db,
    signature: MajorValDecTemplate,
    _declarative_term_menu: &DecTermMenu,
) -> DeclarativeTypeResult<DecTerm> {
    Ok(signature.return_ty(db).leashed_ty(db))
}
