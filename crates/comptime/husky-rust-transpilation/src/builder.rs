pub(crate) mod keyword;

use crate::*;
use husky_coword::Ident;
use husky_hir_eager_expr::HirEagerExprArena;
use husky_hir_expr::HirExprRegion;
use husky_hir_lazy_expr::HirLazyExprArena;
use husky_hir_ty::{HirTemplateSymbol, HirTypeSymbol};
use husky_syn_opr::Bracket;

const INDENT_UNIT: u32 = 4;

pub(crate) struct RustTranspilationBuilder<'a> {
    db: &'a dyn RustTranspilationDb,
    result: String,
    current_indent: u32,
    is_list_start: Option<bool>,
    hir_expr_region: Option<HirExprRegion>,
}

impl<'a> RustTranspilationBuilder<'a> {
    pub(crate) fn new(db: &'a dyn RustTranspilationDb) -> Self {
        Self {
            db,
            result: Default::default(),
            current_indent: 0,
            is_list_start: None,
            hir_expr_region: None,
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

    pub(crate) fn heterogeneous_bracketed_comma_list(
        &mut self,
        bracket: Bracket,
        items: impl FnOnce(&mut Self),
    ) {
        self.write_str(bracket.bra_code());
        items(self);
        self.write_str(bracket.ket_code());
    }

    pub(crate) fn heterogeneous_comma_list_items<A: TranspileToRust>(
        &mut self,
        items: impl IntoIterator<Item = A>,
    ) {
        for item in items {
            self.heterogeneous_comma_list_item(item)
        }
    }

    pub(crate) fn heterogeneous_comma_list_item<A: TranspileToRust>(&mut self, item: A) {
        let Some(ref mut is_list_start) = self.is_list_start else {
            unreachable!()
        };
        if *is_list_start {
            *is_list_start = false
        } else {
            self.write_str(", ")
        }
        item.transpile_to_rust(self)
    }

    pub(crate) fn curly_block(&mut self, f: impl FnOnce(&mut Self)) {
        self.write_token_str("{");
        self.current_indent += INDENT_UNIT;
        f(self);
        self.current_indent -= INDENT_UNIT;
        if !self.result.ends_with("\n") {
            self.result += "\n"
        }
        self.write_indent();
        self.write_str("}\n");
    }

    pub(crate) fn with_hir_expr_region(
        &mut self,
        hir_expr_region: impl Into<HirExprRegion>,
        f: impl FnOnce(&mut Self),
    ) {
        debug_assert!(self.hir_expr_region.is_none());
        self.hir_expr_region = Some(hir_expr_region.into());
        f(self);
        self.hir_expr_region = None
    }

    pub(crate) fn hir_eager_expr_arena(&self) -> &HirEagerExprArena {
        let Some(HirExprRegion::Eager(hir_eager_expr_region)) = self.hir_expr_region else {
            unreachable!()
        };
        hir_eager_expr_region.hir_eager_expr_arena(self.db)
    }

    pub(crate) fn hir_lazy_expr_arena(&self) -> &HirLazyExprArena {
        let Some(HirExprRegion::Lazy(hir_lazy_expr_region)) = self.hir_expr_region else {
            unreachable!()
        };
        hir_lazy_expr_region.hir_lazy_expr_arena(self.db)
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
                HirTypeSymbol::SelfType => builder.write_str("This"),
                HirTypeSymbol::SelfLifetime => todo!(),
                HirTypeSymbol::SelfPlace => todo!(),
            },
            HirTemplateSymbol::Const(_) => todo!(),
            HirTemplateSymbol::Lifetime(_) => todo!(),
            HirTemplateSymbol::Place(_) => todo!(),
        }
    }
}
