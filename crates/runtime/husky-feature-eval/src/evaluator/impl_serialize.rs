use super::*;
use husky_comptime::*;
use husky_data_viewer::HuskyDataViewer;
use husky_entity_kind::TyKind;

impl<'temp, 'eval: 'temp> FeatureEvaluator<'temp, 'eval> {
    pub(super) fn serialize(
        &self,
        comptime: &HuskyComptime,
        value: &__Register<'eval>,
        ty: EntityRoutePtr,
    ) -> serde_json::Value {
        let ty_data_viewer: Arc<HuskyDataViewer> = self.db.ty_data_viewer(ty);
        ty_data_viewer.serialize(self.db.upcast(), value)
    }
}
