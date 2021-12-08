//! This module provides `StaticIndex` which is used for powering
//! read-only code browsers and emitting LSIF

use std::collections::HashMap;

use common::*;

use hir::{db::HirDatabase, Crate, Module, Semantics};
use husky_lang_db::{
    defs::Definition,
    vfs::{FileId, FileRange},
    HuskyLangDatabase,
};
use rustc_hash::FxHashSet;
use syntax::{SyntaxKind::*, SyntaxToken};

use crate::{
    hover::hover_for_definition, Fold, HoverConfig, HoverDocFormat, HoverResult,
    IdeDatabaseSnapshot, TryToNav,
};

/// A static representation of fully analyzed source code.
///
/// The intended use-case is powering read-only code browsers and emitting LSIF
#[derive(Debug)]
pub struct StaticIndex<'a> {
    pub files: Vec<StaticIndexedFile>,
    pub tokens: TokenStore,
    analysis: &'a IdeDatabaseSnapshot,
    db: &'a HuskyLangDatabase,
    def_map: HashMap<Definition, TokenId>,
}

#[derive(Debug)]
pub struct ReferenceData {
    pub range: FileRange,
    pub is_definition: bool,
}

#[derive(Debug)]
pub struct TokenStaticData {
    pub hover: Option<HoverResult>,
    pub definition: Option<FileRange>,
    pub references: Vec<ReferenceData>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenId(usize);

impl TokenId {
    pub fn raw(self) -> usize {
        self.0
    }
}

#[derive(Default, Debug)]
pub struct TokenStore(Vec<TokenStaticData>);

impl TokenStore {
    pub fn insert(&mut self, data: TokenStaticData) -> TokenId {
        let id = TokenId(self.0.len());
        self.0.push(data);
        id
    }

    pub fn get_mut(&mut self, id: TokenId) -> Option<&mut TokenStaticData> {
        self.0.get_mut(id.0)
    }

    pub fn get(&self, id: TokenId) -> Option<&TokenStaticData> {
        self.0.get(id.0)
    }

    pub fn iter(self) -> impl Iterator<Item = (TokenId, TokenStaticData)> {
        self.0.into_iter().enumerate().map(|(i, x)| (TokenId(i), x))
    }
}

#[derive(Debug)]
pub struct StaticIndexedFile {
    pub file_id: FileId,
    pub folds: Vec<Fold>,
    pub tokens: Vec<(TextRange, TokenId)>,
}

fn all_modules(db: &dyn HirDatabase) -> Vec<Module> {
    todo!()
}

impl StaticIndex<'_> {
    fn add_file(&mut self, file_id: FileId) {
        todo!()
    }

    pub fn compute(analysis: &IdeDatabaseSnapshot) -> StaticIndex {
        todo!()
    }
}

fn get_definition(sema: &Semantics<HuskyLangDatabase>, token: SyntaxToken) -> Option<Definition> {
    todo!()
}
