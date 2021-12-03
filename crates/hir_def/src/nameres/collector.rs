//! The core of the module-level name resolution algorithm.
//!
//! `DefCollector::collect` contains the fixed-point iteration loop which
//! resolves imports and expands macros.

use std::iter;

use arena::Idx;
use base_db::{CrateId, Edition, FileID, ProcMacroId};
use hir_expand::{
    builtin_attr_macro::find_builtin_attr,
    builtin_derive_macro::find_builtin_derive,
    builtin_fn_macro::find_builtin_macro,
    name::{name, AsName, Name},
    proc_macro::ProcMacroExpander,
    ExpandTo, HirFileID, MacroCallId, MacroCallKind, MacroDefId, MacroDefKind,
};
use hir_expand::{InFile, MacroCallLoc};
use itertools::Itertools;
use limit::Limit;
use rustc_hash::{FxHashMap, FxHashSet};
use syntax::{ast, SmolStr};

use crate::{
    attr::{Attr, AttrId, Attrs},
    builtin_attr,
    db::DefDatabase,
    intern::Interned,
    item_scope::{ImportType, PerNsGlobImports},
    item_tree::{
        self, Fields, FileItemTreeId, ImportKind, ItemTree, ItemTreeId, MacroCall, MacroDef,
        MacroRules, Mod, ModItem, ModKind, TreeId,
    },
    nameres::{
        diagnostics::DefDiagnostic,
        mod_resolution::ModDir,
        path_resolution::ReachedFixedPoint,
        proc_macro::{ProcMacroDef, ProcMacroKind},
        BuiltinShadowMode, DefMap, ModuleData, ModuleOrigin, ResolveMode,
    },
    path::{ImportAlias, ModPath, PathKind},
    per_ns::PerNamespace,
    visibility::{RawVisibility, Visibility},
    AdtId, AstId, AstIdWithPath, ConstLoc, EnumLoc, EnumVariantId, FunctionLoc, ImplLoc, Intern,
    LocalModuleId, ModuleDefId, StaticLoc, StructLoc, TraitLoc, TypeAliasLoc, UnionLoc,
};

static GLOB_RECURSION_LIMIT: Limit = Limit::new(100);
static EXPANSION_DEPTH_LIMIT: Limit = Limit::new(128);
static FIXED_POINT_LIMIT: Limit = Limit::new(8192);

pub(super) fn collect_defs(
    db: &dyn DefDatabase,
    mut def_map: DefMap,
    block: Option<AstId<ast::BlockExpr>>,
) -> DefMap {
    let crate_graph = db.crate_graph();

    let mut deps = FxHashMap::default();
    // populate external prelude and dependency list
    for dep in &crate_graph[def_map.krate].dependencies {
        tracing::debug!("crate dep {:?} -> {:?}", dep.name, dep.crate_id);
        let dep_def_map = db.crate_def_map(dep.crate_id);
        let dep_root = dep_def_map.module_id(dep_def_map.root);

        deps.insert(dep.as_name(), dep_root.into());

        if dep.is_prelude() && block.is_none() {
            def_map
                .extern_prelude
                .insert(dep.as_name(), dep_root.into());
        }
    }

    let mut collector = DefCollector {
        db,
        def_map,
        deps,
        glob_imports: FxHashMap::default(),
        unresolved_imports: Vec::new(),
        resolved_imports: Vec::new(),
        mod_dirs: FxHashMap::default(),
        from_glob_import: Default::default(),
        skip_attrs: Default::default(),
        derive_helpers_in_scope: Default::default(),
        registered_attrs: Default::default(),
        registered_tools: Default::default(),
    };
    match block {
        Some(block) => collector.seed_with_inner(block),
        None => collector.seed_with_top_level(),
    }
    collector.collect();
    let mut def_map = collector.def_map;
    def_map.shrink_to_fit();
    def_map
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum PartialResolvedImport {
    /// None of any namespaces is resolved
    Unresolved,
    /// One of namespaces is resolved
    Indeterminate(PerNamespace),
    /// All namespaces are resolved, OR it comes from other crate
    Resolved(PerNamespace),
}

impl PartialResolvedImport {
    fn namespaces(self) -> PerNamespace {
        match self {
            PartialResolvedImport::Unresolved => PerNamespace::none(),
            PartialResolvedImport::Indeterminate(ns) | PartialResolvedImport::Resolved(ns) => ns,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ImportSource {
    Import {
        id: ItemTreeId<item_tree::Import>,
        use_tree: Idx<ast::UseTree>,
    },
    ExternCrate(ItemTreeId<item_tree::ExternCrate>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Import {
    path: Interned<ModPath>,
    alias: Option<ImportAlias>,
    visibility: RawVisibility,
    kind: ImportKind,
    is_prelude: bool,
    is_extern_crate: bool,
    is_macro_use: bool,
    source: ImportSource,
}

impl Import {
    fn from_use(
        db: &dyn DefDatabase,
        krate: CrateId,
        tree: &ItemTree,
        id: ItemTreeId<item_tree::Import>,
    ) -> Vec<Self> {
        let it = &tree[id.value];
        let attrs = &tree.attrs(db, krate, ModItem::from(id.value).into());
        let visibility = &tree[it.visibility];
        let is_prelude = attrs.by_key("prelude_import").exists();

        let mut res = Vec::new();
        it.use_tree.expand(|idx, path, kind, alias| {
            res.push(Self {
                path: Interned::new(path), // FIXME this makes little sense
                alias,
                visibility: visibility.clone(),
                kind,
                is_prelude,
                is_extern_crate: false,
                is_macro_use: false,
                source: ImportSource::Import { id, use_tree: idx },
            });
        });
        res
    }

    fn from_extern_crate(
        db: &dyn DefDatabase,
        krate: CrateId,
        tree: &ItemTree,
        id: ItemTreeId<item_tree::ExternCrate>,
    ) -> Self {
        let it = &tree[id.value];
        let attrs = &tree.attrs(db, krate, ModItem::from(id.value).into());
        let visibility = &tree[it.visibility];
        Self {
            path: Interned::new(ModPath::from_segments(
                PathKind::Plain,
                iter::once(it.name.clone()),
            )),
            alias: it.alias.clone(),
            visibility: visibility.clone(),
            kind: ImportKind::Plain,
            is_prelude: false,
            is_extern_crate: true,
            is_macro_use: attrs.by_key("macro_use").exists(),
            source: ImportSource::ExternCrate(id),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ImportDirective {
    module_id: LocalModuleId,
    import: Import,
    status: PartialResolvedImport,
}

/// Walks the tree of module recursively
struct DefCollector<'a> {
    db: &'a dyn DefDatabase,
    def_map: DefMap,
    deps: FxHashMap<Name, ModuleDefId>,
    glob_imports: FxHashMap<LocalModuleId, Vec<(LocalModuleId, Visibility)>>,
    unresolved_imports: Vec<ImportDirective>,
    resolved_imports: Vec<ImportDirective>,
    mod_dirs: FxHashMap<LocalModuleId, ModDir>,
    /// List of procedural macros defined by this crate. This is read from the dynamic library
    /// built by the build system, and is the list of proc. macros we can actually expand. It is
    /// empty when proc. macro support is disabled (in which case we still do name resolution for
    /// them).
    from_glob_import: PerNsGlobImports,
    /// If we fail to resolve an attribute on a `ModItem`, we fall back to ignoring the attribute.
    /// This map is used to skip all attributes up to and including the one that failed to resolve,
    /// in order to not expand them twice.
    ///
    /// This also stores the attributes to skip when we resolve derive helpers and non-macro
    /// non-builtin attributes in general.
    skip_attrs: FxHashMap<InFile<ModItem>, AttrId>,
    /// Tracks which custom derives are in scope for an item, to allow resolution of derive helper
    /// attributes.
    derive_helpers_in_scope: FxHashMap<AstId<ast::Item>, Vec<Name>>,
    /// Custom attributes registered with `#![register_attr]`.
    registered_attrs: Vec<SmolStr>,
    /// Custom tool modules registered with `#![register_tool]`.
    registered_tools: Vec<SmolStr>,
}

impl DefCollector<'_> {
    fn seed_with_top_level(&mut self) {
        todo!()
    }

    fn seed_with_inner(&mut self, block: AstId<ast::BlockExpr>) {
        todo!()
    }

    fn resolution_loop(&mut self) {
        let _p = profile::span("DefCollector::resolution_loop");

        // main name resolution fixed-point loop.
        let mut i = 0;
        'outer: loop {
            loop {
                self.db.unwind_if_cancelled();
                {
                    let _p = profile::span("resolve_imports loop");
                    loop {
                        if self.resolve_imports() == ReachedFixedPoint::Yes {
                            break;
                        }
                    }
                }

                i += 1;
                if FIXED_POINT_LIMIT.check(i).is_err() {
                    tracing::error!("name resolution is stuck");
                    break 'outer;
                }
            }
        }
    }

    fn collect(&mut self) {
        let _p = profile::span("DefCollector::collect");

        self.resolution_loop();

        // Resolve all indeterminate resolved imports again
        // As some of the macros will expand newly import shadowing partial resolved imports
        // FIXME: We maybe could skip this, if we handle the indeterminate imports in `resolve_imports`
        // correctly
        let partial_resolved = self.resolved_imports.iter().filter_map(|directive| {
            if let PartialResolvedImport::Indeterminate(_) = directive.status {
                let mut directive = directive.clone();
                directive.status = PartialResolvedImport::Unresolved;
                Some(directive)
            } else {
                None
            }
        });
        self.unresolved_imports.extend(partial_resolved);
        self.resolve_imports();

        let unresolved_imports = std::mem::take(&mut self.unresolved_imports);
        // show unresolved imports in completion, etc
        for directive in &unresolved_imports {
            self.record_resolved_import(directive)
        }
        self.unresolved_imports = unresolved_imports;
    }

    fn inject_prelude(&mut self, crate_attrs: &Attrs) {
        // See compiler/rustc_builtin_macros/src/standard_library_imports.rs

        if crate_attrs.by_key("no_core").exists() {
            // libcore does not get a prelude.
            return;
        }

        let krate = if crate_attrs.by_key("no_std").exists() {
            name![core]
        } else {
            let std = name![std];
            if self.def_map.extern_prelude().any(|(name, _)| *name == std) {
                std
            } else {
                // If `std` does not exist for some reason, fall back to core. This mostly helps
                // keep r-a's own tests minimal.
                name![core]
            }
        };

        let edition = match self.def_map.edition {
            Edition::Edition2015 => name![rust_2015],
            Edition::Edition2018 => name![rust_2018],
            Edition::Edition2021 => name![rust_2021],
        };

        let path_kind = if self.def_map.edition == Edition::Edition2015 {
            PathKind::Plain
        } else {
            PathKind::Abs
        };
        let path = ModPath::from_segments(
            path_kind.clone(),
            [krate.clone(), name![prelude], edition].into_iter(),
        );
        // Fall back to the older `std::prelude::v1` for compatibility with Rust <1.52.0
        // FIXME remove this fallback
        let fallback_path =
            ModPath::from_segments(path_kind, [krate, name![prelude], name![v1]].into_iter());

        for path in &[path, fallback_path] {
            let (per_ns, _) = self.def_map.resolve_path(
                self.db,
                self.def_map.root,
                path,
                BuiltinShadowMode::Other,
            );

            match per_ns.types {
                Some((ModuleDefId::ModuleId(m), _)) => {
                    self.def_map.prelude = Some(m);
                    return;
                }
                types => {
                    tracing::debug!(
                        "could not resolve prelude path `{}` to module (resolved to {:?})",
                        path,
                        types
                    );
                }
            }
        }
    }

    /// Tries to resolve every currently unresolved import.
    fn resolve_imports(&mut self) -> ReachedFixedPoint {
        let mut res = ReachedFixedPoint::Yes;
        let imports = std::mem::take(&mut self.unresolved_imports);
        let imports = imports
            .into_iter()
            .filter_map(|mut directive| {
                directive.status = self.resolve_import(directive.module_id, &directive.import);
                match directive.status {
                    PartialResolvedImport::Indeterminate(_) => {
                        self.record_resolved_import(&directive);
                        // FIXME: For avoid performance regression,
                        // we consider an imported resolved if it is indeterminate (i.e not all namespace resolved)
                        self.resolved_imports.push(directive);
                        res = ReachedFixedPoint::No;
                        None
                    }
                    PartialResolvedImport::Resolved(_) => {
                        self.record_resolved_import(&directive);
                        self.resolved_imports.push(directive);
                        res = ReachedFixedPoint::No;
                        None
                    }
                    PartialResolvedImport::Unresolved => Some(directive),
                }
            })
            .collect();
        self.unresolved_imports = imports;
        res
    }

    fn resolve_import(&self, module_id: LocalModuleId, import: &Import) -> PartialResolvedImport {
        let _p = profile::span("resolve_import").detail(|| format!("{}", import.path));
        tracing::debug!(
            "resolving import: {:?} ({:?})",
            import,
            self.def_map.edition
        );
        if import.is_extern_crate {
            let name = import
                .path
                .as_ident()
                .expect("extern crate should have been desugared to one-element path");

            let res = self.resolve_extern_crate(name);

            if res.is_none() {
                PartialResolvedImport::Unresolved
            } else {
                PartialResolvedImport::Resolved(res)
            }
        } else {
            let res = self.def_map.resolve_path_fp_with_macro(
                self.db,
                ResolveMode::Import,
                module_id,
                &import.path,
                BuiltinShadowMode::Module,
            );

            let def = res.resolved_def;
            if res.reached_fixedpoint == ReachedFixedPoint::No || def.is_none() {
                return PartialResolvedImport::Unresolved;
            }

            if let Some(krate) = res.krate {
                if krate != self.def_map.krate {
                    return PartialResolvedImport::Resolved(
                        def.filter_visibility(|v| matches!(v, Visibility::Public)),
                    );
                }
            }

            // Check whether all namespace is resolved
            if def.take_types().is_some() && def.take_values().is_some() {
                PartialResolvedImport::Resolved(def)
            } else {
                PartialResolvedImport::Indeterminate(def)
            }
        }
    }

    fn resolve_extern_crate(&self, name: &Name) -> PerNamespace {
        if *name == name!(self) {
            cov_mark::hit!(extern_crate_self_as);
            let root = match self.def_map.block {
                Some(_) => {
                    let def_map = self.def_map.crate_root(self.db).def_map(self.db);
                    def_map.module_id(def_map.root())
                }
                None => self.def_map.module_id(self.def_map.root()),
            };
            PerNamespace::types(root.into(), Visibility::Public)
        } else {
            self.deps.get(name).map_or(PerNamespace::none(), |&it| {
                PerNamespace::types(it, Visibility::Public)
            })
        }
    }

    fn record_resolved_import(&mut self, directive: &ImportDirective) {
        let _p = profile::span("record_resolved_import");

        let module_id = directive.module_id;
        let import = &directive.import;
        let mut def = directive.status.namespaces();
        let vis = self
            .def_map
            .resolve_visibility(self.db, module_id, &directive.import.visibility)
            .unwrap_or(Visibility::Public);

        match import.kind {
            ImportKind::Plain | ImportKind::TypeOnly => {
                let name = match &import.alias {
                    Some(ImportAlias::Alias(name)) => Some(name),
                    Some(ImportAlias::Underscore) => None,
                    None => match import.path.segments().last() {
                        Some(last_segment) => Some(last_segment),
                        None => {
                            cov_mark::hit!(bogus_paths);
                            return;
                        }
                    },
                };

                if import.kind == ImportKind::TypeOnly {
                    def.values = None;
                }

                tracing::debug!("resolved import {:?} ({:?}) to {:?}", name, import, def);

                // extern crates in the crate root are special-cased to insert entries into the extern prelude: rust-lang/rust#54658
                if import.is_extern_crate && module_id == self.def_map.root {
                    if let (Some(def), Some(name)) = (def.take_types(), name) {
                        self.def_map.extern_prelude.insert(name.clone(), def);
                    }
                }

                self.update(module_id, &[(name.cloned(), def)], vis, ImportType::Named);
            }
            ImportKind::Glob => {
                tracing::debug!("glob import: {:?}", import);
                match def.take_types() {
                    Some(ModuleDefId::ModuleId(m)) => {
                        if import.is_prelude {
                            // Note: This dodgily overrides the injected prelude. The rustc
                            // implementation seems to work the same though.
                            cov_mark::hit!(std_prelude);
                            self.def_map.prelude = Some(m);
                        } else if m.krate != self.def_map.krate {
                            cov_mark::hit!(glob_across_crates);
                            // glob import from other crate => we can just import everything once
                            let item_map = m.def_map(self.db);
                            let scope = &item_map[m.local_id].scope;

                            // Module scoped macros is included
                            let items = scope
                                .resolutions()
                                // only keep visible names...
                                .map(|(n, res)| {
                                    (
                                        n,
                                        res.filter_visibility(|v| v.is_visible_from_other_crate()),
                                    )
                                })
                                .filter(|(_, res)| !res.is_none())
                                .collect::<Vec<_>>();

                            self.update(module_id, &items, vis, ImportType::Glob);
                        } else {
                            // glob import from same crate => we do an initial
                            // import, and then need to propagate any further
                            // additions
                            let def_map;
                            let scope = if m.block == self.def_map.block_id() {
                                &self.def_map[m.local_id].scope
                            } else {
                                def_map = m.def_map(self.db);
                                &def_map[m.local_id].scope
                            };

                            // Module scoped macros is included
                            let items = scope
                                .resolutions()
                                // only keep visible names...
                                .map(|(n, res)| {
                                    (
                                        n,
                                        res.filter_visibility(|v| {
                                            v.is_visible_from_def_map(
                                                self.db,
                                                &self.def_map,
                                                module_id,
                                            )
                                        }),
                                    )
                                })
                                .filter(|(_, res)| !res.is_none())
                                .collect::<Vec<_>>();

                            self.update(module_id, &items, vis, ImportType::Glob);
                            // record the glob import in case we add further items
                            let glob = self.glob_imports.entry(m.local_id).or_default();
                            if !glob.iter().any(|(mid, _)| *mid == module_id) {
                                glob.push((module_id, vis));
                            }
                        }
                    }
                    Some(ModuleDefId::AdtId(AdtId::EnumId(e))) => {
                        cov_mark::hit!(glob_enum);
                        // glob import from enum => just import all the variants

                        // XXX: urgh, so this works by accident! Here, we look at
                        // the enum data, and, in theory, this might require us to
                        // look back at the crate_def_map, creating a cycle. For
                        // example, `enum E { crate::some_macro!(); }`. Luckily, the
                        // only kind of macro that is allowed inside enum is a
                        // `cfg_macro`, and we don't need to run name resolution for
                        // it, but this is sheer luck!
                        let enum_data = self.db.enum_data(e);
                        let resolutions = enum_data
                            .variants
                            .iter()
                            .map(|(local_id, variant_data)| {
                                let name = variant_data.name.clone();
                                let variant = EnumVariantId {
                                    parent: e,
                                    local_id,
                                };
                                let res = PerNamespace::both(variant.into(), variant.into(), vis);
                                (Some(name), res)
                            })
                            .collect::<Vec<_>>();
                        self.update(module_id, &resolutions, vis, ImportType::Glob);
                    }
                    Some(d) => {
                        tracing::debug!("glob import {:?} from non-module/enum {:?}", import, d);
                    }
                    None => {
                        tracing::debug!("glob import {:?} didn't resolve as type", import);
                    }
                }
            }
        }
    }

    fn update(
        &mut self,
        module_id: LocalModuleId,
        resolutions: &[(Option<Name>, PerNamespace)],
        vis: Visibility,
        import_type: ImportType,
    ) {
        self.db.unwind_if_cancelled();
        self.update_recursive(module_id, resolutions, vis, import_type, 0)
    }

    fn update_recursive(
        &mut self,
        module_id: LocalModuleId,
        resolutions: &[(Option<Name>, PerNamespace)],
        // All resolutions are imported with this visibility; the visibilities in
        // the `PerNs` values are ignored and overwritten
        vis: Visibility,
        import_type: ImportType,
        depth: usize,
    ) {
        if GLOB_RECURSION_LIMIT.check(depth).is_err() {
            // prevent stack overflows (but this shouldn't be possible)
            panic!("infinite recursion in glob imports!");
        }
        let mut changed = false;

        for (name, res) in resolutions {
            match name {
                Some(name) => {
                    let scope = &mut self.def_map.modules[module_id].scope;
                    changed |= scope.push_res_with_import(
                        &mut self.from_glob_import,
                        (module_id, name.clone()),
                        res.with_visibility(vis),
                        import_type,
                    );
                }
                None => {
                    let tr = match res.take_types() {
                        Some(ModuleDefId::TraitId(tr)) => tr,
                        Some(other) => {
                            tracing::debug!("non-trait `_` import of {:?}", other);
                            continue;
                        }
                        None => continue,
                    };
                    let old_vis = self.def_map.modules[module_id].scope.unnamed_trait_vis(tr);
                    let should_update = match old_vis {
                        None => true,
                        Some(old_vis) => {
                            let max_vis = old_vis.max(vis, &self.def_map).unwrap_or_else(|| {
                                panic!("`Tr as _` imports with unrelated visibilities {:?} and {:?} (trait {:?})", old_vis, vis, tr);
                            });

                            if max_vis == old_vis {
                                false
                            } else {
                                cov_mark::hit!(upgrade_underscore_visibility);
                                true
                            }
                        }
                    };

                    if should_update {
                        changed = true;
                        self.def_map.modules[module_id]
                            .scope
                            .push_unnamed_trait(tr, vis);
                    }
                }
            }
        }

        if !changed {
            return;
        }
        let glob_imports = self
            .glob_imports
            .get(&module_id)
            .into_iter()
            .flat_map(|v| v.iter())
            .filter(|(glob_importing_module, _)| {
                // we know all resolutions have the same visibility (`vis`), so we
                // just need to check that once
                vis.is_visible_from_def_map(self.db, &self.def_map, *glob_importing_module)
            })
            .cloned()
            .collect::<Vec<_>>();

        for (glob_importing_module, glob_import_vis) in glob_imports {
            self.update_recursive(
                glob_importing_module,
                resolutions,
                glob_import_vis,
                ImportType::Glob,
                depth + 1,
            );
        }
    }
}
/// Walks a single module, populating defs, imports and macros
struct ModCollector<'a, 'b> {
    def_collector: &'a mut DefCollector<'b>,
    macro_depth: usize,
    module_id: LocalModuleId,
    tree_id: TreeId,
    item_tree: &'a ItemTree,
    mod_dir: ModDir,
}

impl ModCollector<'_, '_> {
    fn collect(&mut self, items: &[ModItem]) {
        todo!()
    }

    fn collect_module(&mut self, module: &Mod, attrs: &Attrs) {
        let path_attr = attrs.by_key("path").string_value();
        let is_macro_use = attrs.by_key("macro_use").exists();
        match &module.kind {
            // inline module, just recurse
            ModKind::Inline { items } => {
                let module_id = self.push_child_module(
                    module.name.clone(),
                    AstId::new(self.file_id(), module.ast_id),
                    None,
                    &self.item_tree[module.visibility],
                );

                if let Some(mod_dir) = self
                    .mod_dir
                    .descend_into_definition(&module.name, path_attr)
                {
                    ModCollector {
                        def_collector: &mut *self.def_collector,
                        macro_depth: self.macro_depth,
                        module_id,
                        tree_id: self.tree_id,
                        item_tree: self.item_tree,
                        mod_dir,
                    }
                    .collect(&*items);
                }
            }
            // out of line module, resolve, parse and recurse
            ModKind::Outline {} => {
                todo!()
            }
        }
    }

    fn push_child_module(
        &mut self,
        name: Name,
        declaration: AstId<ast::Module>,
        definition: Option<(FileID, bool)>,
        visibility: &crate::visibility::RawVisibility,
    ) -> LocalModuleId {
        let vis = self
            .def_collector
            .def_map
            .resolve_visibility(self.def_collector.db, self.module_id, visibility)
            .unwrap_or(Visibility::Public);
        let modules = &mut self.def_collector.def_map.modules;
        let origin = match definition {
            None => ModuleOrigin::Inline {
                definition: declaration,
            },
            Some((definition, is_mod_rs)) => ModuleOrigin::File {
                declaration,
                definition,
                is_mod_rs,
            },
        };

        let res = modules.alloc(ModuleData::new(origin, vis));
        modules[res].parent = Some(self.module_id);
        modules[self.module_id].children.insert(name.clone(), res);

        let module = self.def_collector.def_map.module_id(res);
        let def = ModuleDefId::from(module);

        self.def_collector.def_map.modules[self.module_id]
            .scope
            .declare(def);
        self.def_collector.update(
            self.module_id,
            &[(Some(name), PerNamespace::from_def(def, vis, false))],
            vis,
            ImportType::Named,
        );
        res
    }

    /// Resolves attributes on an item.
    ///
    /// Returns `Err` when some attributes could not be resolved to builtins and have been
    /// registered as unresolved.
    ///
    /// If `ignore_up_to` is `Some`, attributes preceding and including that attribute will be
    /// assumed to be resolved already.
    fn resolve_attributes(&mut self, attrs: &Attrs, mod_item: ModItem) -> Result<(), ()> {
        let mut ignore_up_to = self
            .def_collector
            .skip_attrs
            .get(&InFile::new(self.file_id(), mod_item))
            .copied();
        let iter = attrs
            .iter()
            .dedup_by(|a, b| {
                // FIXME: this should not be required, all attributes on an item should have a
                // unique ID!
                // Still, this occurs because `#[cfg_attr]` can "expand" to multiple attributes:
                //     #[cfg_attr(not(off), unresolved, unresolved)]
                //     struct S;
                // We should come up with a different way to ID attributes.
                a.id == b.id
            })
            .skip_while(|attr| match ignore_up_to {
                Some(id) if attr.id == id => {
                    ignore_up_to = None;
                    true
                }
                Some(_) => true,
                None => false,
            });

        for attr in iter {
            if self.is_builtin_or_registered_attr(&attr.path) {
                continue;
            }
            tracing::debug!("non-builtin attribute {}", attr.path);

            let ast_id = AstIdWithPath::new(
                self.file_id(),
                mod_item.ast_id(self.item_tree),
                attr.path.as_ref().clone(),
            );

            return Err(());
        }

        Ok(())
    }

    fn is_builtin_or_registered_attr(&self, path: &ModPath) -> bool {
        if path.kind != PathKind::Plain {
            return false;
        }

        let segments = path.segments();

        if let Some(name) = segments.first() {
            let name = name.to_smol_str();
            let pred = |n: &_| *n == name;

            let registered = self
                .def_collector
                .registered_tools
                .iter()
                .map(SmolStr::as_str);
            let is_tool = builtin_attr::TOOL_MODULES
                .iter()
                .copied()
                .chain(registered)
                .any(pred);
            if is_tool {
                return true;
            }

            if segments.len() == 1 {
                let registered = self
                    .def_collector
                    .registered_attrs
                    .iter()
                    .map(SmolStr::as_str);
                let is_inert = builtin_attr::INERT_ATTRIBUTES
                    .iter()
                    .copied()
                    .chain(registered)
                    .any(pred);
                return is_inert;
            }
        }
        false
    }

    fn emit_unconfigured_diagnostic(&mut self, item: ModItem) {
        todo!()
    }

    fn file_id(&self) -> HirFileID {
        self.tree_id.file_id()
    }
}

#[cfg(test)]
mod tests {
    use crate::{db::DefDatabase, test_db::TestDB};
    use base_db::{fixture::WithFixture, SourceDatabase};

    use super::*;

    fn do_collect_defs(db: &dyn DefDatabase, def_map: DefMap) -> DefMap {
        let mut collector = DefCollector {
            db,
            def_map,
            deps: FxHashMap::default(),
            glob_imports: FxHashMap::default(),
            unresolved_imports: Vec::new(),
            resolved_imports: Vec::new(),
            mod_dirs: FxHashMap::default(),
            from_glob_import: Default::default(),
            skip_attrs: Default::default(),
            derive_helpers_in_scope: Default::default(),
            registered_attrs: Default::default(),
            registered_tools: Default::default(),
        };
        collector.seed_with_top_level();
        collector.collect();
        collector.def_map
    }

    fn do_resolve(not_ra_fixture: &str) -> DefMap {
        let (db, file_id) = TestDB::with_single_file(not_ra_fixture);
        let krate = db.test_crate();

        let edition = db.crate_graph()[krate].edition;
        let module_origin = ModuleOrigin::CrateRoot {
            definition: file_id,
        };
        let def_map = DefMap::empty(krate, edition, module_origin);
        do_collect_defs(&db, def_map)
    }

    #[test]
    fn test_macro_expand_will_stop_1() {
        do_resolve(
            r#"
macro_rules! foo {
    ($($ty:ty)*) => { foo!($($ty)*); }
}
foo!(KABOOM);
"#,
        );
        do_resolve(
            r#"
macro_rules! foo {
    ($($ty:ty)*) => { foo!(() $($ty)*); }
}
foo!(KABOOM);
"#,
        );
    }

    #[ignore]
    #[test]
    fn test_macro_expand_will_stop_2() {
        // FIXME: this test does succeed, but takes quite a while: 90 seconds in
        // the release mode. That's why the argument is not an ra_fixture --
        // otherwise injection highlighting gets stuck.
        //
        // We need to find a way to fail this faster.
        do_resolve(
            r#"
macro_rules! foo {
    ($($ty:ty)*) => { foo!($($ty)* $($ty)*); }
}
foo!(KABOOM);
"#,
        );
    }
}
