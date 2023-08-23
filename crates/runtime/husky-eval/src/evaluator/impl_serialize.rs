use super::*;
use husky_data_viewer::HuskyDataViewer;

impl<'temp> FeatureEvaluator<'temp> {
    pub(super) fn serialize(&self, value: &__RegularValue, ty: EtherealTerm) -> serde_json::Value {
        let ty_data_viewer: Arc<HuskyDataViewer> = self.db.ty_data_viewer(ty);
        ty_data_viewer.serialize(self.db.upcast(), value)
    }
}
