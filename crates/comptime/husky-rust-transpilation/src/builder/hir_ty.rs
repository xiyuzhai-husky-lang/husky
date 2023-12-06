use either::*;
use husky_entity_path::PreludeContainerTypePath;
use husky_hir_ty::ritchie::{HirRitchieParameter, HirRitchieType};
use husky_term_prelude::RitchieTypeKind;

use super::*;

impl TranspileToRustWith<HirEagerExprRegion> for HirType {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let db = builder.db;
        match self {
            HirType::PathLeading(path_leading_hir_ty) => {
                let template_arguments = path_leading_hir_ty.template_arguments(db);
                match path_leading_hir_ty.ty_path(db).refine(db) {
                    Left(PreludeTypePath::Container(PreludeContainerTypePath::Array)) => {
                        debug_assert_eq!(template_arguments.len(), 2);
                        builder.bracketed(RustBracket::Box, |builder| {
                            template_arguments[1].transpile_to_rust(builder);
                            builder.punctuation(RustPunctuation::SemicolonInArray);
                            template_arguments[0].transpile_to_rust(builder);
                        })
                    }
                    Left(PreludeTypePath::Container(PreludeContainerTypePath::Slice)) => {
                        debug_assert_eq!(template_arguments.len(), 1);
                        builder.bracketed(RustBracket::Box, |builder| {
                            template_arguments[0].transpile_to_rust(builder)
                        })
                    }
                    _ => {
                        path_leading_hir_ty.ty_path(db).transpile_to_rust(builder);
                        if !template_arguments.is_empty() {
                            builder.bracketed_comma_list(RustBracket::Angle, template_arguments)
                        }
                    }
                }
            }
            HirType::Symbol(symbol) => builder.hir_comptime_symbol(symbol),
            HirType::TypeAssociatedType(_) => todo!(),
            HirType::TraitAssociatedType(_) => todo!(),
            HirType::Ritchie(hir_ritchie_ty) => hir_ritchie_ty.transpile_to_rust(builder),
        }
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirRitchieType {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let db = builder.db();
        match self.ritchie_ty_kind(db) {
            RitchieTypeKind::Fn => builder.word("fn"),
            RitchieTypeKind::Gn => builder.word("gn"),
        }
        builder.bracketed_comma_list(RustBracket::Par, self.parameters(db).iter());
        builder.punctuation(RustPunctuation::LightArrow);
        self.return_ty(db).transpile_to_rust(builder)
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirTrait {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let db = builder.db;
        self.trai_path(db).transpile_to_rust(builder);
        let template_arguments = self.template_arguments(db);
        if !template_arguments.is_empty() {
            builder.bracketed_comma_list(RustBracket::Angle, template_arguments)
        }
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirTemplateArgument {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        match self {
            HirTemplateArgument::Vacant => todo!(),
            HirTemplateArgument::Type(hir_ty) => hir_ty.transpile_to_rust(builder),
            HirTemplateArgument::Constant(hir_constant) => hir_constant.transpile_to_rust(builder),
            HirTemplateArgument::Lifetime(_) => todo!(),
            HirTemplateArgument::Place(_) => todo!(),
        }
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirConstant {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
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
            HirConstant::Symbol(symbol) => builder.hir_comptime_symbol(symbol),
        }
    }
}
