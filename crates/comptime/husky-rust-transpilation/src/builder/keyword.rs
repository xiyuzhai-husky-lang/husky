use super::*;

pub enum RustKeyword {
    Fn,
    Impl,
    Pub,
    Type,
    Mod,
    Trait,
}

impl RustKeyword {
    fn code(self) -> &'static str {
        match self {
            RustKeyword::Fn => todo!(),
            RustKeyword::Impl => todo!(),
            RustKeyword::Pub => todo!(),
            RustKeyword::Type => todo!(),
            RustKeyword::Mod => todo!(),
            RustKeyword::Trait => todo!(),
        }
    }
}

impl<'a> RustTranspilationBuilder<'a> {
    pub(crate) fn keyword(&mut self, keyword: RustKeyword) {
        self.write_token_str(keyword.code())
    }
}
