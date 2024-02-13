use super::*;
use either::*;
use husky_entity_kind::ritchie::RitchieItemKind;
use husky_entity_path::PreludeIndirectionTypePath;
use husky_hir_ty::{instantiation::HirTermSvarResolution, ritchie::HirRitchieType, HirConstSvar};

impl TranspileToRustWith<HirEagerExprRegion> for HirType {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let db = builder.db;
        match self {
            HirType::PathLeading(path_leading_hir_ty) => {
                let template_arguments = path_leading_hir_ty.template_arguments(db);
                match path_leading_hir_ty.ty_path(db).refine(db) {
                    Left(PreludeTypePath::REF) => {
                        debug_assert_eq!(template_arguments.len(), 2);
                        builder.punctuation(RustPunctuation::Ambersand);
                        template_arguments[0].transpile_to_rust(builder);
                        template_arguments[1].transpile_to_rust(builder)
                    }
                    Left(PreludeTypePath::ARRAY) => {
                        debug_assert_eq!(template_arguments.len(), 2);
                        builder.bracketed(RustDelimiter::Box, |builder| {
                            template_arguments[1].transpile_to_rust(builder);
                            builder.punctuation(RustPunctuation::SemicolonInArray);
                            template_arguments[0].transpile_to_rust(builder);
                        })
                    }
                    Left(PreludeTypePath::SLICE) => {
                        debug_assert_eq!(template_arguments.len(), 1);
                        builder.bracketed(RustDelimiter::Box, |builder| {
                            template_arguments[0].transpile_to_rust(builder)
                        })
                    }
                    Left(PreludeTypePath::Indirection(PreludeIndirectionTypePath::Leash))
                        if let HirTemplateArgument::Type(HirType::PathLeading(inner_ty)) =
                            template_arguments[0]
                            && inner_ty.ty_path(db).refine(db)
                                == Left(PreludeTypePath::CYCLIC_SLICE) =>
                    {
                        debug_assert_eq!(template_arguments.len(), 1);
                        builder.cyclic_slice_leashed_ty();
                        builder.bracketed(RustDelimiter::Angle, |builder| {
                            inner_ty.template_arguments(db)[0].transpile_to_rust(builder)
                        })
                    }
                    _ => {
                        path_leading_hir_ty.ty_path(db).transpile_to_rust(builder);
                        if !template_arguments.is_empty() {
                            builder.bracketed_comma_list(RustDelimiter::Angle, template_arguments)
                        }
                    }
                }
            }
            HirType::Svar(svar) => builder.hir_template_svar(svar),
            HirType::TypeAssocType(_) => todo!(),
            HirType::TraitAssocType(_) => todo!(),
            HirType::Ritchie(hir_ritchie_ty) => hir_ritchie_ty.transpile_to_rust(builder),
        }
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirRitchieType {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let db = builder.db();
        builder.word(self.ritchie_ty_kind(db).code());
        builder.bracketed_comma_list(RustDelimiter::Par, self.parameters(db).iter());
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
            builder.bracketed_comma_list(RustDelimiter::Angle, template_arguments)
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

impl TranspileToRustWith<HirEagerExprRegion> for HirTermSvarResolution {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        match self {
            HirTermSvarResolution::Explicit(arg) => arg.transpile_to_rust(builder),
            HirTermSvarResolution::SelfLifetime => todo!(),
            HirTermSvarResolution::SelfPlace(_) => todo!(),
        }
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirConstant {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        match self {
            HirConstant::Unit(value) => builder.result += "()",
            HirConstant::Bool(value) => builder.write_display_copyable(value),
            HirConstant::Char(value) => builder.write_display_copyable(value),
            HirConstant::I8(value) => builder.write_display_copyable(value),
            HirConstant::I16(value) => builder.write_display_copyable(value),
            HirConstant::I32(value) => builder.write_display_copyable(value),
            HirConstant::I64(value) => builder.write_display_copyable(value),
            HirConstant::I128(value) => builder.write_display_copyable(value),
            HirConstant::ISize(value) => builder.write_display_copyable(value),
            HirConstant::U8(value) => builder.write_display_copyable(value),
            HirConstant::U16(value) => builder.write_display_copyable(value),
            HirConstant::U32(value) => builder.write_display_copyable(value),
            HirConstant::U64(value) => builder.write_display_copyable(value),
            HirConstant::U128(value) => builder.write_display_copyable(value),
            HirConstant::USize(value) => builder.write_display_copyable(value),
            HirConstant::R8(value) => builder.write_display_copyable(value),
            HirConstant::R16(value) => builder.write_display_copyable(value),
            HirConstant::R32(value) => builder.write_display_copyable(value),
            HirConstant::R64(value) => builder.write_display_copyable(value),
            HirConstant::R128(value) => builder.write_display_copyable(value),
            HirConstant::RSize(value) => builder.write_display_copyable(value),
            HirConstant::Symbol(symbol) => builder.hir_template_svar(symbol),
            HirConstant::TypeVariant(path) => path.transpile_to_rust(builder),
            HirConstant::StaticLifetime => todo!(),
        }
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirConstSvar {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        builder.hir_template_svar(self)
    }
}
