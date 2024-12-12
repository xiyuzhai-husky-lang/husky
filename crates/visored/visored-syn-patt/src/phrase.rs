#[eterned::eterned]
pub struct VdPhrasePatt {
    #[return_ref]
    data: VdPhrasePattData,
}

impl std::fmt::Debug for VdPhrasePatt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VdPhrasePattData {}
