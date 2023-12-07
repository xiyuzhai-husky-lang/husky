mod ad_hoc;
mod hir_ty;
pub(crate) mod keyword;
mod literal;
mod macro_name;
mod punctuation;

pub(crate) use self::keyword::*;
pub(crate) use self::macro_name::*;
pub(crate) use self::punctuation::*;

use crate::{expr::site::HirEagerExprSite, *};
use husky_coword::{Ident, Label};
use husky_entity_path::{PreludeTypePath, PrincipalEntityPath, TypePath};
use husky_hir_eager_expr::{
    symbol::{
        comptime_symbol::HirEagerComptimeSymbolName,
        runtime_symbol::{HirEagerRuntimeSymbolIdx, HirEagerRuntimeSymbolName},
    },
    HirEagerExprArena, HirEagerExprIdx, HirEagerExprRegion, HirEagerPatternExprArena,
    HirEagerStmtArena,
};
use husky_hir_lazy_expr::{HirLazyExprArena, HirLazyExprRegion, HirLazyStmtArena};
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_hir_ty::{
    trai::HirTrait, HirConstant, HirTemplateArgument, HirTemplateSymbol, HirType, HirTypeSymbol,
};
use husky_print_utils::p;
use husky_term_prelude::TermLiteral;

const INDENT_UNIT: u32 = 4;

pub(crate) struct RustTranspilationBuilderBase<'a> {
    db: &'a ::salsa::Db,
    result: String,
    current_indent: u32,
    is_list_start: Option<bool>,
    spaced: bool,
}

impl<'a> RustTranspilationBuilderBase<'a> {
    pub(crate) fn new(db: &'a ::salsa::Db, result: Option<&'static str>) -> Self {
        Self {
            db,
            result: result.unwrap_or_default().to_string(),
            current_indent: 0,
            is_list_start: None,
            spaced: true,
        }
    }

    pub(crate) fn finish(self) -> String {
        self.result
    }

    pub(crate) fn result(&self) -> &str {
        &self.result
    }

    pub(crate) fn db(&self) -> &'a ::salsa::Db {
        self.db
    }

    fn fresh_line(&mut self) {
        if !self.is_line_fresh() {
            self.result += "\n";
        }
        self.write_indent();
    }

    fn fresh_paragraph(&mut self) {
        if self.result.len() == 0 {
            ()
        } else if !self.is_line_fresh() {
            if !self.result.ends_with("{") {
                self.result += "\n"
            }
            self.result += "\n";
            self.write_indent();
        } else if !(self.result.ends_with("{\n") || self.result.ends_with("\n\n")) {
            self.result += "\n";
            self.write_indent();
        }
    }

    pub(crate) fn comment(&mut self, s: &str) {
        for line in s.lines() {
            self.fresh_line();
            self.result += "// ";
            self.result += line
        }
    }

    fn write_indent(&mut self) {
        for _ in 0..self.current_indent {
            self.result.push(' ')
        }
    }

    pub(crate) fn is_line_fresh(&self) -> bool {
        self.result.is_empty() || self.result.ends_with("\n")
    }

    fn write_str(&mut self, s: &str) {
        self.result += s
    }

    fn word(&mut self, word: &str) {
        if self.result.ends_with(|c: char| c.is_alphanumeric()) {
            self.write_str(" ")
        }
        self.write_str(word)
    }

    fn write_display_copyable(&mut self, t: impl std::fmt::Display + Copy) {
        use std::fmt::Write;
        write!(self.result, "{}", t).unwrap();
    }

    pub(crate) fn eager_head(
        &mut self,
        hir_eager_expr_region: HirEagerExprRegion,
        head: impl FnOnce(&mut RustTranspilationBuilder<HirEagerExprRegion>),
    ) {
        head(&mut RustTranspilationBuilder {
            base: self,
            extension: hir_eager_expr_region,
        })
    }

    pub(crate) fn eager_body(
        &mut self,
        hir_eager_expr_region: HirEagerExprRegion,
        body: HirEagerExprIdx,
    ) {
        (body, HirEagerExprSite::new_root(None)).transpile_to_rust(&mut RustTranspilationBuilder {
            base: self,
            extension: hir_eager_expr_region,
        })
    }

    pub(crate) fn rem_eulid(&mut self) {
        self.write_str("rem_euclid")
    }

    pub(crate) fn pow(&mut self) {
        self.write_str("pow")
    }

    pub(crate) fn zero(&mut self) {
        self.write_str("0")
    }
}

pub(crate) struct RustTranspilationBuilder<'a, 'b, E = ()> {
    base: &'b mut RustTranspilationBuilderBase<'a>,
    extension: E,
}

impl<'a, 'b, E> RustTranspilationBuilder<'a, 'b, E> {
    pub(crate) fn on_fresh_semicolon_line(&mut self, f: impl FnOnce(&mut Self)) {
        self.fresh_line();
        f(self);
        self.write_str(";")
    }

    pub(crate) fn on_fresh_line(&mut self, f: impl FnOnce(&mut Self)) {
        self.fresh_line();
        f(self);
    }

    pub(crate) fn on_fresh_paragraph(&mut self, f: impl FnOnce(&mut Self)) {
        self.fresh_paragraph();
        f(self)
    }

    pub(crate) fn bracketed(&mut self, bracket: RustBracket, f: impl FnOnce(&mut Self)) {
        self.write_str(bracket.bra_code());
        f(self);
        self.write_str(bracket.ket_code());
    }

    pub(crate) fn bracketed_list_with(&mut self, bracket: RustBracket, f: impl FnOnce(&mut Self)) {
        let is_list_start = std::mem::replace(&mut self.is_list_start, Some(true));
        self.write_str(bracket.bra_code());
        f(self);
        self.write_str(bracket.ket_code());
        self.is_list_start = is_list_start
    }

    pub(crate) fn curly_block(&mut self, f: impl FnOnce(&mut Self)) {
        if !(self.result.ends_with("\n") || self.result.ends_with(" ") || self.result.is_empty()) {
            self.result += " "
        }
        self.write_str("{");
        self.current_indent += INDENT_UNIT;
        f(self);
        self.current_indent -= INDENT_UNIT;
        if !self.result.ends_with("\n") {
            self.result += "\n"
        }
        self.write_indent();
        self.write_str("}");
    }

    pub(crate) fn heterogeneous_comma_list_items<A: TranspileToRustWith<E>>(
        &mut self,
        items: impl IntoIterator<Item = A>,
    ) {
        for item in items {
            self.heterogeneous_comma_list_item(item)
        }
    }

    pub(crate) fn heterogeneous_comma_list_item<A: TranspileToRustWith<E>>(&mut self, item: A) {
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

    pub(crate) fn bracketed_comma_list<A: TranspileToRustWith<E>>(
        &mut self,
        bracket: RustBracket,
        items: impl IntoIterator<Item = A>,
    ) {
        self.write_str(bracket.bra_code());
        self.punctuated_list(items, RustPunctuation::CommaSpaced);
        self.write_str(bracket.ket_code());
    }

    pub(crate) fn punctuated_list<A: TranspileToRustWith<E>>(
        &mut self,
        items: impl IntoIterator<Item = A>,
        punctuation: RustPunctuation,
    ) {
        let mut start = true;
        for item in items {
            if start {
                start = false
            } else {
                self.punctuation(punctuation)
            }
            item.transpile_to_rust(self)
        }
    }

    pub(crate) fn bracketed_multiline_comma_list<A: TranspileToRustWith<E>>(
        &mut self,
        bracket: RustBracket,
        items: impl IntoIterator<Item = A>,
    ) {
        self.write_str(bracket.bra_code());
        self.current_indent += INDENT_UNIT;
        for item in items {
            self.fresh_line();
            item.transpile_to_rust(self);
            self.write_str(",")
        }
        self.current_indent -= INDENT_UNIT;
        self.fresh_line();
        self.write_str(bracket.ket_code());
    }

    pub(crate) fn bracketed_multiline_list<A: TranspileToRustWith<E>>(
        &mut self,
        bracket: RustBracket,
        items: impl IntoIterator<Item = A>,
    ) {
        self.write_str(bracket.bra_code());
        self.current_indent += INDENT_UNIT;
        for item in items {
            self.fresh_line();
            item.transpile_to_rust(self);
        }
        self.current_indent -= INDENT_UNIT;
        self.fresh_line();
        self.write_str(bracket.ket_code());
    }
}

impl<'a, 'b> RustTranspilationBuilder<'a, 'b> {
    pub(crate) fn new(base: &'b mut RustTranspilationBuilderBase<'a>) -> Self {
        Self {
            base,
            extension: (),
        }
    }
}

impl<'a, 'b> RustTranspilationBuilder<'a, 'b, HirEagerExprRegion> {
    // todo: there is room for optimization
    pub(crate) fn hir_eager_expr_arena(&self) -> &'a HirEagerExprArena {
        self.extension.expr_arena(self.db)
    }

    // todo: there is room for optimization
    pub(crate) fn hir_eager_pattern_expr_arena(&self) -> &'a HirEagerPatternExprArena {
        self.extension.pattern_expr_arena(self.db)
    }

    // todo: there is room for optimization
    pub(crate) fn hir_eager_stmt_arena(&self) -> &'a HirEagerStmtArena {
        self.extension.stmt_arena(self.db)
    }

    fn hir_comptime_symbol(&mut self, symbol: impl Into<HirTemplateSymbol>) {
        let hir_comptime_symbol = symbol.into();
        let Some(symbol_name) = self
            .extension
            .comptime_symbol_region_data(self.db)
            .symbol_name(hir_comptime_symbol)
        else {
            let db = self.db;
            use salsa::DebugWithDb;
            p!(
                self.extension.path(db).debug(db),
                hir_comptime_symbol.debug(db),
                self.extension.comptime_symbol_region_data(db).debug(db)
            );
            todo!()
        };
        match symbol_name {
            HirEagerComptimeSymbolName::SelfType => self.word("Self"),
            HirEagerComptimeSymbolName::Ident(ident) => ident.transpile_to_rust(self),
            HirEagerComptimeSymbolName::Label(label) => label.transpile_to_rust(self),
        }
    }

    /// if `return_ty` is obviously a unit, this will do nothing,
    /// otherwise it will transcribe a `->` and the type
    pub(crate) fn return_ty(&mut self, return_ty: HirType) {
        let db = self.db;
        if !return_ty.is_core_basic_unit_obviously(db) {
            self.punctuation(RustPunctuation::LightArrow);
            return_ty.transpile_to_rust(self)
        }
    }
}

impl<'a, 'b> RustTranspilationBuilder<'a, 'b, HirLazyExprRegion> {
    // todo: there is room for optimization
    pub(crate) fn hir_lazy_expr_arena(&self) -> &'a HirLazyExprArena {
        self.extension.hir_lazy_expr_arena(self.db)
    }

    // todo: there is room for optimization
    pub(crate) fn hir_lazy_stmt_arena(&self) -> &'a HirLazyStmtArena {
        self.extension.hir_lazy_stmt_arena(self.db)
    }
}

impl<'a, 'b, E> std::ops::Deref for RustTranspilationBuilder<'a, 'b, E> {
    type Target = RustTranspilationBuilderBase<'a>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'a, 'b, E> std::ops::DerefMut for RustTranspilationBuilder<'a, 'b, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub(crate) trait TranspileToRustWith<E = ()> {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>);
}

impl<T, E> TranspileToRustWith<E> for Option<T>
where
    T: TranspileToRustWith<E>,
{
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        match self {
            Some(t) => t.transpile_to_rust(builder),
            None => (),
        }
    }
}

impl<T, E> TranspileToRustWith<E> for &T
where
    T: TranspileToRustWith<E> + Copy,
{
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        <T as TranspileToRustWith<E>>::transpile_to_rust(*self, builder)
    }
}

impl<T, E> TranspileToRustWith<E> for &[T]
where
    for<'a> &'a T: TranspileToRustWith<E>,
{
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        for t in self {
            t.transpile_to_rust(builder)
        }
    }
}

impl<E> TranspileToRustWith<E> for Ident {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db();
        builder.word(self.data(db))
    }
}

impl<E> TranspileToRustWith<E> for Label {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db();
        builder.write_str("'");
        builder.write_str(self.ident().data(db))
    }
}

impl<E> TranspileToRustWith<E> for HirTemplateSymbol {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        match self {
            HirTemplateSymbol::Type(symbol) => match symbol {
                HirTypeSymbol::Type {
                    attrs: _,
                    variance: _,
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

impl<'a, 'b, E> RustTranspilationBuilder<'a, 'b, E> {
    pub(crate) fn self_ty(&mut self) {
        self.word("Self")
    }

    pub(crate) fn self_value(&mut self) {
        self.word("self")
    }
}
impl TranspileToRustWith<HirEagerExprRegion> for HirEagerRuntimeSymbolIdx {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let db = builder.db;
        let hir_eager_runtime_symbol_region_data = builder.extension.runtime_symbol_region_data(db);
        if builder.result.ends_with(|c: char| c.is_alphabetic()) {
            builder.write_str(" ")
        }
        match hir_eager_runtime_symbol_region_data[self].name() {
            HirEagerRuntimeSymbolName::SelfValue => builder.word("self"),
            HirEagerRuntimeSymbolName::Ident(ident) => ident.transpile_to_rust(builder),
        }
    }
}
