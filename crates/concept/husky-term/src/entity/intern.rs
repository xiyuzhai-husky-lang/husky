use super::TermEntityPath;
use interner::{DefaultInternedPtr, Interner};
pub type TermEntityPathPtr = DefaultInternedPtr<TermEntityPath, TermEntityPath>;

pub type TermEntityPathInterner = Interner<TermEntityPathPtr>;
