use visored_mir_opr::separator::VdMirBaseSeparator;

pub enum VdBsqOrderComparison {
    Lt,
    Gt,
    Le,
    Ge,
}

impl VdBsqOrderComparison {
    pub fn from_mir_base_separator(separator: VdMirBaseSeparator) -> Option<Self> {
        match separator {
            VdMirBaseSeparator::Lt => Some(Self::Lt),
            VdMirBaseSeparator::Gt => Some(Self::Gt),
            VdMirBaseSeparator::Le => Some(Self::Le),
            VdMirBaseSeparator::Ge => Some(Self::Ge),
            _ => None,
        }
    }
}
