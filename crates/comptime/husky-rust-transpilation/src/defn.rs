mod associated_item;
mod attr;
mod impl_block;
mod major_item;
mod submodule;
mod ty_variant;

use crate::*;
use husky_corgi_config::transpilation_setup::TranspilationSetup;
use husky_entity_kind::{
    AssociatedItemKind, EntityKind, FugitiveKind, MajorItemKind, TraitItemKind, TypeItemKind,
};
use husky_entity_path::{ItemPath, MajorItemPath};
use husky_entity_syn_tree::helpers::paths::{
    crate_module_paths, module_item_paths, module_submodule_item_paths, HasModulePaths,
};
use husky_hir_decl::parameter::{
    parenate::eager::{HirEagerParenateParameter, HirEagerParenateParameters},
    self_value::eager::HirEagerSelfValueParameter,
    template::{HirTemplateParameter, HirTemplateParameterData, HirTemplateParameters},
};
use husky_hir_defn::*;
use husky_hir_eager_expr::HirEagerExprRegion;
use husky_hir_ty::ritchie::{HirEagerContract, HirRitchieParameter, HirRitchieRegularParameter};
use husky_manifest::HasPackageManifest;
use husky_vfs::ModulePathData;

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn module_defn_rust_transpilation(
    db: &::salsa::Db,
    module_path: ModulePath,
    setup: TranspilationSetup,
) -> String {
    let is_root = module_path.is_root(db);
    let result = if is_root {
        Some(format!(
            r#"#![allow(warnings, non_snake_case)]
use husky_core::*;
use {}::{{*, ugly::*}};

{}::init_crate!();

"#,
            setup.rust_data(db).unwrap().task_dependency_ident.data(db),
            setup.rust_data(db).unwrap().task_dependency_ident.data(db)
        ))
    } else {
        None
    };
    let mut builder_base = RustTranspilationBuilderBase::new(
        db,
        module_path.toolchain(db),
        setup,
        result,
        Some(module_path.crate_path(db)),
    );
    let mut builder: RustTranspilationBuilder = RustTranspilationBuilder::new(&mut builder_base);
    let submodule_paths = module_submodule_item_paths(db, module_path);
    // declare submodules
    for submodule_path in submodule_paths {
        submodule_path.hir_defn(db).transpile_to_rust(&mut builder)
    }
    // pub use all in submodules
    builder.on_fresh_paragraph(|builder| {
        for &submodule_path in submodule_paths {
            builder.pub_use_all_in_submodule(submodule_path)
        }
    });
    builder.on_fresh_paragraph(|builder| match module_path.data(db) {
        // use all in dependencies if root
        ModulePathData::Root(crate_path) => builder.on_fresh_paragraph(|builder| {
            for dep in crate_path.package_path(db).dependencies(db).unwrap() {
                builder.use_all_in_dep(dep)
            }
        }),
        ModulePathData::Child { parent, .. } => match parent.data(db) {
            // use all in crate if second to root
            ModulePathData::Root(_) => builder.use_all_in_crate(),
            // use in super otherwise
            ModulePathData::Child { .. } => builder.use_all_in_super(),
        },
    });
    for item_path in module_item_paths(db, module_path) {
        match item_path.item_kind(db) {
            EntityKind::MajorItem {
                module_item_kind: MajorItemKind::Fugitive(FugitiveKind::FunctionGn),
                ..
            } => continue,
            _ => (),
        };
        if let Some(hir_defn) = item_path.hir_defn(db) {
            match hir_defn {
                HirDefn::MajorItem(hir_defn) => {
                    builder.on_fresh_paragraph(|builder| {
                        builder.rustfmt_skip();
                        hir_defn.transpile_to_rust(builder)
                    });
                }
                HirDefn::ImplBlock(hir_defn) => {
                    builder.on_fresh_paragraph(|builder| hir_defn.transpile_to_rust(builder));
                }
                _ => (),
            }
        }
    }
    builder_base.finish()
}

#[test]
fn module_defn_rust_transpilation_works() {
    let db = &mut DB::default();
    db.ast_expect_test_display(
        |db, module_path: ModulePath| -> String {
            let package_path = module_path.package_path(db);
            if package_path.is_virtual(db) {
                return "// virtual package".to_string();
            }
            let setup = TranspilationSetup::new_ad_hoc(package_path, db);
            crate::defn::module_defn_rust_transpilation(db, module_path, setup).to_string()
        },
        &AstTestConfig::new("module_defn_rust_transpilation")
            .with_vfs_test_domains_config(VfsTestDomainsConfig::ExcludeLibrary)
            .with_expect_file_extension("rs"),
    );
}

impl TranspileToRustWith for HirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        todo!();
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

impl TranspileToRustWith<HirEagerExprRegion> for &HirTemplateParameter {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        match self.data() {
            HirTemplateParameterData::Type { ident, traits } => {
                ident.transpile_to_rust(builder);
                if traits.len() > 0 {
                    builder.punctuation(RustPunctuation::Colon);
                    builder.punctuated_list(traits, RustPunctuation::Add)
                }
            }
            HirTemplateParameterData::Constant { ident, ty: _ } => {
                // ad hoc
                // we should skip #runtime constants
                // use salsa::DebugWithDb;
                // p!(ident.debug(builder.db()));
                // todo!()
            }
            HirTemplateParameterData::Lifetime { label: _ } => todo!(),
            HirTemplateParameterData::Place { label: _ } => todo!(),
        }
    }
}

impl<'a> TranspileToRustWith<HirEagerExprRegion> for &HirTemplateParameters {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        if self.is_empty() {
            return;
        }
        builder.bracketed_comma_list(RustBracket::Angle, self.as_ref())
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirEagerSelfValueParameter {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let db = builder.db();
        let HirEagerSelfValueParameter { contract, self_ty } = self;
        match contract {
            HirEagerContract::Pure => {
                if !self_ty.always_copyable(db) {
                    builder.punctuation(RustPunctuation::Ambersand)
                }
            }
            HirEagerContract::Move => (),
            HirEagerContract::Borrow => builder.punctuation(RustPunctuation::Ambersand),
            HirEagerContract::BorrowMut => {
                builder.punctuation(RustPunctuation::Ambersand);
                builder.keyword(RustKeyword::Mut)
            }
            HirEagerContract::Const => todo!(),
            HirEagerContract::Leash => todo!(),
            HirEagerContract::At => todo!(),
        }
        builder.self_value()
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirEagerParenateParameter {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let db = builder.db();
        match self {
            HirEagerParenateParameter::Ordinary {
                pattern_expr_idx,
                contract,
                ty,
            } => {
                pattern_expr_idx.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Colon);
                match contract {
                    HirEagerContract::Pure => match ty.always_copyable(db) {
                        true => (),
                        false => builder.punctuation(RustPunctuation::Ambersand),
                    },
                    HirEagerContract::Move => (),
                    HirEagerContract::Borrow => builder.punctuation(RustPunctuation::Ambersand),
                    HirEagerContract::BorrowMut => {
                        builder.punctuation(RustPunctuation::Ambersand);
                        builder.keyword(RustKeyword::Mut)
                    }
                    HirEagerContract::Const => todo!(),
                    HirEagerContract::Leash => todo!(),
                    HirEagerContract::At => todo!(),
                }
                ty.transpile_to_rust(builder)
            }
            HirEagerParenateParameter::Keyed => todo!(),
            HirEagerParenateParameter::Variadic => todo!(),
        }
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for &HirEagerParenateParameters {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        builder.bracketed_comma_list(RustBracket::Par, self.iter());
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirRitchieParameter {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        match self {
            HirRitchieParameter::Ordinary(param) => param.transpile_to_rust(builder),
            HirRitchieParameter::Variadic(_) => todo!(),
            HirRitchieParameter::Keyed(_) => todo!(),
        }
    }
}

impl TranspileToRustWith<HirEagerExprRegion> for HirRitchieRegularParameter {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        self.ty().transpile_to_rust(builder)
    }
}
