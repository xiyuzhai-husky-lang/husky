use husky_expr::{CurrentSymbolFullMap, CurrentSymbolMap, InheritedSymbolFullMap};

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = SignatureDb)]
pub struct RawTermSymbolRegion {
    registry: TermSymbolRegistry,
    inherited_symbol_terms: InheritedSymbolFullMap<RawTermSymbol>,
    current_symbol_terms: CurrentSymbolFullMap<RawTermSymbol>,
    self_ty_term: Option<RawTerm>,
    self_value_term: Option<RawTermSymbol>,
}

impl RawTermSymbolRegion {
    /// will initialize `inherited_symbol_terms`;
    /// but will leave current_symbol_terms unintialized;
    /// `self_ty_term` is set to that of parent if parent exists, otherwise none;
    /// `self_value_term` is set to that of parent if parent exists, otherwise none
    pub(super) fn new(parent: Option<&RawTermSymbolRegion>, symbol_region: &SymbolRegion) -> Self {
        let registry = parent.map_or(Default::default(), |parent| parent.registry.clone());
        let inherited_symbol_terms =
            InheritedSymbolFullMap::new(symbol_region.inherited_symbol_arena(), |symbol| {
                parent
                    .unwrap()
                    .parent_symbol_term(symbol.parent_symbol_idx())
            });
        Self {
            registry,
            inherited_symbol_terms,
            current_symbol_terms: Default::default(),
            self_ty_term: parent.map(|parent| parent.self_ty_term).flatten(),
            self_value_term: parent.map(|parent| parent.self_value_term).flatten(),
        }
    }

    pub(super) fn init_self_ty_and_value(
        &mut self,
        db: &dyn SignatureDb,
        region_path: RegionPath,
        symbol_region: &SymbolRegion,
    ) {
        if symbol_region.allow_self_ty().to_bool() && self.self_ty_term.is_none() {
            self.self_ty_term = Some(match region_path {
                RegionPath::Decl(DeclRegionPath::Entity(EntityPath::ModuleItem(
                    ModuleItemPath::Trait(_),
                ))) => self.trai_self_ty_term(db),
                RegionPath::Decl(DeclRegionPath::Entity(EntityPath::ModuleItem(
                    ModuleItemPath::Type(ty_path),
                ))) => self.ty_self_ty_term(db, ty_path),
                RegionPath::Decl(DeclRegionPath::ImplBlock(impl_block_id)) => match impl_block_id {
                    ImplBlockId::Type(impl_block_id) => {
                        self.ty_self_ty_term(db, impl_block_id.ty_path())
                    }
                    ImplBlockId::TraitForType(impl_block_id) => {
                        self.ty_self_ty_term(db, impl_block_id.ty_path())
                    }
                    ImplBlockId::IllFormed(_) => unreachable!(),
                },
                _ => unreachable!(),
            })
        }
        if symbol_region.allow_self_value().to_bool() && self.self_value_term.is_none() {
            self.self_value_term = Some(
                self.registry
                    .new_symbol(db, Ok(self.self_ty_term.expect("self type should exists")))
                    .into(),
            )
        }
    }
    fn trai_self_ty_term(&mut self, db: &dyn SignatureDb) -> RawTerm {
        // todo: general universe
        self.registry.new_symbol(db, Ok(RawTerm::TYPE)).into()
    }

    fn ty_self_ty_term(&self, db: &dyn SignatureDb, ty_path: TypePath) -> RawTerm {
        let mut self_ty: RawTerm = RawTermEntityPath::Type(ty_path.into()).into();
        for current_symbol_term in self.current_symbol_terms.iter().copied() {
            self_ty = self_ty.apply(db, current_symbol_term)
        }
        self_ty
    }

    #[inline(always)]
    pub(super) fn add_new_symbol(
        &mut self,
        db: &dyn SignatureDb,
        idx: CurrentSymbolIdx,
        ty: Result<RawTerm, RawTermSymbolTypeErrorKind>,
    ) {
        self.current_symbol_terms
            .insert_next(idx, self.registry.new_symbol(db, ty))
    }
}

impl RawTermSymbolRegion {
    pub fn self_ty_term(&self) -> Option<RawTerm> {
        self.self_ty_term
    }

    pub fn self_value_term(&self) -> Option<RawTermSymbol> {
        self.self_value_term
    }

    fn parent_symbol_term(&self, parent_symbol_idx: ParentSymbolIdx) -> RawTermSymbol {
        match parent_symbol_idx {
            ParentSymbolIdx::Inherited(inherited_symbol_idx) => {
                self.inherited_symbol_term(inherited_symbol_idx)
            }
            ParentSymbolIdx::Current(current_symbol_idx) => {
                self.current_symbol_term(current_symbol_idx).unwrap()
            }
        }
    }

    pub fn inherited_symbol_term(&self, inherited_symbol_idx: InheritedSymbolIdx) -> RawTermSymbol {
        self.inherited_symbol_terms[inherited_symbol_idx]
    }

    pub fn current_symbol_term(
        &self,
        current_symbol_idx: CurrentSymbolIdx,
    ) -> Option<RawTermSymbol> {
        self.current_symbol_terms
            .get(current_symbol_idx.raw())
            .copied()
    }
}
