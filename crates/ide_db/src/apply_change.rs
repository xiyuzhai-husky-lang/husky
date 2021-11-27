//! Applies changes to the IDE state transactionally.

use std::sync::Arc;

use base_db::{
    salsa::{Database, Durability},
    Change, SourceRootId,
};
use profile::{memory_usage, Bytes};
use rustc_hash::FxHashSet;

use crate::{symbol_index::SymbolsDatabase, RootDatabase};

impl RootDatabase {
    pub fn request_cancellation(&mut self) {
        let _p = profile::span("RootDatabase::request_cancellation");
        self.salsa_runtime_mut().synthetic_write(Durability::LOW);
    }

    pub fn apply_change(&mut self, change: Change) {
        let _p = profile::span("RootDatabase::apply_change");
        self.request_cancellation();
        tracing::info!("apply_change {:?}", change);
        if let Some(roots) = &change.roots {
            let mut local_roots = FxHashSet::default();
            let mut library_roots = FxHashSet::default();
            for (idx, root) in roots.iter().enumerate() {
                let root_id = SourceRootId(idx as u32);
                if root.is_library {
                    library_roots.insert(root_id);
                } else {
                    local_roots.insert(root_id);
                }
            }
            self.set_local_roots_with_durability(Arc::new(local_roots), Durability::HIGH);
            self.set_library_roots_with_durability(Arc::new(library_roots), Durability::HIGH);
        }
        change.apply(self);
    }

    // Feature: Memory Usage
    //
    // Clears rust-analyzer's internal database and prints memory usage statistics.
    //
    // |===
    // | Editor  | Action Name
    //
    // | VS Code | **Rust Analyzer: Memory Usage (Clears Database)**
    // |===
    // image::https://user-images.githubusercontent.com/48062697/113065592-08559f00-91b1-11eb-8c96-64b88068ec02.gif[]
    pub fn per_query_memory_usage(&mut self) -> Vec<(String, Bytes)> {
        let mut acc: Vec<(String, Bytes)> = vec![];
        macro_rules! purge_each_query {
            ($($q:path)*) => {$(
                let before = memory_usage().allocated;
                $q.in_db(self).purge();
                let after = memory_usage().allocated;
                let q: $q = Default::default();
                let name = format!("{:?}", q);
                acc.push((name, before - after));
            )*}
        }
        purge_each_query![
            // SourceDatabase
            base_db::ParseQuery
            base_db::CrateGraphQuery

            // SourceDatabaseExt
            base_db::FileTextQuery
            base_db::FileSourceRootQuery
            base_db::SourceRootQuery
            base_db::SourceRootCratesQuery

            // AstDatabase
            semantics::db::AstIdMapQuery
            semantics::db::MacroArgTextQuery
            semantics::db::MacroDefQuery
            semantics::db::ParseMacroExpansionQuery
            semantics::db::MacroExpandQuery
            semantics::db::HygieneFrameQuery
            semantics::db::InternMacroCallQuery

            // DefDatabase
            semantics::db::FileItemTreeQuery
            semantics::db::BlockDefMapQuery
            semantics::db::CrateDefMapQueryQuery
            semantics::db::FieldsAttrsQuery
            semantics::db::VariantsAttrsQuery
            semantics::db::FieldsAttrsSourceMapQuery
            semantics::db::VariantsAttrsSourceMapQuery
            semantics::db::StructDataQuery
            semantics::db::UnionDataQuery
            semantics::db::EnumDataQuery
            semantics::db::ImplDataQuery
            semantics::db::TraitDataQuery
            semantics::db::TypeAliasDataQuery
            semantics::db::FunctionDataQuery
            semantics::db::ConstDataQuery
            semantics::db::StaticDataQuery
            semantics::db::BodyWithSourceMapQuery
            semantics::db::BodyQuery
            semantics::db::ExprScopesQuery
            semantics::db::GenericParamsQuery
            semantics::db::AttrsQuery
            semantics::db::CrateLangItemsQuery
            semantics::db::LangItemQuery
            semantics::db::ImportMapQuery

            // HirDatabase
            semantics::db::InferQueryQuery
            semantics::db::TyQuery
            semantics::db::ValueTyQuery
            semantics::db::ImplSelfTyQuery
            semantics::db::ImplTraitQuery
            semantics::db::FieldTypesQuery
            semantics::db::CallableItemSignatureQuery
            semantics::db::GenericPredicatesForParamQuery
            semantics::db::GenericPredicatesQuery
            semantics::db::GenericDefaultsQuery
            semantics::db::InherentImplsInCrateQuery
            semantics::db::TraitEnvironmentQuery
            semantics::db::TraitImplsInCrateQuery
            semantics::db::TraitImplsInDepsQuery
            semantics::db::AssociatedTyDataQuery
            semantics::db::AssociatedTyDataQuery
            semantics::db::TraitDatumQuery
            semantics::db::StructDatumQuery
            semantics::db::ImplDatumQuery
            semantics::db::FnDefDatumQuery
            semantics::db::ReturnTypeImplTraitsQuery
            semantics::db::InternCallableDefQuery
            semantics::db::InternTypeParamIdQuery
            semantics::db::InternImplTraitIdQuery
            semantics::db::InternClosureQuery
            semantics::db::AssociatedTyValueQuery
            semantics::db::TraitSolveQueryQuery
            semantics::db::InternTypeParamIdQuery

            // SymbolsDatabase
            crate::symbol_index::FileSymbolsQuery
            crate::symbol_index::LibrarySymbolsQuery
            crate::symbol_index::LocalRootsQuery
            crate::symbol_index::LibraryRootsQuery

            // LineIndexDatabase
            crate::LineIndexQuery

            // InternDatabase
            semantics::db::InternFunctionQuery
            semantics::db::InternStructQuery
            semantics::db::InternUnionQuery
            semantics::db::InternEnumQuery
            semantics::db::InternConstQuery
            semantics::db::InternStaticQuery
            semantics::db::InternTraitQuery
            semantics::db::InternTypeAliasQuery
            semantics::db::InternImplQuery
        ];

        acc.sort_by_key(|it| std::cmp::Reverse(it.1));
        acc
    }
}
