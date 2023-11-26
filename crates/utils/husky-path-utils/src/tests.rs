use husky_coword::CowordJar;

#[salsa::db(CowordJar)]
#[derive(Default)]
pub(crate) struct DB;
