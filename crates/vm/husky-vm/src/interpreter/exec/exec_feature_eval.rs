use crate::*;

impl<'temp> Interpreter<'temp> {
    pub(super) fn exec_feature_eval(
        &mut self,
        feature_uid: EntityUid,
        mode: Mode,
        ins: &Instruction,
        ty: EtherealTerm,
    ) -> __VMResult<()> {
        let ctx = self.opt_ctx.unwrap();
        let result = ctx.eval_feature_from_uid(feature_uid.raw());
        match mode {
            Mode::Fast | Mode::TrackMutation => (),
            Mode::TrackHistory => {
                todo!()
                // self.history.write(
                //     ins,
                //     HistoryEntry::PureExpr {
                //         result: result.clone(),
                //         ty,
                //     },
                // );
            }
        }
        self.stack.push(result?);
        Ok(())
    }
}
