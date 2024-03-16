use husky_coword::{Ident, Label};

#[enum_class::from_variants]
#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolName {
    Ident(Ident),
    Label(Label),
    SelfType,
    SelfValue,
    SelfLifetime,
    SelfPlace,
}

impl SymbolName {
    pub fn ident(self) -> Option<Ident> {
        match self {
            SymbolName::Ident(ident) => Some(ident),
            SymbolName::Label(_)
            | SymbolName::SelfType
            | SymbolName::SelfValue
            | SymbolName::SelfLifetime
            | SymbolName::SelfPlace => None,
        }
    }
}

impl salsa::DisplayWithDb for SymbolName {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &salsa::Db,
    ) -> std::fmt::Result {
        match self {
            SymbolName::Ident(ident) => f.write_str(ident.data(db)),
            SymbolName::Label(label) => f.write_fmt(format_args!("'{}", label.data(db))),
            SymbolName::SelfType => f.write_str("Self"),
            SymbolName::SelfValue => f.write_str("self"),
            SymbolName::SelfLifetime => f.write_str("'self_lifetime"),
            SymbolName::SelfPlace => f.write_str("'self_place"),
        }
    }
}
