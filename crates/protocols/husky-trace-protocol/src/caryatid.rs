use crate::*;
use husky_devsoul_interface::{
    item_path::ItemPathIdInterface,
    pedestal::{IsPedestal, IsPedestalFull},
};

use ui::ui::IsUi;

use self::view::action::TraceViewActionBuffer;

pub trait IsCaryatid:
    std::fmt::Debug + Default + Clone + Eq + std::hash::Hash + Send + Sync + 'static
{
    type Pedestal: IsPedestalFull;

    type UiBuffer: IsCaryatidUiBuffer<Caryatid = Self>;

    fn init_ui_buffer(&self) -> Self::UiBuffer;

    fn pedestal(&self, var_deps: &[ItemPathIdInterface]) -> Self::Pedestal;
    fn covers(&self, var_deps: &[ItemPathIdInterface]) -> bool;
}

pub trait IsCaryatidUiBuffer {
    type Caryatid: IsCaryatid;

    fn update(&mut self, caryatid: &Self::Caryatid);
}

pub trait IsCaryatidFull: IsCaryatid + Serialize + for<'a> Deserialize<'a> {}

impl<T> IsCaryatidFull for T where T: IsCaryatid + Serialize + for<'a> Deserialize<'a> {}

pub trait CaryatidUi<Ui: IsUi>: IsCaryatidFull {
    fn caryatid_ui<TraceProtocol>(
        &self,
        ui: &mut Ui,
        caryatid_buffer: &mut Self::UiBuffer,
        action_buffer: &mut TraceViewActionBuffer<TraceProtocol>,
    ) where
        TraceProtocol: IsTraceProtocol<Caryatid = Self>;
}

pub type TraceCaryatidUiBuffer<TraceProtocol> =
    <<TraceProtocol as IsTraceProtocol>::Caryatid as IsCaryatid>::UiBuffer;
