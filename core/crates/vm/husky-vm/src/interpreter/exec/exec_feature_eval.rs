use crate::*;
use avec::Avec;
use husky_check_utils::should;
use husky_print_utils::p;

impl<'temp, 'eval: 'temp> Interpreter<'temp, 'eval> {
    pub(super) fn exec_feature_eval(&mut self, feature_uid: EntityUid) -> VMControl<'eval> {
        todo!()
    }
}
