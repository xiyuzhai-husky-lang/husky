use crate::ui::IsUi;
use husky_visual_protocol::synchrotron::VisualSynchrotron;

pub trait VisualWidget<Ui: IsUi> {
    fn ui(&self, visual_synchrotron: &VisualSynchrotron, cache: &mut Ui::Cache, ui: &mut Ui);
}
