#[eterned::eterned]
pub struct VdSentencePatt {
    #[return_ref]
    data: VdSentencePattData,
}

impl std::fmt::Debug for VdSentencePatt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VdSentencePattData {}
