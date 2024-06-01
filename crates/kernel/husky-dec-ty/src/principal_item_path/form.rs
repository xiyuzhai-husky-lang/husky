use super::*;
use husky_dec_signature::signature::major_item::form::{
    compterm::MajorComptermDecTemplate, function_ritchie::MajorFunctionRitchieDecTemplate,
    val::MajorValDecTemplate, MajorFormDecTemplate,
};
use husky_entity_path::path::major_item::form::MajorFormPath;
use husky_vfs::toolchain::Toolchain;
use smallvec::ToSmallVec;

#[salsa::tracked(jar = DecTypeJar)]
pub fn form_path_declarative_ty(
    db: &::salsa::Db,
    path: MajorFormPath,
) -> DeclarativeTypeResult<DecTerm> {
    let signature = match path.dec_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    let Ok(variances) = form_path_variances(db, path) else {
        todo!()
    };
    // ad hoc
    let variances = &variances;
    let dec_term_menu = db.dec_term_menu(path.toolchain(db)).unwrap();
    match signature {
        MajorFormDecTemplate::Ritchie(signature) => {
            ritchie_path_declarative_ty(db, path.toolchain(db), variances, signature)
        }
        MajorFormDecTemplate::Val(signature) => {
            val_path_declarative_ty(db, signature, dec_term_menu)
        }
        MajorFormDecTemplate::TypeAlias(_) => todo!(),
        MajorFormDecTemplate::Compterm(signature) => {
            compterm_path_declarative_ty(db, signature, dec_term_menu)
        }
        MajorFormDecTemplate::Static(_) => todo!(),
    }
}

pub(crate) fn ritchie_path_declarative_ty(
    db: &::salsa::Db,
    toolchain: Toolchain,
    variances: &[Variance],
    signature: MajorFunctionRitchieDecTemplate,
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
            signature.ritchie_item_kind(db).into(),
            parenate_parameters,
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

pub(crate) fn compterm_path_declarative_ty(
    db: &::salsa::Db,
    signature: MajorComptermDecTemplate,
    _declarative_term_menu: &DecTermMenu,
) -> DeclarativeTypeResult<DecTerm> {
    Ok(signature.return_ty(db).leashed_ty(db))
}
