use self::view::action::TraceViewActionBuffer;
use crate::*;
use anchor::Anchor;
use husky_item_path_interface::ItemPathIdInterface;
use husky_linket_impl::{pedestal::IsPedestal, pedestal::IsPedestalFull, var_id::IsVarId};
use ui::ui::IsUi;
use windlass::Windlass;

pub trait IsCaryatid:
    std::fmt::Debug
    + Default
    + Clone
    + Eq
    + Send
    + Sync
    + std::ops::Index<ItemPathIdInterface, Output = Windlass<<Self::Pedestal as IsPedestal>::VarId>>
    + 'static
{
    type Pedestal: IsPedestalFull;

    type UiBuffer: IsCaryatidUiBuffer<Caryatid = Self>;

    fn init_ui_buffer(&self) -> Self::UiBuffer;

    fn pedestal(&self, var_deps: &[ItemPathIdInterface]) -> Option<Self::Pedestal>;
    fn has(&self, var_deps: &[ItemPathIdInterface]) -> bool;
    fn with_extra_var_deps(&self, var_deps: &[ItemPathIdInterface]) -> Self;
    fn var_path_windlasses(
        &self,
    ) -> impl Iterator<
        Item = (
            ItemPathIdInterface,
            Windlass<<Self::Pedestal as IsPedestal>::VarId>,
        ),
    >;
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
        trace_synchrotron: &TraceSynchrotron<TraceProtocol>,
        ui: &mut Ui,
        caryatid_buffer: &mut Self::UiBuffer,
        action_buffer: &mut TraceViewActionBuffer<TraceProtocol>,
    ) where
        TraceProtocol: IsTraceProtocol<Pedestal = Self::Pedestal, Caryatid = Self>;
}

pub type TraceCaryatidUiBuffer<TraceProtocol> =
    <<TraceProtocol as IsTraceProtocol>::Caryatid as IsCaryatid>::UiBuffer;
