#[eterned::eterned]
pub struct VdClausePatt {
    #[return_ref]
    data: VdClausePattData,
}

impl std::fmt::Debug for VdClausePatt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VdClausePattData {}
