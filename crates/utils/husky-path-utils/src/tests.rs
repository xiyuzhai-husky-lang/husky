use husky_coword::CowordJar;

#[salsa::test_db(CowordJar)]
#[derive(Default)]
pub(crate) struct DB;
