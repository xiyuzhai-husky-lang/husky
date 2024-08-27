use husky_item_path_interface::ItemPathIdInterface;
use husky_linket_impl::pedestal::IsPedestal;
use vec_like::SmallVecPairMap;

#[enum_class::from_variants]
pub enum Chart<Pedestal: IsPedestal, R> {
    Dim0(ChartDim0<Pedestal, R>),
    Dim1(ChartDim1<Pedestal, R>),
    Dim2(ChartDim2<Pedestal, R>),
}

pub type ChartDim0<Pedestal, R> = (Pedestal, R);
pub type ChartDim1<Pedestal, R> = Vec<(Pedestal, R)>;
pub type ChartDim2<Pedestal, R> = Vec<Vec<(Pedestal, R)>>;
