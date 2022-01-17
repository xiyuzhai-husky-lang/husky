use common::*;

use std::sync::Arc;

use ast::AstText;
use fold::{FoldIterItem, FoldStorage};
use scope::ScopeId;

use crate::{config::*, *};

// currently dataset config is put in main file

#[salsa::query_group(ConfigQueryGroupStorage)]
pub trait ConfigQueryGroup: ast::AstQueryGroup {
    fn config(&self, main_file: file::FileId) -> SemanticResultArc<Config>;
}

fn config(this: &dyn ConfigQueryGroup, main_file: file::FileId) -> SemanticResultArc<Config> {
    let ast_text = this.ast_text(main_file)?;
    config_from_ast(&ast_text)
}

fn config_from_ast(ast_text: &AstText) -> SemanticResultArc<Config> {
    Ok(Arc::new(Config {
        dataset: dataset_config_from_ast(ast_text)?,
    }))
}

fn dataset_config_from_ast(ast_text: &AstText) -> SemanticResult<DatasetConfig> {
    p!(ast_text);
    todo!()
}
