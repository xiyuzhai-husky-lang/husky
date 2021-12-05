//! Implementation of find-usages functionality.
//!
//! It is based on the standard ide trick: first, we run a fast text search to
//! get a super-set of matches. Then, we we confirm each match using precise
//! name resolution.

use std::{convert::TryInto, mem};

use base_db::{FileID, FileRange, SourceDatabase, SourceDatabaseExt};
use hir::{AsAssocItem, InFile, ModuleSource, Semantics, Visibility};
use once_cell::unsync::Lazy;
use rustc_hash::FxHashMap;
use syntax::{ast, TextRange, TextSize};

use crate::{
    defs::{Definition, NameClass, NameRefClass},
    RootDatabase,
};

#[derive(Debug, Default, Clone)]
pub struct UsageSearchResult {
    pub references: FxHashMap<FileID, Vec<FileReference>>,
}

impl UsageSearchResult {
    pub fn is_empty(&self) -> bool {
        self.references.is_empty()
    }

    pub fn len(&self) -> usize {
        self.references.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&FileID, &[FileReference])> + '_ {
        self.references
            .iter()
            .map(|(file_id, refs)| (file_id, &**refs))
    }

    pub fn file_ranges(&self) -> impl Iterator<Item = FileRange> + '_ {
        self.references.iter().flat_map(|(&file_id, refs)| {
            refs.iter()
                .map(move |&FileReference { range, .. }| FileRange { file_id, range })
        })
    }
}

impl IntoIterator for UsageSearchResult {
    type Item = (FileID, Vec<FileReference>);
    type IntoIter = <FxHashMap<FileID, Vec<FileReference>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.references.into_iter()
    }
}

#[derive(Debug, Clone)]
pub struct FileReference {
    pub range: TextRange,
    pub name: ast::NameLike,
    pub category: Option<ReferenceCategory>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ReferenceCategory {
    // FIXME: Add this variant and delete the `retain_adt_literal_usages` function.
    // Create
    Write,
    Read,
    // FIXME: Some day should be able to search in doc comments. Would probably
    // need to switch from enum to bitflags then?
    // DocComment
}

/// Generally, `search_scope` returns files that might contain references for the element.
/// For `pub(crate)` things it's a crate, for `pub` things it's a crate and dependant crates.
/// In some cases, the location of the references is known to within a `TextRange`,
/// e.g. for things like local variables.
#[derive(Clone, Debug)]
pub struct SearchScope {
    entries: FxHashMap<FileID, Option<TextRange>>,
}

impl SearchScope {
    fn new(entries: FxHashMap<FileID, Option<TextRange>>) -> SearchScope {
        SearchScope { entries }
    }

    fn crate_graph(db: &RootDatabase) -> SearchScope {
        let mut entries = FxHashMap::default();

        let graph = db.crate_graph();
        for krate in graph.iter() {
            let root_file = graph[krate].root_file_id;
            let source_root_id = db.file_source_root(root_file);
            let source_root = db.source_root(source_root_id);
            entries.extend(source_root.iter().map(|id| (id, None)));
        }
        SearchScope { entries }
    }

    fn reverse_dependencies(db: &RootDatabase, of: hir::Crate) -> SearchScope {
        todo!()
    }

    fn krate(db: &RootDatabase, of: hir::Crate) -> SearchScope {
        todo!()
    }

    fn module(db: &RootDatabase, module: hir::Module) -> SearchScope {
        todo!()
    }

    pub fn empty() -> SearchScope {
        SearchScope::new(FxHashMap::default())
    }

    pub fn single_file(file: FileID) -> SearchScope {
        SearchScope::new(std::iter::once((file, None)).collect())
    }

    pub fn file_range(range: FileRange) -> SearchScope {
        SearchScope::new(std::iter::once((range.file_id, Some(range.range))).collect())
    }

    pub fn files(files: &[FileID]) -> SearchScope {
        SearchScope::new(files.iter().map(|f| (*f, None)).collect())
    }

    pub fn intersection(&self, other: &SearchScope) -> SearchScope {
        let (mut small, mut large) = (&self.entries, &other.entries);
        if small.len() > large.len() {
            mem::swap(&mut small, &mut large)
        }

        let res = small
            .iter()
            .filter_map(|(file_id, r1)| {
                let r2 = large.get(file_id)?;
                let r = intersect_ranges(*r1, *r2)?;
                Some((*file_id, r))
            })
            .collect();

        return SearchScope::new(res);

        fn intersect_ranges(
            r1: Option<TextRange>,
            r2: Option<TextRange>,
        ) -> Option<Option<TextRange>> {
            match (r1, r2) {
                (None, r) | (r, None) => Some(r),
                (Some(r1), Some(r2)) => {
                    let r = r1.intersect(r2)?;
                    Some(Some(r))
                }
            }
        }
    }
}

impl IntoIterator for SearchScope {
    type Item = (FileID, Option<TextRange>);
    type IntoIter = std::collections::hash_map::IntoIter<FileID, Option<TextRange>>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl Definition {
    fn search_scope(&self, db: &RootDatabase) -> SearchScope {
        todo!()
    }

    pub fn usages<'a>(self, sema: &'a Semantics<RootDatabase>) -> FindUsages<'a> {
        FindUsages {
            def: self,
            sema,
            scope: None,
            include_self_kw_refs: None,
            search_self_mod: false,
        }
    }
}

#[derive(Clone)]
pub struct FindUsages<'a> {
    def: Definition,
    sema: &'a Semantics<'a, RootDatabase>,
    scope: Option<SearchScope>,
    include_self_kw_refs: Option<hir::Type>,
    search_self_mod: bool,
}

impl<'a> FindUsages<'a> {
    /// Enable searching for `Self` when the definition is a type or `self` for modules.
    pub fn include_self_refs(mut self) -> FindUsages<'a> {
        self.include_self_kw_refs = def_to_ty(self.sema, &self.def);
        self.search_self_mod = true;
        self
    }

    pub fn in_scope(self, scope: SearchScope) -> FindUsages<'a> {
        self.set_scope(Some(scope))
    }

    pub fn set_scope(mut self, scope: Option<SearchScope>) -> FindUsages<'a> {
        assert!(self.scope.is_none());
        self.scope = scope;
        self
    }

    pub fn at_least_one(&self) -> bool {
        let mut found = false;
        self.search(&mut |_, _| {
            found = true;
            true
        });
        found
    }

    pub fn all(self) -> UsageSearchResult {
        let mut res = UsageSearchResult::default();
        self.search(&mut |file_id, reference| {
            res.references.entry(file_id).or_default().push(reference);
            false
        });
        res
    }

    fn search(&self, sink: &mut dyn FnMut(FileID, FileReference) -> bool) {
        todo!()
    }

    fn found_self_ty_name_ref(
        &self,
        self_ty: &hir::Type,
        name_ref: &ast::NameRef,
        sink: &mut dyn FnMut(FileID, FileReference) -> bool,
    ) -> bool {
        todo!()
    }

    fn found_self_module_name_ref(
        &self,
        name_ref: &ast::NameRef,
        sink: &mut dyn FnMut(FileID, FileReference) -> bool,
    ) -> bool {
        todo!()
    }

    fn found_name_ref(
        &self,
        name_ref: &ast::NameRef,
        sink: &mut dyn FnMut(FileID, FileReference) -> bool,
    ) -> bool {
        todo!()
    }

    fn found_name(
        &self,
        name: &ast::Name,
        sink: &mut dyn FnMut(FileID, FileReference) -> bool,
    ) -> bool {
        todo!()
    }
}

fn def_to_ty(sema: &Semantics<RootDatabase>, def: &Definition) -> Option<hir::Type> {
    todo!()
}

impl ReferenceCategory {
    fn new(def: &Definition, r: &ast::NameRef) -> Option<ReferenceCategory> {
        todo!()
    }
}
