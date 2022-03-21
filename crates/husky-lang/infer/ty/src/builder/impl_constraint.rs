use super::*;

impl<'a> TySheetBuilder<'a> {
    pub(super) fn add_variable_constraint(
        &mut self,
        ident: CustomIdentifier,
        line_group_idx: usize,
        constraint: Constraint,
        change_flag: &mut bool,
    ) -> Option<ScopePtr> {
        if let Some(entry) = self.ty_sheet.variables.get_mut((ident, line_group_idx)) {
            Some(entry.add_constraint(constraint))
        } else {
            self.ty_sheet.variables.insert_new(
                (ident, line_group_idx),
                TySheetEntry::new(constraint.clone()),
            );
            *change_flag = true;
            constraint.ty_result.ok()
        }
    }

    pub(super) fn add_expr_constraint(
        &mut self,
        expr_idx: RawExprIdx,
        constraint: Constraint,
        change_flag: &mut bool,
    ) -> Option<ScopePtr> {
        if let Some(entry) = self.ty_sheet.exprs.get_mut(expr_idx) {
            Some(entry.add_constraint(constraint))
        } else {
            self.ty_sheet
                .exprs
                .insert_new(expr_idx, TySheetEntry::new(constraint.clone()));
            *change_flag = true;
            constraint.ty_result.ok()
        }
    }
}
