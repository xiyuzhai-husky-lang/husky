use crate::{
    code_lens::{module_code_lenses, CodeLens},
    *,
};
use husky_vfs::path::module_path::ModulePath;

pub fn module_lsp_code_lenses_unresolved(
    module_path: ModulePath,
    db: &::salsa::Db,
) -> Vec<lsp_types::CodeLens> {
    let code_lenses = module_code_lenses(db, module_path);
    code_lenses
        .iter()
        .map(|code_lens| code_lens.lsp_unresolved())
        .collect()
}

impl CodeLens {
    fn lsp_unresolved(&self) -> lsp_types::CodeLens {
        todo!()
    }

    fn lsp_resolved(&self) -> lsp_types::CodeLens {
        todo!()
    }
}

#[test]
fn module_lsp_code_lenses_unresolved_works() {
    DB::ast_rich_test_debug(
        |module_path, db| module_lsp_code_lenses_unresolved(db, module_path),
        &AstTestConfig::new(
            "module_lsp_code_lens_unresolved",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::IDE,
        ),
    );
}
