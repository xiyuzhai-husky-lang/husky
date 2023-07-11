use crate::*;
use husky_manifest::PackageDependency;
use husky_print_utils::p;
use husky_token::IdentToken;

#[derive(Debug, Default, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct EntitySymbolTable(Vec<EntitySymbolEntry>);

impl EntitySymbolTable {
    pub(crate) fn as_ref<'a>(&'a self) -> EntitySymbolTableRef<'a> {
        EntitySymbolTableRef(&self.0)
    }

    pub(crate) fn data<'a>(&'a self) -> &'a [EntitySymbolEntry] {
        &self.0
    }

    pub(crate) fn insert(&mut self, new_entry: EntitySymbolEntry) -> EntityTreeResult<()> {
        // todo: should there be checks?
        self.0.push(new_entry);
        Ok(())
    }

    pub(crate) fn extend(
        &mut self,
        iter: impl IntoIterator<Item = EntitySymbolEntry>,
    ) -> EntityTreeResult<()> {
        for new_entry in iter {
            self.insert(new_entry)?
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EntitySymbolTableRef<'a>(&'a [EntitySymbolEntry]);

impl<'a> EntitySymbolTableRef<'a> {
    // todo: add token_idx: TokenIdx
    pub fn resolve_ident(
        &self,
        db: &dyn EntityTreeDb,
        reference_module_path: ReferenceModulePath,
        ident: Ident,
    ) -> Option<EntitySymbol> {
        self.0.iter().find_map(|entry| {
            ((entry.ident == ident) && entry.is_visible_from(db, reference_module_path))
                .then_some(entry.symbol)
        })
    }

    pub(crate) fn data(&self) -> &'a [EntitySymbolEntry] {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for EntitySymbolTableRef<'a> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        f.debug_tuple("EntitySymbolTableRef")
            .field(&(&self.0).debug(db))
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct EntitySymbolEntry {
    ident: Ident,
    visibility: Scope,
    symbol: EntitySymbol,
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn none_core_crate_universal_prelude(
    db: &dyn EntityTreeDb,
    toolchain: Toolchain,
) -> PreludeResult<EntitySymbolTable> {
    let vfs_path_menu = db.vfs_path_menu(toolchain);
    let entity_path_menu = db.entity_path_menu(toolchain);
    let coword_menu = db.coword_menu();
    let core_prelude_module = vfs_path_menu.core_prelude();
    let mut table = EntitySymbolTable::default();
    table.insert(EntitySymbolEntry {
        ident: coword_menu.core_ident(),
        visibility: Scope::Pub,
        symbol: EntitySymbol::UniversalPrelude {
            entity_path: vfs_path_menu.core().into(),
        },
    });
    table.extend(
        entity_tree_sheet(db, core_prelude_module)
            .map_err(|e| PreludeError::CorePreludeEntityTreeSheet(Box::new(e)))?
            .module_symbols()
            .data()
            .iter()
            .map(|entry| EntitySymbolEntry {
                ident: entry.ident,
                visibility: Scope::Pub,
                symbol: EntitySymbol::UniversalPrelude {
                    entity_path: entry.symbol.path(db),
                },
            }),
    );
    Ok(table)
}

impl EntitySymbolEntry {
    pub(crate) fn new_crate_root(db: &dyn EntityTreeDb, crate_path: CratePath) -> Self {
        let root_module_path = ModulePath::new_root(db, crate_path);
        Self {
            ident: db.coword_menu().crate_ident(),
            visibility: Scope::PubUnder(root_module_path),
            symbol: EntitySymbol::CrateRoot { root_module_path },
        }
    }

    pub(crate) fn new_package_dependency(
        db: &dyn EntityTreeDb,
        package_dependency: &PackageDependency,
    ) -> Self {
        let package_path = package_dependency.package_path();
        Self {
            ident: package_path.ident(db),
            visibility: Scope::Pub,
            symbol: EntitySymbol::PackageDependency {
                entity_path: package_path.lib_module(db).into(),
            },
        }
    }

    pub(crate) fn new_use_symbol_entry(
        db: &dyn EntityTreeDb,
        original_symbol: EntitySymbol,
        rule: &mut OnceUseRule,
    ) -> Self {
        rule.mark_as_resolved(original_symbol);
        let visibility = rule.visibility();
        Self {
            ident: rule.ident().unwrap(),
            visibility,
            symbol: UseSymbol::new(
                db,
                original_symbol,
                original_symbol.path(db),
                visibility,
                rule.ast_idx(),
                rule.use_expr_idx(),
            )
            .into(),
        }
    }

    pub(crate) fn new_use_ty_variant_entry(
        db: &dyn EntityTreeDb,
        parent_rule: &OnceUseRule,
        ident: Ident,
        ty_variant_path: TypeVariantPath,
    ) -> Self {
        let visibility = parent_rule.visibility();
        Self {
            ident,
            visibility,
            symbol: UseSymbol::new(
                db,
                EntitySymbol::TypeVariant { ty_variant_path },
                ty_variant_path.into(),
                visibility,
                parent_rule.ast_idx(),
                parent_rule.use_expr_idx(),
            )
            .into(),
        }
    }

    pub(crate) fn export_via_use_all(
        &self,
        db: &dyn EntityTreeDb,
        reference_module_path: ModulePath,
        rule: &UseAllModuleSymbolsRule,
    ) -> Option<Self> {
        self.is_visible_from(db, reference_module_path.into())
            .then_some(EntitySymbolEntry {
                ident: self.ident,
                visibility: rule.visibility(),
                symbol: UseSymbol::new(
                    db,
                    self.symbol,
                    self.symbol.path(db),
                    rule.visibility(),
                    rule.ast_idx(),
                    rule.use_expr_idx(),
                )
                .into(),
            })
    }

    pub(crate) fn is_visible_from(
        &self,
        db: &dyn EntityTreeDb,
        module_path: ReferenceModulePath,
    ) -> bool {
        self.visibility.is_visible_from(db, module_path)
    }

    pub fn symbol(&self) -> EntitySymbol {
        self.symbol
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn visibility(&self) -> Scope {
        self.visibility
    }
}

// module items and submodules
#[derive(Debug, Default, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct MajorEntityNodeTable {
    entries: Vec<EntityNodeEntry>,
}

impl MajorEntityNodeTable {
    pub(crate) fn entity_symbol_table(&self, db: &dyn EntityTreeDb) -> EntitySymbolTable {
        EntitySymbolTable(
            self.entries
                .iter()
                .filter_map(|entry| EntitySymbolEntry::from_node(db, entry))
                .collect(),
        )
    }

    pub(crate) fn try_add_new_node(
        &mut self,
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
        entity_path: EntityPath,
    ) {
        if let Some(entry) =
            EntityNodeEntry::new(db, registry, visibility, ast_idx, ident_token, entity_path)
        {
            self.entries.push(entry)
        }
    }

    pub(crate) fn node(&self, node_path: EntityNodePath) -> Option<EntityNode> {
        self.entries
            .iter()
            .find_map(|entry| (entry.node_path == node_path).then_some(entry.node))
    }

    pub(crate) fn node_paths<'a>(&'a self) -> impl Iterator<Item = EntityNodePath> + 'a {
        self.entries.iter().map(|entry| entry.node_path)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct EntityNodeEntry {
    node: EntityNode,
    /// cached for performance, always equal to node.node_path(db)
    node_path: EntityNodePath,
    /// cached for performance, always equal to node.ident(db)
    ident: Ident,
    /// cached for performance, always equal to node.visibility(db)
    visibility: Scope,
}

impl EntitySymbolEntry {
    fn from_node(db: &dyn EntityTreeDb, node_entry: &EntityNodeEntry) -> Option<Self> {
        Some(EntitySymbolEntry {
            ident: node_entry.ident,
            visibility: node_entry.visibility,
            symbol: EntitySymbol::from_node(db, node_entry.node)?,
        })
    }
}

impl EntityNodeEntry {
    fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
        entity_path: EntityPath,
    ) -> Option<Self> {
        let node =
            EntityNode::try_new(db, registry, visibility, ast_idx, ident_token, entity_path)?;
        Some(Self {
            node_path: node.node_path(db),
            ident: ident_token.ident(),
            visibility,
            node,
        })
    }

    pub fn node(&self) -> EntityNode {
        self.node
    }
}
