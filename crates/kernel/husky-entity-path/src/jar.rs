use crate::*;

#[salsa::jar(db = EntityPathDb)]
pub struct EntityPathJar(
    ItemPathId,
    crate::path::major_item::ty::prelude_ty_path,
    crate::path::major_item::trai::prelude_trai_path,
    item_path_menu,
);
