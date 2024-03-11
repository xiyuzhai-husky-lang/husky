use husky_hir_ty::quary::HirQuary;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinQual {
    Ref,
    RefMut,
    Transient,
}

impl LinQual {
    pub const ALL: &'static [Self] = &[LinQual::Ref, LinQual::RefMut, LinQual::Transient];

    pub(crate) fn from_hir(quary: HirQuary) -> Self {
        match quary {
            HirQuary::Const => todo!(),
            HirQuary::StackPure { place } => todo!(),
            HirQuary::ImmutableOnStack { place } => todo!(),
            HirQuary::MutableOnStack { place } => todo!(),
            HirQuary::Transient => todo!(),
            HirQuary::Ref { guard } => todo!(),
            HirQuary::RefMut { place, lifetime } => todo!(),
            HirQuary::Leashed => todo!(),
            HirQuary::Todo => todo!(),
            HirQuary::Svar(_) => todo!(),
        }
    }
}
