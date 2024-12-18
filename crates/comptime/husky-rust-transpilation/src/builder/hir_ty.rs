use super::*;
use either::*;
use husky_entity_kind::ritchie::RitchieItemKind;
use husky_entity_path::path::major_item::ty::{PreludeIndirectionTypePath, PreludeTypePath};
use husky_hir_ty::{
    instantiation::HirTermSymbolicVariableResolution, ritchie::HirRitchieType,
    HirComptermTemplateVariable,
};
use husky_term_prelude::ritchie::RitchieTypeKind;

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
                        builder.delimited(RustDelimiter::Box, |builder| {
                            template_arguments[1].transpile_to_rust(builder);
                            builder.punctuation(RustPunctuation::SemicolonInArray);
                            template_arguments[0].transpile_to_rust(builder);
                        })
                    }
                    Left(PreludeTypePath::SLICE) => {
                        debug_assert_eq!(template_arguments.len(), 1);
                        builder.delimited(RustDelimiter::Box, |builder| {
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
                        builder.delimited(RustDelimiter::Angle, |builder| {
                            inner_ty.template_arguments(db)[0].transpile_to_rust(builder)
                        })
                    }
                    _ => {
                        path_leading_hir_ty.ty_path(db).transpile_to_rust(builder);
                        if !template_arguments.is_empty() {
                            builder.delimited_comma_list(RustDelimiter::Angle, template_arguments)
                        }
                    }
                }
            }
            HirType::Variable(template_variable) => {
                builder.hir_template_variable(template_variable)
            }
            HirType::TypeAssocType(_) => todo!(),
            HirType::TraitAssocType(_) => todo!(),
            HirType::Ritchie(hir_ritchie_ty) => hir_ritchie_ty.transpile_to_rust(builder),
            HirType::TypeVar(_) => todo!(),
            HirType::Quaried => todo!(),
        }
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirRitchieType {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let db = builder.db();
        match self.ritchie_ty_kind(db) {
            RitchieTypeKind::Item(ritchie_ty_kind) => builder.word(ritchie_ty_kind.display_str()),
            RitchieTypeKind::Closure(_) => unreachable!(),
        }
        builder.delimited_comma_list(RustDelimiter::Par, self.parameters(db).iter());
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
            builder.delimited_comma_list(RustDelimiter::Angle, template_arguments)
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
            HirTemplateArgument::ContractedQuary(_) => todo!(),
        }
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirTermSymbolicVariableResolution {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        match self {
            HirTermSymbolicVariableResolution::Explicit(arg) => arg.transpile_to_rust(builder),
            HirTermSymbolicVariableResolution::SelfLifetime => todo!(),
            HirTermSymbolicVariableResolution::SelfContractedQuary(_) => todo!(),
        }
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirCompterm {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        match self {
            HirCompterm::Unit(value) => builder.result += "()",
            HirCompterm::Bool(value) => builder.write_display_copyable(value),
            HirCompterm::Char(value) => builder.write_display_copyable(value),
            HirCompterm::I8(value) => builder.write_display_copyable(value),
            HirCompterm::I16(value) => builder.write_display_copyable(value),
            HirCompterm::I32(value) => builder.write_display_copyable(value),
            HirCompterm::I64(value) => builder.write_display_copyable(value),
            HirCompterm::I128(value) => builder.write_display_copyable(value),
            HirCompterm::ISize(value) => builder.write_display_copyable(value),
            HirCompterm::U8(value) => builder.write_display_copyable(value),
            HirCompterm::U16(value) => builder.write_display_copyable(value),
            HirCompterm::U32(value) => builder.write_display_copyable(value),
            HirCompterm::U64(value) => builder.write_display_copyable(value),
            HirCompterm::U128(value) => builder.write_display_copyable(value),
            HirCompterm::USize(value) => builder.write_display_copyable(value),
            HirCompterm::R8(value) => builder.write_display_copyable(value),
            HirCompterm::R16(value) => builder.write_display_copyable(value),
            HirCompterm::R32(value) => builder.write_display_copyable(value),
            HirCompterm::R64(value) => builder.write_display_copyable(value),
            HirCompterm::R128(value) => builder.write_display_copyable(value),
            HirCompterm::RSize(value) => builder.write_display_copyable(value),
            HirCompterm::Symbol(symbol) => builder.hir_template_variable(symbol),
            HirCompterm::TypeVariant(path) => path.transpile_to_rust(builder),
            HirCompterm::StaticLifetime => todo!(),
        }
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirComptermTemplateVariable {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        builder.hir_template_variable(self)
    }
}
