mod prelude;

pub use self::prelude::*;

use crate::*;
use husky_coword::coword_menu;
use husky_manifest::ManifestDependency;

use husky_token::IdentToken;
use husky_vfs::error::VfsResult;

#[derive(Debug, Default, PartialEq, Eq)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct EntitySymbolTable(Vec<EntitySymbolEntry>);

impl EntitySymbolTable {
    pub(crate) fn as_ref<'a>(&'a self) -> EntitySymbolTableRef<'a> {
        EntitySymbolTableRef(&self.0)
    }

    pub(crate) fn data<'a>(&'a self) -> &'a [EntitySymbolEntry] {
        &self.0
    }

    pub(crate) fn push(&mut self, new_entry: EntitySymbolEntry) -> EntitySynTreeResult<()> {
        // todo: should there be checks?
        self.0.push(new_entry);
        Ok(())
    }

    pub(crate) fn extend(
        &mut self,
        iter: impl IntoIterator<Item = EntitySymbolEntry>,
    ) -> EntitySynTreeResult<()> {
        for new_entry in iter {
            self.push(new_entry)?
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct EntitySymbolTableRef<'a>(&'a [EntitySymbolEntry]);

impl<'a> EntitySymbolTableRef<'a> {
    // todo: add token_idx: TokenIdx
    pub fn resolve_ident(
        &self,
        db: &::salsa::Db,
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

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct EntitySymbolEntry {
    ident: Ident,
    visibility: Scope,
    symbol: EntitySymbol,
}

impl EntitySymbolEntry {
    pub(crate) fn new_crate_root(db: &::salsa::Db, crate_path: CratePath) -> Self {
        let root_module_path = crate_path.root_module_path(db);
        Self {
            ident: coword_menu(db).crate_ident(),
            visibility: Scope::PubUnder(root_module_path),
            symbol: EntitySymbol::CrateRoot { root_module_path },
        }
    }

    pub(crate) fn new_package_dependency(
        db: &::salsa::Db,
        package_dependency: &ManifestDependency,
    ) -> VfsResult<Self> {
        let package_path = package_dependency.package_path();
        let Some(lib_root_module_path) = package_path.lib_root_module_path(db) else {
            todo!()
        };
        Ok(Self {
            ident: package_path.ident(db),
            visibility: Scope::Pub,
            symbol: EntitySymbol::PackageDependency {
                item_path: lib_root_module_path.into(),
            },
        })
    }

    pub(crate) fn new_use_symbol_entry(
        db: &::salsa::Db,
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
        db: &::salsa::Db,
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
        db: &::salsa::Db,
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
        db: &::salsa::Db,
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
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct MajorEntityNodeTable {
    entries: Vec<ItemNodeEntry>,
}

impl MajorEntityNodeTable {
    pub(crate) fn item_symbol_table(&self, db: &::salsa::Db) -> EntitySymbolTable {
        EntitySymbolTable(
            self.entries
                .iter()
                .filter_map(|entry| EntitySymbolEntry::from_node(db, entry))
                .collect(),
        )
    }

    pub(crate) fn try_add_new_node(
        &mut self,
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
        item_path: ItemPath,
        block: DefnBlock,
    ) {
        if let Some(entry) = ItemNodeEntry::new(
            db,
            registry,
            visibility,
            ast_idx,
            ident_token,
            item_path,
            block,
        ) {
            self.entries.push(entry)
        }
    }

    pub(crate) fn node(&self, syn_node_path: ItemSynNodePath) -> Option<&ItemSynNode> {
        self.entries
            .iter()
            .find_map(|entry| (entry.syn_node_path == syn_node_path).then_some(&entry.node))
    }

    pub(crate) fn node_paths<'a>(&'a self) -> impl Iterator<Item = ItemSynNodePath> + 'a {
        self.entries.iter().map(|entry| entry.syn_node_path)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct ItemNodeEntry {
    node: ItemSynNode,
    /// cached for performance, always equal to node.syn_node_path(db)
    syn_node_path: ItemSynNodePath,
    /// cached for performance, always equal to node.ident(db)
    ident: Ident,
    /// cached for performance, always equal to node.visibility(db)
    visibility: Scope,
}

impl EntitySymbolEntry {
    fn from_node(db: &::salsa::Db, node_entry: &ItemNodeEntry) -> Option<Self> {
        Some(EntitySymbolEntry {
            ident: node_entry.ident,
            visibility: node_entry.visibility,
            symbol: EntitySymbol::from_node(db, &node_entry.node)?,
        })
    }
}

impl ItemNodeEntry {
    fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
        item_path: ItemPath,
        block: DefnBlock,
    ) -> Option<Self> {
        let node = ItemSynNode::try_new(
            db,
            registry,
            visibility,
            ast_idx,
            ident_token,
            item_path,
            block,
        )?;
        Some(Self {
            syn_node_path: node.syn_node_path(db),
            ident: ident_token.ident(),
            visibility,
            node,
        })
    }

    pub(crate) fn syn_node(&self) -> &ItemSynNode {
        &self.node
    }
}
