mod associated_item;
mod attr;
mod impl_block;
mod major_item;
mod submodule;
mod ty_variant;

use crate::*;
use husky_entity_syn_tree::helpers::paths::module_item_paths;
use husky_hir_decl::parameter::{
    parenate::eager::{HirEagerParenateParameter, HirEagerParenateParameters},
    self_value::eager::HirEagerSelfValueParameter,
    template::{HirTemplateParameter, HirTemplateParameterData, HirTemplateParameters},
};
use husky_hir_defn::*;
use husky_hir_eager_expr::HirEagerExprRegion;
use husky_hir_ty::ritchie::HirRitchieParameter;
use husky_print_utils::p;
use husky_vfs::ModulePathData;

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn module_defn_rust_transpilation(
    db: &dyn RustTranspilationDb,
    module_path: ModulePath,
) -> String {
    let mut builder_base = RustTranspilationBuilderBase::new(
        db,
        match module_path.data(db) {
            ModulePathData::Root(_) => Some(
                r#"\
#[cfg(feature = "__linkages")]
mod __linkages;
"#,
            ),
            ModulePathData::Child { .. } => None,
        },
    );
    let mut builder: RustTranspilationBuilder = RustTranspilationBuilder::new(&mut builder_base);
    for item_path in module_item_paths(db, module_path) {
        if let Some(hir_defn) = item_path.hir_defn(db) {
            match hir_defn {
                HirDefn::Submodule(_) | HirDefn::MajorItem(_) => {
                    builder.make_defn_fresh_lines();
                    hir_defn.transpile_to_rust(&mut builder)
                }
                _ => (),
            }
        }
    }
    builder_base.finish()
}

#[test]
fn module_defn_rust_transpilation_works() {
    DB::default().ast_expect_test_display(
        |db, module_path| crate::defn::module_defn_rust_transpilation(db, module_path).to_string(),
        &AstTestConfig::new("module_defn_rust_transpilation")
            .with_vfs_test_domains_config(VfsTestDomainsConfig::ExcludeLibrary)
            .with_expect_file_extension("rs"),
    );
}

impl TranspileToRust for HirDefn {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        match self {
            HirDefn::Submodule(hir_defn) => hir_defn.transpile_to_rust(builder),
            HirDefn::MajorItem(hir_defn) => hir_defn.transpile_to_rust(builder),
            HirDefn::TypeVariant(hir_defn) => hir_defn.transpile_to_rust(builder),
            HirDefn::ImplBlock(hir_defn) => hir_defn.transpile_to_rust(builder),
            HirDefn::AssociatedItem(hir_defn) => hir_defn.transpile_to_rust(builder),
            HirDefn::Attr(hir_defn) => hir_defn.transpile_to_rust(builder),
        }
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirTemplateParameter {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        // self.symbol().transpile_to_rust(builder)
        match self.data() {
            HirTemplateParameterData::Type { ident, traits: _ } => {
                ident.transpile_to_rust(builder)
                // todo: traits
            }
            HirTemplateParameterData::Constant { ident, ty: _ } => {
                use salsa::DebugWithDb;
                p!(ident.debug(builder.db()));
                todo!()
            }
            HirTemplateParameterData::Lifetime { label: _ } => todo!(),
            HirTemplateParameterData::Place { label: _ } => todo!(),
        }
    }
}

impl<'a> TranspileToRust<HirEagerExprRegion> for HirTemplateParameters {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        if self.is_empty() {
            return;
        }
        builder.bracketed_comma_list(RustBracket::Angle, self.as_ref())
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirEagerSelfValueParameter {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        builder.self_value()
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirEagerParenateParameter {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        match self {
            HirEagerParenateParameter::Ordinary {
                pattern_expr_idx,
                ty,
            } => {
                pattern_expr_idx.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Colon);
                ty.transpile_to_rust(builder)
            }
            HirEagerParenateParameter::Keyed => todo!(),
            HirEagerParenateParameter::Variadic => todo!(),
        }
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirEagerParenateParameters {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        builder.bracketed_comma_list(RustBracket::Par, self.iter());
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirRitchieParameter {
    fn transpile_to_rust(&self, _builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        todo!()
    }
}
