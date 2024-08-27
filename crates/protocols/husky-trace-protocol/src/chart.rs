use husky_item_path_interface::ItemPathIdInterface;
use husky_linket_impl::{pedestal::JointPedestal, static_var::IsStaticVarId};
use vec_like::SmallVecPairMap;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq)]
pub enum Chart<StaticVarId: IsStaticVarId, R> {
    Dim0(ChartDim0<StaticVarId, R>),
    Dim1(ChartDim1<StaticVarId, R>),
    Dim2(ChartDim2<StaticVarId, R>),
}

pub type ChartDim0<StaticVarId, R> = (JointPedestal<StaticVarId>, R);
pub type ChartDim1<StaticVarId, R> = Vec<(JointPedestal<StaticVarId>, R)>;
pub type ChartDim2<StaticVarId, R> = Vec<Vec<(JointPedestal<StaticVarId>, R)>>;
