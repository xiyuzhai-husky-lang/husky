pub(crate) use expect_test::*;
pub(crate) use salsa::DebugWithDb;

use husky_coword::CowordJar;

#[salsa::db(CowordJar)]
pub struct DB {}
