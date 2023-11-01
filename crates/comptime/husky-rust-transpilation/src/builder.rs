use crate::*;
use husky_coword::Ident;
use husky_hir_ty::{HirTemplateSymbol, HirTypeSymbol};
use husky_syn_opr::Bracket;
use husky_token_data::{Keyword, StmtKeyword};

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
        self.write_token_str(";\n")
    }

    fn write_indent(&mut self) {
        for _ in 0..self.current_indent {
            self.result.push(' ')
        }
    }

    fn write_token_str(&mut self, s: &str) {
        self.result += s
    }

    fn write_str(&mut self, s: &str) {
        self.result += s
    }

    pub(crate) fn bracketed_comma_list<A: TranspileToRust>(
        &mut self,
        bracket: Bracket,
        items: impl IntoIterator<Item = A>,
    ) {
        self.write_str(bracket.bra_code());
        let mut start = true;
        for item in items {
            if start {
                start = false
            } else {
                self.write_str(", ")
            }
            item.transpile_to_rust(self)
        }
        self.write_str(bracket.ket_code());
    }
}

pub(crate) trait TranspileToRust {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder);
}

impl<T> TranspileToRust for &T
where
    T: TranspileToRust,
{
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        <T as TranspileToRust>::transpile_to_rust(self, builder)
    }
}

impl TranspileToRust for Keyword {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self {
            Keyword::Stmt(StmtKeyword::Forext) => todo!(),
            Keyword::End(_) => todo!(),
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

impl TranspileToRust for HirTemplateSymbol {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self {
            HirTemplateSymbol::Type(symbol) => match symbol {
                HirTypeSymbol::Type {
                    attrs,
                    variance,
                    disambiguator,
                } => match disambiguator {
                    0 => builder.write_str("A"),
                    1 => builder.write_str("B"),
                    2 => builder.write_str("C"),
                    _ => todo!(),
                },
                HirTypeSymbol::SelfType => todo!(),
                HirTypeSymbol::SelfLifetime => todo!(),
                HirTypeSymbol::SelfPlace => todo!(),
            },
            HirTemplateSymbol::Const(_) => todo!(),
            HirTemplateSymbol::Lifetime(_) => todo!(),
            HirTemplateSymbol::Place(_) => todo!(),
        }
    }
}
