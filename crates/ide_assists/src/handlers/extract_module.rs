use std::collections::{HashMap, HashSet};

use hir::{HasSource, ModuleSource};
use ide_db::{
    assists::{AssistId, AssistKind},
    base_db::FileID,
    defs::{Definition, NameClass, NameRefClass},
    search::{FileReference, SearchScope},
};
use stdx::format_to;
use syntax::{
    ast::{self, edit::IndentLevel},
    SingleFileParseTree, SyntaxNode, TextRange,
};

use crate::{AssistContext, Assists};

pub(crate) fn extract_module(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

#[derive(Debug)]
struct Module {
    text_range: TextRange,
    name: String,
    body_items: Vec<ast::Item>,
}

fn extract_target(node: &SyntaxNode, selection_range: TextRange) -> Option<Module> {
    todo!()
}

impl Module {
    fn get_usages_and_record_fields(
        &self,
        ctx: &AssistContext,
    ) -> (HashMap<FileID, Vec<(TextRange, String)>>, Vec<SyntaxNode>) {
        todo!()
    }

    fn expand_and_group_usages_file_wise(
        &self,
        ctx: &AssistContext,
        node_def: Definition,
        refs: &mut HashMap<FileID, Vec<(TextRange, String)>>,
    ) {
        for (file_id, references) in node_def.usages(&ctx.sema).all() {
            if let Some(file_refs) = refs.get_mut(&file_id) {
                let mut usages = self.expand_ref_to_usages(references, ctx, file_id);
                file_refs.append(&mut usages);
            } else {
                refs.insert(file_id, self.expand_ref_to_usages(references, ctx, file_id));
            }
        }
    }

    fn expand_ref_to_usages(
        &self,
        refs: Vec<FileReference>,
        ctx: &AssistContext,
        file_id: FileID,
    ) -> Vec<(TextRange, String)> {
        todo!()
    }

    fn get_usage_to_be_processed(
        &self,
        source_file: &SingleFileParseTree,
        FileReference { range, name, .. }: FileReference,
    ) -> Option<(TextRange, String)> {
        todo!()
    }

    fn change_visibility(&self, record_fields: Vec<SyntaxNode>) -> Option<Vec<ast::Item>> {
        todo!()
    }

    fn resolve_imports(
        &mut self,
        curr_parent_module: Option<ast::Module>,
        ctx: &AssistContext,
    ) -> Vec<TextRange> {
        todo!()
    }

    fn process_names_and_namerefs_for_import_resolve(
        &mut self,
        def: Definition,
        node_syntax: &SyntaxNode,
        curr_parent_module: &Option<ast::Module>,
        ctx: &AssistContext,
    ) -> Option<TextRange> {
        todo!()
    }

    fn make_use_stmt_of_node_with_super(&mut self, node_syntax: &SyntaxNode) {
        todo!()
    }

    fn process_use_stmt_for_import_resolve(
        &self,
        use_stmt_opt: Option<ast::Use>,
        node_syntax: &SyntaxNode,
    ) -> Option<(Vec<ast::Path>, Option<TextRange>)> {
        todo!()
    }
}

fn does_source_exists_outside_sel_in_same_mod(
    def: Definition,
    ctx: &AssistContext,
    curr_parent_module: &Option<ast::Module>,
    selection_range: TextRange,
    curr_file_id: FileID,
) -> bool {
    todo!()
}

fn get_replacements_for_visibilty_change(
    items: Vec<ast::Item>,
    is_clone_for_updated: bool,
) -> (
    Vec<ast::Item>,
    Vec<(Option<ast::Visibility>, SyntaxNode)>,
    Vec<(Option<ast::Visibility>, SyntaxNode)>,
    Vec<ast::Impl>,
) {
    todo!()
}

fn get_use_tree_paths_from_path(
    path: ast::Path,
    use_tree_str: &mut Vec<ast::Path>,
) -> Option<&mut Vec<ast::Path>> {
    todo!()
}

fn add_change_vis(
    vis: Option<ast::Visibility>,
    node_or_token_opt: Option<syntax::SyntaxElement>,
) -> Option<()> {
    todo!()
}

fn compare_hir_and_ast_module(
    ast_module: &ast::Module,
    hir_module: hir::Module,
    ctx: &AssistContext,
) -> Option<()> {
    todo!()
}
