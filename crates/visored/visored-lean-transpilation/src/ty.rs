use crate::*;
use dictionary::item_path::VdItemPathTranslation;
use lean_term::ty::LnType;
use visored_zfc_ty::ty::{VdZfcType, VdZfcTypeData};

pub enum VdZfcTypeLeanTranspilation {
    Type(LnType),
}

impl VdTranspileToLean<VdZfcTypeLeanTranspilation> for VdZfcType {
    fn to_lean(self, builder: &mut VdLeanTranspilationBuilder) -> VdZfcTypeLeanTranspilation {
        let db = builder.db();
        if self.refinements(db).is_empty() {
            VdZfcTypeLeanTranspilation::Type(
                builder.build_ln_ty_from_vd_zfc_ty_without_refinements(self),
            )
        } else {
            todo!()
        }
    }
}

impl<'a> VdLeanTranspilationBuilder<'a> {
    fn build_ln_ty_from_vd_zfc_ty_without_refinements(&self, ty: VdZfcType) -> LnType {
        debug_assert!(ty.refinements(self.db()).is_empty());
        match ty.data(self.db()) {
            VdZfcTypeData::ItemPath(path) => {
                let Some(translation) = self.dictionary().item_path_translation(path) else {
                    todo!()
                };
                match *translation {
                    VdItemPathTranslation::ItemPath(ln_item_path) => {
                        LnType::new_item_path(ln_item_path)
                    }
                }
            }
        }
    }
}
