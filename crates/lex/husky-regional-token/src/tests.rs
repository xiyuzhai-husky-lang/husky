pub use salsa::test_utils::Db;

use salsa::Storage;

#[salsa::test_db(
    husky_coword::CowordJar,
    husky_vfs::VfsJar,
    husky_token_data::db::TokenDataJar,
    husky_token::TokenJar,
    husky_term_prelude::TermPreludeJar,
    husky_entity_path::EntityPathJar
)]
#[derive(Default)]
pub(crate) struct DB;
