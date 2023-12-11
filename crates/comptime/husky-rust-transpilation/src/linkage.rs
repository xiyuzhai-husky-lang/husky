use crate::*;
use husky_corgi_config::transpilation_setup::TranspilationSetup;
use husky_javelin::{javelin::JavelinData, path::JavelinPath};
use husky_linkage::linkage::{package_linkages, Linkage, LinkageData};
use salsa::DebugWithDb;

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn package_linkages_transpilation(
    db: &::salsa::Db,
    package_path: PackagePath,
    setup: TranspilationSetup,
) -> String {
    let mut builder_base = RustTranspilationBuilderBase::new(
        db,
        package_path.toolchain(db),
        setup,
        Some(format!(
            r#"use husky_core::*;
use {}::*;
use {}::*;
"#,
            setup.rust_data(db).unwrap().task_dependency_ident.data(db),
            package_path.ident(db).data(db)
        )),
    );
    let mut builder = RustTranspilationBuilder::new(&mut builder_base);
    builder.on_fresh_semicolon_paragraph(|builder| {
        builder.rustfmt_skip();
        builder.macro_name(RustMacroName::Linkages);
        builder.bracketed_multiline_comma_list(RustBracket::Box, package_linkages(db, package_path))
    });
    builder_base.finish()
}

impl TranspileToRustWith<()> for Linkage {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<()>) {
        let db = builder.db;
        match *self.data(db) {
            LinkageData::FunctionFnItem(path) => path.transpile_to_rust(builder),
            LinkageData::ValItem(path) => path.transpile_to_rust(builder),
            LinkageData::TypeMethodFn(path) => path.transpile_to_rust(builder),
            LinkageData::TypeConstructor(ty_path) => builder.ty_constructor(ty_path),
            LinkageData::TypeAssociatedFunctionFn(path) => path.transpile_to_rust(builder),
            LinkageData::TraitForTypeMethodFn(path) => path.transpile_to_rust(builder),
            LinkageData::TraitForTypeAssociatedFunctionFn(path) => path.transpile_to_rust(builder),
        }
    }
}

impl TranspileToRustWith<()> for JavelinPath {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<()>) {
        match self {
            JavelinPath::Fugitive(slf) => slf.transpile_to_rust(builder),
            JavelinPath::TypeItem(slf) => slf.transpile_to_rust(builder),
            JavelinPath::TraitItem(slf) => slf.transpile_to_rust(builder),
            JavelinPath::TraitForTypeItem(slf) => slf.transpile_to_rust(builder),
            JavelinPath::TypeConstructor(slf) => slf.transpile_to_rust(builder),
            JavelinPath::TypeVariantConstructor(slf) => slf.transpile_to_rust(builder),
        }
    }
}
