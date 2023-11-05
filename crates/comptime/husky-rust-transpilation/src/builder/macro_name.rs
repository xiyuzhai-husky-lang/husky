use super::*;

pub enum RustMacroName {
    Vec,
    Println,
    Unreachable,
    Panic,
    Require,
    Assert,
    Matches,
}

impl RustMacroName {
    fn code(self) -> &'static str {
        match self {
            RustMacroName::Vec => "vec!",
            RustMacroName::Println => "println!",
            RustMacroName::Unreachable => "unreachable!",
            RustMacroName::Panic => "panic!",
            RustMacroName::Require => "require!",
            RustMacroName::Assert => "assert!",
            RustMacroName::Matches => "matches!",
        }
    }
}

impl<'a> RustTranspilationBuilder<'a> {
    pub(crate) fn macro_name(&mut self, macro_name: RustMacroName) {
        if self.result.ends_with(|c: char| c.is_alphabetic()) {
            self.write_str(" ")
        }
        self.write_str(macro_name.code())
    }
}
