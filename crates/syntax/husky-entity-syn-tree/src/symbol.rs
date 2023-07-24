use crate::*;
use husky_token::{IdentToken, TokenIdx};

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub struct UseSymbol {
    #[id]
    pub original_symbol: EntitySymbol,
    pub path: PrincipalEntityPath,
    pub visibility: Scope,
    pub ast_idx: AstIdx,
    pub use_expr_idx: UseExprIdx,
}

impl ModuleItemSynNode {
    pub fn ident(&self, db: &dyn EntitySynTreeDb) -> Ident {
        self.syn_node_path(db).ident(db)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
#[enum_class::from_variants]
pub enum EntitySymbol {
    CrateRoot {
        root_module_path: ModulePath,
    },
    SelfModule {
        module_path: ModulePath,
    },
    SuperModule {
        current_module_path: ModulePath,
        super_module_path: ModulePath,
    },
    UniversalPrelude {
        item_path: PrincipalEntityPath,
    },
    PackageDependency {
        item_path: PrincipalEntityPath,
    },
    Submodule {
        submodule_path: SubmodulePath,
        node: SubmoduleSynNode,
    },
    ModuleItem {
        module_item_path: ModuleItemPath,
        node: ModuleItemSynNode,
    },
    TypeVariant {
        ty_variant_path: TypeVariantPath,
    },
    Use(UseSymbol),
}

impl EntitySymbol {
    pub(crate) fn from_node(db: &dyn EntitySynTreeDb, node: EntitySynNode) -> Option<Self> {
        match node {
            EntitySynNode::Submodule(node) => Some(EntitySymbol::Submodule {
                submodule_path: node.unambiguous_path(db)?,
                node,
            }),
            EntitySynNode::ModuleItem(node) => Some(EntitySymbol::ModuleItem {
                module_item_path: node.unambiguous_path(db)?,
                node,
            }),
            EntitySynNode::AssociatedItem(_)
            | EntitySynNode::TypeVariant(_)
            | EntitySynNode::ImplBlock(_) => {
                unreachable!()
            }
        }
    }
}

impl EntitySymbol {
    pub fn path(self, db: &dyn EntitySynTreeDb) -> PrincipalEntityPath {
        match self {
            EntitySymbol::CrateRoot { root_module_path } => root_module_path.into(),
            EntitySymbol::SelfModule { module_path } => module_path.into(),
            EntitySymbol::SuperModule {
                super_module_path, ..
            } => super_module_path.into(),
            EntitySymbol::UniversalPrelude { item_path }
            | EntitySymbol::PackageDependency { item_path } => item_path.into(),
            EntitySymbol::Submodule { submodule_path, .. } => submodule_path.inner().into(),
            EntitySymbol::ModuleItem {
                module_item_path, ..
            } => module_item_path.into(),
            // symbol.path(db).into(),
            EntitySymbol::Use(symbol) => symbol.path(db).into(),
            EntitySymbol::TypeVariant { ty_variant_path } => ty_variant_path.into(),
        }
    }

    pub fn module_item_node(self) -> Option<ModuleItemSynNode> {
        match self {
            EntitySymbol::ModuleItem { node, .. } => Some(node),
            _ => None,
        }
    }
}

// can only see module symbols
#[derive(Debug, Clone, Copy)]
pub struct ModuleSymbolContext<'a> {
    crate_prelude: CratePrelude<'a>,
    module_symbols: EntitySymbolTableRef<'a>,
}

impl<'a> ModuleSymbolContext<'a> {
    pub fn new(crate_prelude: CratePrelude<'a>, module_symbols: EntitySymbolTableRef<'a>) -> Self {
        Self {
            crate_prelude,
            module_symbols,
        }
    }

    pub fn new_default(db: &'a dyn EntitySynTreeDb, crate_path: CratePath) -> PreludeResult<Self> {
        Ok(Self {
            crate_prelude: CratePrelude::new(db, crate_path)?,
            module_symbols: Default::default(),
        })
    }

    pub fn resolve_ident(
        &self,
        db: &'a dyn EntitySynTreeDb,
        reference_module_path: ModulePath,
        _token_idx: TokenIdx,
        ident: Ident,
    ) -> Option<EntitySymbol> {
        self.module_symbols
            .resolve_ident(db, reference_module_path.into(), ident)
            .or_else(|| {
                self.crate_prelude
                    .resolve_ident(db, reference_module_path, ident)
            })
    }
}

pub(crate) fn module_symbol_context<'a>(
    db: &'a dyn EntitySynTreeDb,
    module_path: ModulePath,
) -> ItemSynTreeResult<ModuleSymbolContext<'a>> {
    let item_tree_sheet = db.item_syn_tree_sheet(module_path)?;
    Ok(ModuleSymbolContext {
        crate_prelude: CratePrelude::new(db, module_path.crate_path(db))?,
        module_symbols: item_tree_sheet.module_symbols().into(),
    })
}
