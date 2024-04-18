use husky_hir_ty::{
    quary::{HirContractedQuary, HirQuary},
    ritchie::HirContract,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinQual {
    Ref,
    RefMut,
    Transient,
}

impl LinQual {
    pub const ALL: &'static [Self] = &[LinQual::Ref, LinQual::RefMut, LinQual::Transient];

    // ad hoc
    pub fn from_hir(contracted_quary: HirContractedQuary) -> Self {
        match contracted_quary.quary() {
            HirQuary::Const => todo!(),
            HirQuary::StackPure { .. }
            | HirQuary::ImmutableOnStack { .. }
            | HirQuary::MutableOnStack { .. }
            | HirQuary::Ref { .. } => match contracted_quary.contract() {
                Some(contract) => match contract {
                    HirContract::Pure => LinQual::Ref,
                    HirContract::Move => LinQual::Transient,
                    HirContract::Borrow => LinQual::Ref,
                    HirContract::BorrowMut => LinQual::RefMut,
                    HirContract::Const => todo!(),
                    HirContract::Leash => LinQual::Ref,
                    HirContract::At => todo!(),
                },
                None => LinQual::Ref,
            },
            HirQuary::Transient => todo!(),
            HirQuary::RefMut { place, lifetime } => todo!(),
            HirQuary::Leashed { place_idx } => LinQual::Ref,
            HirQuary::Todo => todo!(),
            HirQuary::Variable(_) => todo!(),
        }
    }
}
