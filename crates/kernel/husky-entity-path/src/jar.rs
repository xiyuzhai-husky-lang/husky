use crate::*;

#[salsa::jar]
pub struct EntityPathJar(
    crate::path::ItemPathId,
    crate::path::major_item::ty::prelude_ty_path,
    crate::path::major_item::trai::prelude_trai_path,
    crate::menu::item_path_menu,
);
