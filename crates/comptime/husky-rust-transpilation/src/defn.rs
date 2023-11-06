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
use husky_hir_ty::{ritchie::HirRitchieParameter, HirTemplateSymbol, HirTypeSymbol};
use husky_print_utils::p;

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub fn module_defn_rust_transpilation(
    db: &dyn RustTranspilationDb,
    module_path: ModulePath,
) -> String {
    let mut builder = RustTranspilationBuilder::new(db);
    for item_path in module_item_paths(db, module_path)
        .as_ref()
        .expect("no error at this stage")
    {
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
    builder.finish()
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

impl TranspileToRust for HirTemplateParameter {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        // self.symbol().transpile_to_rust(builder)
        match self.data() {
            HirTemplateParameterData::Type { ident, traits } => {
                ident.transpile_to_rust(builder)
                // todo: traits
            }
            HirTemplateParameterData::Constant { ident, ty } => {
                use salsa::DebugWithDb;
                p!(ident.debug(builder.db()));
                todo!()
            }
            HirTemplateParameterData::Lifetime { label } => todo!(),
            HirTemplateParameterData::Place { label } => todo!(),
        }
    }
}

impl<'a> TranspileToRust for HirTemplateParameters {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        if self.is_empty() {
            return;
        }
        builder.bracketed_comma_list(RustBracket::Angle, self.as_ref())
    }
}

impl TranspileToRust for HirEagerSelfValueParameter {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        builder.self_value()
    }
}

impl TranspileToRust for HirEagerParenateParameter {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
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

impl TranspileToRust for HirEagerParenateParameters {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        builder.bracketed_comma_list(RustBracket::Par, self.iter());
    }
}

impl TranspileToRust for HirRitchieParameter {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder) {
        todo!()
    }
}
