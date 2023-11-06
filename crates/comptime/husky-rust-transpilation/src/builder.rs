pub(crate) mod keyword;
mod literal;
mod macro_name;
mod opr;
mod path;
mod punctuation;

pub(crate) use self::keyword::*;
pub(crate) use self::macro_name::*;
pub(crate) use self::punctuation::*;

use crate::{expr::RustPrecedence, *};
use husky_coword::Ident;
use husky_entity_path::{PreludeTypePath, PrincipalEntityPath, TypePath};
use husky_hir_eager_expr::{HirEagerExprArena, HirEagerPatternExprArena, HirEagerStmtArena};
use husky_hir_expr::HirExprRegion;
use husky_hir_lazy_expr::{HirLazyExprArena, HirLazyStmtArena};
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_hir_ty::{HirConstant, HirTemplateArgument, HirTemplateSymbol, HirType, HirTypeSymbol};
use husky_term_prelude::TermLiteral;

const INDENT_UNIT: u32 = 4;

pub(crate) struct RustTranspilationBuilder<'a> {
    db: &'a dyn RustTranspilationDb,
    result: String,
    current_indent: u32,
    is_list_start: Option<bool>,
    hir_expr_region: Option<HirExprRegion>,
    spaced: bool,
}

impl<'a> RustTranspilationBuilder<'a> {
    pub(crate) fn new(db: &'a dyn RustTranspilationDb) -> Self {
        Self {
            db,
            result: Default::default(),
            current_indent: 0,
            is_list_start: None,
            hir_expr_region: None,
            spaced: true,
        }
    }

    pub(crate) fn finish(self) -> String {
        self.result
    }

    pub(crate) fn result(&self) -> &str {
        &self.result
    }

    pub(crate) fn db(&self) -> &'a dyn RustTranspilationDb {
        self.db
    }

    pub(crate) fn on_new_semicolon_line(&mut self, f: impl FnOnce(&mut Self)) {
        self.make_fresh_line();
        f(self);
        self.write_str(";")
    }

    pub(crate) fn on_new_line(&mut self, f: impl FnOnce(&mut Self)) {
        self.make_fresh_line();
        f(self);
    }

    fn make_fresh_line(&mut self) {
        if !self.fresh_line() {
            self.result += "\n";
            self.write_indent();
        }
    }

    pub(crate) fn make_defn_fresh_lines(&mut self) {
        if !self.fresh_line() {
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
        self.result += "// ";
        self.result += s
    }

    fn write_indent(&mut self) {
        for _ in 0..self.current_indent {
            self.result.push(' ')
        }
    }

    pub(crate) fn fresh_line(&self) -> bool {
        self.result.is_empty() || self.result.ends_with("\n")
    }

    fn write_str(&mut self, s: &str) {
        self.result += s
    }

    fn write_display_copyable(&mut self, t: impl std::fmt::Display + Copy) {
        use std::fmt::Write;
        write!(self.result, "{}", t);
    }

    pub(crate) fn bracketed_comma_list<A: TranspileToRust>(
        &mut self,
        bracket: RustBracket,
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
        bracket: RustBracket,
        items: impl FnOnce(&mut Self),
    ) {
        let is_list_start = std::mem::replace(&mut self.is_list_start, Some(true));
        self.write_str(bracket.bra_code());
        items(self);
        self.write_str(bracket.ket_code());
        self.is_list_start = is_list_start
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

    pub(crate) fn curly_block_with_hir_eager_expr_region(
        &mut self,
        hir_expr_region: impl Into<HirExprRegion>,
        f: impl FnOnce(&mut Self),
    ) {
        self.curly_block(|builder| builder.with_hir_expr_region(hir_expr_region, f))
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

    // todo: there is room for optimization
    pub(crate) fn hir_eager_expr_arena(&self) -> &'a HirEagerExprArena {
        let Some(HirExprRegion::Eager(hir_eager_expr_region)) = self.hir_expr_region else {
            unreachable!()
        };
        hir_eager_expr_region.hir_eager_expr_arena(self.db)
    }

    // todo: there is room for optimization
    pub(crate) fn hir_eager_pattern_expr_arena(&self) -> &'a HirEagerPatternExprArena {
        let Some(HirExprRegion::Eager(hir_eager_expr_region)) = self.hir_expr_region else {
            unreachable!()
        };
        hir_eager_expr_region.hir_eager_pattern_expr_arena(self.db)
    }

    // todo: there is room for optimization
    pub(crate) fn hir_eager_stmt_arena(&self) -> &'a HirEagerStmtArena {
        let Some(HirExprRegion::Eager(hir_eager_expr_region)) = self.hir_expr_region else {
            unreachable!()
        };
        hir_eager_expr_region.hir_eager_stmt_arena(self.db)
    }

    // todo: there is room for optimization
    pub(crate) fn hir_lazy_expr_arena(&self) -> &'a HirLazyExprArena {
        let Some(HirExprRegion::Lazy(hir_lazy_expr_region)) = self.hir_expr_region else {
            unreachable!()
        };
        hir_lazy_expr_region.hir_lazy_expr_arena(self.db)
    }

    // todo: there is room for optimization
    pub(crate) fn hir_lazy_stmt_arena(&self) -> &'a HirLazyStmtArena {
        let Some(HirExprRegion::Lazy(hir_lazy_expr_region)) = self.hir_expr_region else {
            unreachable!()
        };
        hir_lazy_expr_region.hir_lazy_stmt_arena(self.db)
    }

    pub(crate) fn rem_eulid(&mut self) {
        self.write_str("rem_eulicd")
    }

    pub(crate) fn pow(&mut self) {
        self.write_str("pow")
    }

    pub(crate) fn zero(&mut self) {
        self.write_str("0")
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

impl<T> TranspileToRust for Option<T>
where
    T: TranspileToRust,
{
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self {
            Some(t) => t.transpile_to_rust(builder),
            None => (),
        }
    }
}

impl TranspileToRust for Ident {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        if builder.result.ends_with(|c: char| c.is_alphabetic()) {
            builder.write_str(" ")
        }
        builder.write_str(self.data(builder.db()))
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

impl TranspileToRust for HirType {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db;
        match self {
            HirType::PathLeading(path_leading_hir_ty) => {
                path_leading_hir_ty.ty_path(db).transpile_to_rust(builder);
                let template_arguments = path_leading_hir_ty.template_arguments(db);
                if !template_arguments.is_empty() {
                    builder.bracketed_comma_list(RustBracket::Angle, template_arguments)
                }
            }
            HirType::Symbol(_) => builder.write_str(" HirTypeSymbolTodo "),
            HirType::TypeAssociatedType(_) => todo!(),
            HirType::TraitAssociatedType(_) => todo!(),
            HirType::Ritchie() => builder.write_str(" HirTypeRitchieTodo "),
        }
    }
}

impl TranspileToRust for PrincipalEntityPath {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db;
        match self {
            PrincipalEntityPath::Module(path) => path.ident(db).transpile_to_rust(builder),
            PrincipalEntityPath::MajorItem(path) => path.ident(db).transpile_to_rust(builder),
            PrincipalEntityPath::TypeVariant(path) => {
                match path.parent_ty_path(db).prelude_ty_path(db) {
                    Some(PreludeTypePath::Option | PreludeTypePath::Result) => (),
                    _ => {
                        path.parent_ty_path(db).ident(db).transpile_to_rust(builder);
                        builder.punctuation(RustPunctuation::ColonColon);
                    }
                }
                path.ident(db).transpile_to_rust(builder)
            }
        }
    }
}

impl TranspileToRust for TypePath {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        self.ident(db).transpile_to_rust(builder)
    }
}

impl TranspileToRust for HirTemplateArgument {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self {
            HirTemplateArgument::Vacant => todo!(),
            HirTemplateArgument::Type(hir_ty) => hir_ty.transpile_to_rust(builder),
            HirTemplateArgument::Constant(hir_constant) => hir_constant.transpile_to_rust(builder),
            HirTemplateArgument::Lifetime(_) => todo!(),
            HirTemplateArgument::Place(_) => todo!(),
        }
    }
}

impl TranspileToRust for HirConstant {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self {
            HirConstant::Unit(_) => todo!(),
            HirConstant::Bool(_) => todo!(),
            HirConstant::Char(_) => todo!(),
            HirConstant::I8(_) => todo!(),
            HirConstant::I16(_) => todo!(),
            HirConstant::I32(_) => todo!(),
            HirConstant::I64(_) => todo!(),
            HirConstant::I128(_) => todo!(),
            HirConstant::ISize(_) => todo!(),
            HirConstant::U8(_) => todo!(),
            HirConstant::U16(_) => todo!(),
            HirConstant::U32(_) => todo!(),
            HirConstant::U64(_) => todo!(),
            HirConstant::U128(_) => todo!(),
            HirConstant::USize(value) => builder.write_display_copyable(value),
            HirConstant::R8(_) => todo!(),
            HirConstant::R16(_) => todo!(),
            HirConstant::R32(_) => todo!(),
            HirConstant::R64(_) => todo!(),
            HirConstant::R128(_) => todo!(),
            HirConstant::RSize(_) => todo!(),
            HirConstant::Symbol(_) => builder.write_str("HirConstantSymbolTodo"),
        }
    }
}

impl TranspileToRust for TermLiteral {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        match self {
            TermLiteral::Unit => builder.write_str("()"),
            TermLiteral::Bool(value) => builder.write_display_copyable(value),
            TermLiteral::I8(value) => builder.write_display_copyable(value),
            TermLiteral::I16(value) => builder.write_display_copyable(value),
            TermLiteral::I32(value) => builder.write_display_copyable(value),
            TermLiteral::I64(lit) => builder.write_display_copyable(lit.value(db)),
            TermLiteral::I128(lit) => builder.write_display_copyable(lit.value(db)),
            TermLiteral::ISize(lit) => builder.write_display_copyable(lit.value(db)),
            TermLiteral::U8(value) => builder.write_display_copyable(value),
            TermLiteral::U16(value) => builder.write_display_copyable(value),
            TermLiteral::U32(value) => builder.write_display_copyable(value),
            TermLiteral::U64(lit) => builder.write_display_copyable(lit.value(db)),
            TermLiteral::U128(lit) => builder.write_display_copyable(lit.value(db)),
            TermLiteral::USize(lit) => builder.write_display_copyable(lit.value(db)),
            TermLiteral::R8(_) => todo!(),
            TermLiteral::R16(_) => todo!(),
            TermLiteral::R32(value) => builder.write_display_copyable(value),
            TermLiteral::R64(_) => todo!(),
            TermLiteral::R128(_) => todo!(),
            TermLiteral::RSize(_) => todo!(),
            TermLiteral::Nat(_) => todo!(),
            TermLiteral::F32(value) => builder.write_display_copyable(value.into_inner()),
            TermLiteral::F64(_) => todo!(),
            TermLiteral::String(lit) => {
                use std::fmt::Write;
                write!(builder.result, "{:?}", lit.data(db)).unwrap();
            }
            TermLiteral::StaticLifetime => todo!(),
        }
    }
}

impl<'a> RustTranspilationBuilder<'a> {
    pub(crate) fn self_ty(&mut self) {
        self.write_str("Self")
    }

    pub(crate) fn self_value(&mut self) {
        self.write_str("self")
    }
}
