#[eterned::eterned]
pub struct VdExprPatt {
    #[return_ref]
    data: VdExprPattData,
}

impl std::fmt::Debug for VdExprPatt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VdExprPattData {}
