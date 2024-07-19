#[salsa::jar]
pub struct LinketJar(
    crate::linket::Linket,
    crate::linket::linkets_emancipated_by_javelin,
    crate::linket::package_linkets,
    crate::template_argument::ty::LinTypePathLeading,
    crate::template_argument::ty::LinketRitchieType,
    crate::version_stamp::LinketVersionStamp,
    crate::version_stamp::linket_version_stamp,
    crate::version_stamp::linket_ty_path_leading_version_stamp,
    crate::version_stamp::linket_ty_ritchie_version_stamp,
    crate::trai::LinketTrait,
);
