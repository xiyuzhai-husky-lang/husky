pub(crate) use expect_test::*;
use husky_coword::CowordJar;

use crate::*;

#[salsa::db(CowordJar)]
pub struct DB {}
