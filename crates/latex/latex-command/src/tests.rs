#[salsa::db(husky_coword::jar::CowordJar, crate::Jar)]
pub struct DB;
