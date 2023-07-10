use husky_coword::CowordJar;

#[salsa::db(CowordJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<DB>,
}

impl salsa::Database for DB {}
