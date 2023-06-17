use crate::*;
use husky_manifest::PackageDependency;
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
        for _ in self.0.iter().filter(|entry| entry.ident == new_entry.ident) {
            // todo!()
            // ad hoc
            return Ok(());
        }
        self.0.push(new_entry);
        Ok(())
        // todo!()
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
        // ad hoc
        // todo: override
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

impl EntitySymbolEntry {
    pub(crate) fn new_crate_root(db: &dyn EntityTreeDb, crate_path: CratePath) -> Self {
        let root_module_path = ModulePath::new_root(db, crate_path);
        Self {
            ident: db.word_menu().crate_ident(),
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
        rule: &mut UseExprRule,
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

    pub(crate) fn export_via_use_all(
        &self,
        db: &dyn EntityTreeDb,
        reference_module_path: ModulePath,
        rule: &UseAllRule,
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
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct EntityNodeEntry {
    node: EntityNode,
    /// cached for performance, always equal to symbol.ident(db)
    ident: Ident,
    /// cached for performance, always equal to symbol.visibility(db)
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
            ident: ident_token.ident(),
            visibility,
            node,
        })
    }

    pub fn node(&self) -> EntityNode {
        self.node
    }
}
