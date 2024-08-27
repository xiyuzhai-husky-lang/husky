use husky_item_path_interface::ItemPathIdInterface;
use vec_like::SmallVecPairMap;

#[enum_class::from_variants]
pub enum Chart<StaticVarId, R> {
    Dim0(ChartDim0<StaticVarId, R>),
    Dim1(ChartDim1<StaticVarId, R>),
    Dim2(ChartDim2<StaticVarId, R>),
}

pub type ChartDim0<StaticVarId, R> = (SmallVecPairMap<ItemPathIdInterface, StaticVarId, 2>, R);
pub type ChartDim1<StaticVarId, R> = Vec<(SmallVecPairMap<ItemPathIdInterface, StaticVarId, 2>, R)>;
pub type ChartDim2<StaticVarId, R> =
    Vec<Vec<(SmallVecPairMap<ItemPathIdInterface, StaticVarId, 2>, R)>>;
