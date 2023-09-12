use crate::*;
use salsa::{Database, DebugWithDb, Storage};

#[salsa::db(
    husky_coword::CowordJar,
    husky_vfs::VfsJar,
    husky_token_data::db::TokenDataJar,
    husky_token::TokenJar,
    husky_term_prelude::TermPreludeJar,
    husky_entity_path::EntityPathJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: Storage<Self>,
}

impl Database for DB {}
