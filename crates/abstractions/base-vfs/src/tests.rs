use crate::{
    // watch::{WatchedVfs, DEBOUNCE_TEST_SLEEP_TIME},
    *,
};

#[salsa::db(Jar, husky_coword::jar::CowordJar)]
#[derive(Default)]
pub(crate) struct DB;
