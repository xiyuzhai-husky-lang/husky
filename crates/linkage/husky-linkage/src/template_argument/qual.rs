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

    pub(crate) fn from_hir(contracted_quary: HirContractedQuary) -> Self {
        match contracted_quary.quary() {
            HirQuary::Const => todo!(),
            HirQuary::StackPure { place } => todo!(),
            HirQuary::ImmutableOnStack { place } => todo!(),
            HirQuary::MutableOnStack { .. } => match contracted_quary.contract() {
                Some(contract) => match contract {
                    HirContract::Pure => LinQual::Ref,
                    HirContract::Move => LinQual::Transient,
                    HirContract::Borrow => LinQual::Ref,
                    HirContract::BorrowMut => LinQual::RefMut,
                    HirContract::Const => todo!(),
                    HirContract::Leash => todo!(),
                    HirContract::At => todo!(),
                },
                None => unreachable!(),
            },
            HirQuary::Transient => todo!(),
            HirQuary::Ref { guard } => todo!(),
            HirQuary::RefMut { place, lifetime } => todo!(),
            HirQuary::Leashed => todo!(),
            HirQuary::Todo => todo!(),
            HirQuary::Svar(_) => todo!(),
        }
    }
}
