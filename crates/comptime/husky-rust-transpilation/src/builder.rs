use crate::*;
use husky_coword::Ident;
use husky_token_data::Keyword;

pub(crate) struct RustTranspilationBuilder<'a> {
    db: &'a dyn RustTranspilationDb,
    result: String,
    current_indent: u32,
}

impl<'a> RustTranspilationBuilder<'a> {
    pub(crate) fn new(db: &'a dyn RustTranspilationDb) -> Self {
        Self {
            db,
            result: Default::default(),
            current_indent: 0,
        }
    }

    pub(crate) fn finish(self) -> String {
        self.result
    }

    pub(crate) fn db(&self) -> &'a dyn RustTranspilationDb {
        self.db
    }

    pub(crate) fn new_semicolon_line(&mut self, f: impl FnOnce(&mut Self)) {
        self.write_indent();
        f(self);
        self.write_str(";\n")
    }

    // add whitespace if not newline
    fn write_token_str(&mut self, s: &str) {
        if !self.result.ends_with("\n") {
            self.write_str(" ")
        }
        self.write_str(s);
    }

    fn write_indent(&mut self) {
        for _ in 0..self.current_indent {
            self.result.push(' ')
        }
    }

    fn write_str(&mut self, s: &str) {
        self.result += s
    }
}

pub(crate) trait TranspileToRust {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder);
}

impl TranspileToRust for Keyword {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self {
            Keyword::Fugitive(_) => todo!(),
            Keyword::TypeEntity(_) => todo!(),
            Keyword::Stmt(_) => todo!(),
            Keyword::Modifier(_) => todo!(),
            Keyword::Pronoun(_) => todo!(),
            Keyword::Connection(_) => todo!(),
            Keyword::End(_) => todo!(),
            Keyword::Pub => todo!(),
            Keyword::Const => todo!(),
            Keyword::Static => todo!(),
            Keyword::Async => todo!(),
            Keyword::Sorry => todo!(),
            Keyword::Todo => todo!(),
            Keyword::Unreachable => todo!(),
            _ => builder.write_token_str(self.code()),
        }
    }
}

impl TranspileToRust for Ident {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        builder.write_token_str(self.data(builder.db()))
    }
}
