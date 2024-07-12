use crate::{
    code_lens::{module_code_lenses, CodeLens},
    *,
};
use code_lens::CodeLensData;
use husky_entity_path::region::RegionPath;
use husky_entity_tree::helpers::tokra_region::HasRegionalTokenIdxBase;
use husky_sem_static_mut_deps::item_sem_static_mut_deps;
use husky_sem_static_var_deps::item_sem_static_var_deps;
use husky_token::{RangedTokenSheet, TokenDb};
use husky_vfs::path::module_path::ModulePath;
use salsa::DisplayWithDb;

pub fn module_lsp_code_lenses_unresolved(
    module_path: ModulePath,
    db: &::salsa::Db,
) -> Vec<lsp_types::CodeLens> {
    let code_lenses = module_code_lenses(db, module_path);
    let ranged_token_sheet = db.ranged_token_sheet(module_path);
    code_lenses
        .iter()
        .map(|code_lens| code_lens.lsp_unresolved(ranged_token_sheet, db))
        .collect()
}

impl CodeLens {
    fn lsp_unresolved(
        &self,
        ranged_token_sheet: &RangedTokenSheet,
        db: &::salsa::Db,
    ) -> lsp_types::CodeLens {
        lsp_types::CodeLens {
            range: self.range(ranged_token_sheet, db),
            command: Some(self.lsp_command(db)),
            data: None,
        }
    }

    fn lsp_resolved(&self) -> lsp_types::CodeLens {
        match *self.data() {
            CodeLensData::Deps => todo!(),
            CodeLensData::Affect => todo!(),
            CodeLensData::Runnable => todo!(),
            CodeLensData::HasImpls => todo!(),
            CodeLensData::HasReferences => todo!(),
        }
    }

    fn range(&self, ranged_token_sheet: &RangedTokenSheet, db: &::salsa::Db) -> lsp_types::Range {
        let region_path = RegionPath::ItemDecl(self.item_path());
        let regional_token_idx_base = region_path.regional_token_idx_base(db).unwrap();
        let token_idx_range = regional_token_idx_base.token_idx_range();
        let text_range = ranged_token_sheet.tokens_text_range(token_idx_range);
        text_range.into()
    }

    fn lsp_command(&self, db: &::salsa::Db) -> lsp_types::Command {
        match *self.data() {
            CodeLensData::Deps => {
                let mut title = "#deps(".to_string();
                let mut empty = true;
                for _ in item_sem_static_mut_deps(self.item_path(), db) {
                    if empty {
                        empty = false
                    } else {
                        title.push_str(", ");
                    }
                    todo!()
                }
                for item_path in item_sem_static_var_deps(self.item_path(), db) {
                    if empty {
                        empty = false
                    } else {
                        title.push_str(", ");
                    }
                    title.push_str(&item_path.display_with(db).to_string());
                }
                title.push(')');
                lsp_types::Command {
                    title,
                    command: "husky-analyzer.deps".to_string(),
                    arguments: None,
                }
            }
            CodeLensData::Affect => todo!(),
            CodeLensData::Runnable => todo!(),
            CodeLensData::HasImpls => todo!(),
            CodeLensData::HasReferences => todo!(),
        }
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
