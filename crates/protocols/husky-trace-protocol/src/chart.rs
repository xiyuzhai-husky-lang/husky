use husky_item_path_interface::ItemPathIdInterface;
use husky_linket_impl::{pedestal::JointPedestal, var_id::IsVarId};
use vec_like::SmallVecPairMap;

/// the name comes from differential geometry and algebraic geometry
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq)]
pub enum Chart<VarId: IsVarId, R> {
    Dim0(ChartDim0<VarId, R>),
    Dim1(ChartDim1<VarId, R>),
    Dim2(ChartDim2<VarId, R>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ChartDim0<VarId, R>
where
    VarId: IsVarId,
{
    pub joint_pedestal: JointPedestal<VarId>,
    pub r: R,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ChartDim1<VarId, R>
where
    VarId: IsVarId,
{
    pub points: Vec<(VarId, ChartDim0<VarId, R>)>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ChartDim2<VarId, R>
where
    VarId: IsVarId,
{
    pub rows: Vec<(VarId, ChartDim1<VarId, R>)>,
}
