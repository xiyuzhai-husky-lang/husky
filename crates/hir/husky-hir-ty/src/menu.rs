use crate::*;

pub struct HirTypeMenu {
    unit_ty: HirTypePathLeading,
}

impl HirTypeMenu {
    pub fn unit_ty(&self) -> HirTypePathLeading {
        self.unit_ty
    }
}
