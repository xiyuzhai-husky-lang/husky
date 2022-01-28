use common::*;

use std::sync::Arc;

use ast::{Ast, AstResult, AstText};
use fold::{FoldIter, FoldIterItem, FoldStorage, FoldedList};
use scope::ScopeId;

use crate::{config::*, error::*, *};

// currently dataset config is put in main file

#[salsa::query_group(ConfigQueryGroupStorage)]
pub trait ConfigQueryGroup: InferQueryGroup + Upcast<dyn InferQueryGroup> {
    fn config(&self, main_file: file::FileId) -> SemanticResultArc<Config>;
}

fn config(this: &dyn ConfigQueryGroup, main_file: file::FileId) -> SemanticResultArc<Config> {
    let ast_text = this.ast_text(main_file)?;
    config_from_ast(this, &ast_text)
}

fn config_from_ast(this: &dyn ConfigQueryGroup, ast_text: &AstText) -> SemanticResultArc<Config> {
    Ok(Arc::new(Config {
        dataset: dataset_config_from_ast_text(this, ast_text)?,
    }))
}

fn dataset_config_from_ast_text(
    this: &dyn ConfigQueryGroup,
    ast_text: &AstText,
) -> SemanticResult<DatasetConfig> {
    for item in ast_text.folded_results.fold_iter(0) {
        match item.value.as_ref()? {
            Ast::DatasetConfig => {
                return Ok(DatasetConfig::new(stmt::parse_lazy_stmts(
                    this.upcast(),
                    &ast_text.arena,
                    not_none!(item.children),
                )?))
            }
            _ => (),
        }
    }
    err!()
}
