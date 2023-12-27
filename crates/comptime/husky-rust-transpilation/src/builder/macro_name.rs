use super::*;

pub enum RustMacroName {
    Vec,
    Println,
    Todo,
    Unreachable,
    Panic,
    Require,
    Assert,
    Matches,
    Unveil,
    LinkageImpls,
    FnLinkageImpl,
    UnveilFnLinkageImpl,
    GnLinkageImpl,
    StructFieldLinkageImpl,
    TypeDefault,
}

impl RustMacroName {
    fn code(self) -> &'static str {
        match self {
            RustMacroName::Vec => "vec!",
            RustMacroName::Println => "println!",
            RustMacroName::Todo => "todo!",
            RustMacroName::Unreachable => "unreachable!",
            RustMacroName::Panic => "panic!",
            RustMacroName::Require => "require!",
            RustMacroName::Assert => "assert!",
            RustMacroName::Matches => "matches!",
            RustMacroName::Unveil => "unveil!",
            RustMacroName::LinkageImpls => "linkage_impls!",
            RustMacroName::FnLinkageImpl => "fn_linkage_impl!",
            RustMacroName::UnveilFnLinkageImpl => "unveil_fn_linkage_impl!",
            RustMacroName::GnLinkageImpl => "gn_linkage_impl!",
            RustMacroName::StructFieldLinkageImpl => "struct_field_linkage_impl!",
            RustMacroName::TypeDefault => "ty_default_linkage_impl!",
        }
    }
}

impl<'a, 'b, E> RustTranspilationBuilder<'a, 'b, E> {
    pub(crate) fn macro_name(&mut self, macro_name: RustMacroName) {
        if self.result.ends_with(|c: char| c.is_alphabetic()) {
            self.write_str(" ")
        }
        self.write_str(macro_name.code())
    }

    pub(crate) fn macro_call(&mut self, macro_name: RustMacroName, f: impl FnOnce(&mut Self)) {
        self.macro_name(macro_name);
        self.bracketed(RustBracket::Par, f)
    }
}
