#[salsa::jar]
pub struct FlyTermJar(
    crate::term_ritchie_fly_data,
    crate::term_application_fly_data,
);
