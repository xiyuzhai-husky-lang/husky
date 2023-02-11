use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermMenu2 {
    static_str_ref: Term,
    parent: TermMenu1,
}

impl std::ops::Deref for TermMenu2 {
    type Target = TermMenu1;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl TermMenu2 {
    pub(crate) fn new(db: &dyn TermDb, toolchain: Toolchain, menu1: TermMenu1) -> TermResult<Self> {
        // db.it_entity_path_term(db.entity_path_menu(toolchain).as_ref()?.r32());
        Ok(TermMenu2 {
            static_str_ref: TermApplication::new(db, menu1.static_ref_ty(), menu1.str_ty_path())
                .into(),
            parent: menu1,
        })
    }

    pub fn static_str_ref(&self) -> Term {
        self.static_str_ref
    }
}
