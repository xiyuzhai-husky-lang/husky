use text::TextRange;

pub struct AtomError {
    pub origin: TextRange,
    pub variant: AtomErrorVariant,
}

pub enum AtomErrorVariant {
    ScopeResolveError,
}
