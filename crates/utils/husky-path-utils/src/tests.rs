use husky_coword::jar::CowordJar;

#[salsa::db(CowordJar)]
#[derive(Default)]
pub(crate) struct DB;
