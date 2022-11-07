#[derive(Debug, Clone, Copy)]
pub struct UnresolvedTermIdx(usize);

pub struct UnresolvedTermRegistry {
    terms: Vec<UnresolvedTerm>,
}

pub struct UnresolvedTerm {}
