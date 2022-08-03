use super::*;
use entity_kind::TyKind;
use husky_compile_time::*;
use husky_data_viewer::HuskyDataViewer;

impl<'temp, 'eval: 'temp> FeatureEvaluator<'temp, 'eval> {
    pub(super) fn serialize(
        &self,
        value: &__Register<'eval>,
        ty: EntityRoutePtr,
    ) -> serde_json::Value {
        let ty_data_viewer: Arc<HuskyDataViewer> = self.db.ty_data_viewer(ty);
        todo!()
    }
}
