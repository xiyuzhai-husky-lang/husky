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
}
