pub(crate) mod keyword;
mod macro_name;
mod punctuation;

pub(crate) use self::keyword::*;
pub(crate) use self::macro_name::*;
pub(crate) use self::punctuation::*;

use crate::*;
use husky_coword::Ident;
use husky_entity_path::{PreludeTypePath, PrincipalEntityPath, TypePath};
use husky_hir_eager_expr::{HirEagerExprArena, HirEagerPatternExprArena, HirEagerStmtArena};
use husky_hir_expr::HirExprRegion;
use husky_hir_lazy_expr::{HirLazyExprArena, HirLazyStmtArena};
use husky_hir_ty::{HirTemplateArgument, HirTemplateSymbol, HirType, HirTypeSymbol};
use husky_term_prelude::TermLiteral;

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

    pub(crate) fn on_new_semicolon_line(&mut self, f: impl FnOnce(&mut Self)) {
        if !self.result.ends_with("\n") {
            self.result += "\n"
        }
        self.write_indent();
        f(self);
        self.write_token_str(";")
    }

    pub(crate) fn on_new_line(&mut self, f: impl FnOnce(&mut Self)) {
        self.write_indent();
        f(self);
        self.write_token_str(";")
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

    fn display_copyable(&mut self, t: impl std::fmt::Display + Copy) {
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

    pub(crate) fn curly_block_with_hir_expr_region(
        &mut self,
        hir_expr_region: impl Into<HirExprRegion>,
        f: impl FnOnce(&mut Self),
    ) {
        self.curly_block(|builder| builder.with_hir_expr_region(hir_expr_region, f))
    }

    fn with_hir_expr_region(
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
            HirType::Symbol(_) => todo!(),
            HirType::TypeAssociatedType(_) => todo!(),
            HirType::TraitAssociatedType(_) => todo!(),
            HirType::Ritchie() => todo!(),
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
            HirTemplateArgument::Constant(_) => todo!(),
            HirTemplateArgument::Lifetime(_) => todo!(),
            HirTemplateArgument::Place(_) => todo!(),
        }
    }
}

impl TranspileToRust for TermLiteral {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        match self {
            TermLiteral::Unit => builder.write_str("()"),
            TermLiteral::Bool(value) => builder.display_copyable(value),
            TermLiteral::I8(value) => builder.display_copyable(value),
            TermLiteral::I16(value) => builder.display_copyable(value),
            TermLiteral::I32(value) => builder.display_copyable(value),
            TermLiteral::I64(lit) => builder.display_copyable(lit.value(db)),
            TermLiteral::I128(lit) => builder.display_copyable(lit.value(db)),
            TermLiteral::ISize(lit) => builder.display_copyable(lit.value(db)),
            TermLiteral::U8(value) => builder.display_copyable(value),
            TermLiteral::U16(value) => builder.display_copyable(value),
            TermLiteral::U32(value) => builder.display_copyable(value),
            TermLiteral::U64(lit) => builder.display_copyable(lit.value(db)),
            TermLiteral::U128(lit) => builder.display_copyable(lit.value(db)),
            TermLiteral::USize(lit) => builder.display_copyable(lit.value(db)),
            TermLiteral::R8(_) => todo!(),
            TermLiteral::R16(_) => todo!(),
            TermLiteral::R32(_) => todo!(),
            TermLiteral::R64(_) => todo!(),
            TermLiteral::R128(_) => todo!(),
            TermLiteral::RSize(_) => todo!(),
            TermLiteral::Nat(_) => todo!(),
            TermLiteral::F32(_) => todo!(),
            TermLiteral::F64(_) => todo!(),
            TermLiteral::String(_) => todo!(),
            TermLiteral::StaticLifetime => todo!(),
        }
    }
}

impl TranspileToRust for husky_opr::binary::BinaryOpr {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        // ad hoc
        builder.write_str(self.code())
    }
}
