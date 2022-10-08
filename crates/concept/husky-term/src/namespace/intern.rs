use super::Namespace;
use interner::{DefaultInternedPtr, Interner};
pub type NamespacePtr = DefaultInternedPtr<Namespace, Namespace>;

pub type NamespaceInterner = Interner<NamespacePtr>;
