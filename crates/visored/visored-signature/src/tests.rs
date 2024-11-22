use crate::*;

#[salsa::db(
    husky_coword::jar::CowordJar,
    latex_vfs::jar::LxVfsJar,
    visored_item_path::jar::VdItemPathJar,
    visored_term::jar::VdTermJar,
    Jar
)]
pub(crate) struct DB();
