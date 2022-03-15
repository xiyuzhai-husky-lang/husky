use common::*;
use file::FilePtr;

use std::sync::Arc;

use ast::{Ast, AstKind, AstResult, AstText};
use fold::{FoldIter, FoldIterItem, FoldStorage, FoldedList};
use scope::ScopePtr;

use crate::{config::*, error::*, *};

// currently dataset config is put in main file

#[salsa::query_group(ConfigQueryGroupStorage)]
pub trait ConfigQueryGroup: InferQueryGroup + Upcast<dyn InferQueryGroup> {
    fn config(&self, main_file: file::FilePtr) -> SemanticResultArc<Config>;
}

fn config(this: &dyn ConfigQueryGroup, main_file: file::FilePtr) -> SemanticResultArc<Config> {
    let ast_text = this.ast_text(main_file)?;
    config_from_ast(this, &ast_text, main_file)
}

fn config_from_ast(
    this: &dyn ConfigQueryGroup,
    ast_text: &AstText,
    file: FilePtr,
) -> SemanticResultArc<Config> {
    Ok(Arc::new(Config {
        dataset: dataset_config_from_ast_text(this, ast_text, file)?,
    }))
}

fn dataset_config_from_ast_text(
    this: &dyn ConfigQueryGroup,
    ast_text: &AstText,
    file: FilePtr,
) -> SemanticResult<DatasetConfig> {
    for item in ast_text.folded_results.fold_iter(0) {
        match item.value.as_ref()?.kind {
            AstKind::DatasetConfig => {
                return Ok(DatasetConfig::new(stmt::parse_decl_stmts(
                    &[],
                    this.upcast(),
                    &ast_text.arena,
                    not_none!(item.children),
                    file,
                )?))
            }
            _ => (),
        }
    }
    err!("dataset config not found")
}
