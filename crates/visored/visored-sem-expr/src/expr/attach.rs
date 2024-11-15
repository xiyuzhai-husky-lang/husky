use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum AttachDispatch {}

impl<'a> VdSemExprBuilder<'a> {
    pub(super) fn build_attach(
        &mut self,
        base: VdSynExprIdx,
        scripts: &[(LxScriptKind, VdSynExprIdx)],
    ) -> (VdSemExprData, VdZfcType) {
        if let Some(data_and_ty) = self.try_build_power(base, scripts) {
            return data_and_ty;
        }
        todo!()
    }

    fn try_build_power(
        &mut self,
        base: VdSynExprIdx,
        scripts: &[(LxScriptKind, VdSynExprIdx)],
    ) -> Option<(VdSemExprData, VdZfcType)> {
        let [(LxScriptKind::Superscript, exponent)] = *scripts else {
            return None;
        };
        match self.syn_expr_arena()[exponent] {
            VdSynExprData::Delimited { .. } => todo!(),
            VdSynExprData::Err(_) => todo!(),
            _ => (),
        }
        // TODO: consider annotation
        // avoid allocation because we're not certain at this point
        let base = self.build_expr_entry(base);
        let exponent = self.build_expr_entry(exponent);
        if let Some(_) = self
            .default_global_dispatch_table()
            .power_default_dispatch(base.ty, exponent.ty)
        {
            todo!()
        }
        todo!()
    }
}
