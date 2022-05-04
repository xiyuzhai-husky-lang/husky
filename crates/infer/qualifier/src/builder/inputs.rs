use super::*;
use check_utils::should;
use defn_head::InputPlaceholder;
use text::TextRanged;

impl<'a> QualifiedTySheetBuilder<'a> {
    pub(super) fn add_inputs(&mut self, inputs: &[InputPlaceholder]) {
        if inputs.len() > 0 {
            if let None = self
                .qualified_ty_sheet
                .variable_qualified_tys
                .get(&(inputs[0].ident.ident, inputs[0].ranged_ty.row()))
            {
                for input in inputs {
                    let ty = input.ranged_ty.route;
                    should!(self
                        .qualified_ty_sheet
                        .variable_qualified_tys
                        .insert(
                            (input.ident.ident, inputs[0].ranged_ty.row()),
                            Ok(QualifiedTy {
                                ty,
                                qual: Qualifier::from_input(
                                    input.contract,
                                    self.db.is_copyable(ty)
                                ),
                            }),
                        )
                        .is_none());
                }
            }
        }
    }
}
