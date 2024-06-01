#[salsa::db(
    husky_coword::jar::CowordJar,
    husky_vfs::jar::VfsJar,
    husky_token_data::jar::TokenDataJar,
    husky_token::TokenJar,
    husky_term_prelude::jar::TermPreludeJar,
    husky_entity_path::jar::EntityPathJar
)]
#[derive(Default)]
pub(crate) struct DB;
