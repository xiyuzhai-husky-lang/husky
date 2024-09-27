use husky_hir_ty::{
    quary::{HirContractedQuary, HirQuary},
    ritchie::HirContract,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinQual {
    Ref,
    Mut,
    Transient,
}

impl LinQual {
    pub const ALL: &'static [Self] = &[LinQual::Ref, LinQual::Mut, LinQual::Transient];

    // ad hoc
    pub fn from_hir_contracted_quary(contracted_quary: HirContractedQuary) -> Self {
        match contracted_quary.quary() {
            HirQuary::Compterm => todo!(),
            HirQuary::StackPure { .. }
            | HirQuary::ImmutableOnStack { .. }
            | HirQuary::MutableOnStack { .. }
            | HirQuary::Ref { .. } => match contracted_quary.contract() {
                Some(contract) => match contract {
                    HirContract::Pure => LinQual::Ref,
                    HirContract::Move => LinQual::Transient,
                    HirContract::Borrow => LinQual::Ref,
                    HirContract::BorrowMut => LinQual::Mut,
                    HirContract::Compterm => todo!(),
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

    pub fn variable_qual(
        contracted_quary: HirContractedQuary,
        is_base_ty_always_copyable: bool,
    ) -> Self {
        match contracted_quary.quary() {
            HirQuary::Compterm => todo!(),
            HirQuary::StackPure { .. }
            | HirQuary::ImmutableOnStack { .. }
            | HirQuary::MutableOnStack { .. }
            | HirQuary::Ref { .. } => match contracted_quary.contract() {
                Some(contract) => match contract {
                    HirContract::Pure => {
                        if is_base_ty_always_copyable {
                            LinQual::Transient
                        } else {
                            LinQual::Ref
                        }
                    }
                    HirContract::Move => LinQual::Transient,
                    HirContract::Borrow => LinQual::Ref,
                    HirContract::BorrowMut => LinQual::Mut,
                    HirContract::Compterm => todo!(),
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
