use husky_expr::{PatternSymbolIdx, PatternSymbolOrderedMap, SymbolOrderedMap};

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = SignatureDb)]
pub struct SymbolRawTermRegion {
    registry: RawTermSymbolRegistry,
    symbol_signatures: SymbolOrderedMap<SymbolSignature>,
    self_ty_term: Option<RawTerm>,
    self_value_term: Option<RawTermSymbol>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SymbolSignature {
    symbol: Option<RawTermSymbol>,
    modifier: SymbolModifier,
    ty: RawTermSymbolTypeResult<RawTerm>,
}

impl SymbolSignature {
    pub fn symbol(self) -> Option<RawTermSymbol> {
        self.symbol
    }

    pub fn modifier(&self) -> SymbolModifier {
        self.modifier
    }

    pub fn ty(&self) -> RawTermSymbolTypeResult<RawTerm> {
        self.ty
    }
}

impl SymbolRawTermRegion {
    #[inline(always)]
    pub(crate) fn add_new_implicit_parameter_symbol_signature(
        &mut self,
        db: &dyn SignatureDb,
        idx: CurrentSymbolIdx,
        ty: RawTermSymbolTypeResult<RawTerm>,
    ) {
        let symbol = self.registry.new_symbol(db, ty);
        self.add_new_current_symbol_signature(
            db,
            idx,
            SymbolSignature {
                symbol: Some(symbol),
                ty,
                modifier: SymbolModifier::Const,
            },
        )
    }

    #[inline(always)]
    pub(crate) fn add_new_explicit_parameter_symbol_signature(
        &mut self,
        db: &dyn SignatureDb,
        current_symbol: CurrentSymbolIdx,
        modifier: SymbolModifier,
        ty: RawTermSymbolTypeResult<RawTerm>,
    ) {
        let symbol = match modifier {
            SymbolModifier::Const => todo!(),
            SymbolModifier::Pure | SymbolModifier::Mut | SymbolModifier::RefMut => None,
        };
        self.add_new_current_symbol_signature(
            db,
            current_symbol,
            SymbolSignature {
                modifier,
                ty,
                symbol,
            },
        )
    }

    #[inline(always)]
    fn add_new_current_symbol_signature(
        &mut self,
        db: &dyn SignatureDb,
        idx: CurrentSymbolIdx,
        signature: SymbolSignature,
    ) {
        self.symbol_signatures.insert_next(idx, signature)
    }
}

impl SymbolRawTermRegion {
    /// will initialize `inherited_symbol_terms`;
    /// but will leave current_symbol_terms unintialized;
    /// `self_ty_term` is set to that of parent if parent exists, otherwise none;
    /// `self_value_term` is set to that of parent if parent exists, otherwise none
    pub(crate) fn new(parent: Option<&SymbolRawTermRegion>, symbol_region: &SymbolRegion) -> Self {
        let registry = parent.map_or(Default::default(), |parent| parent.registry.clone());
        Self {
            registry,
            symbol_signatures: SymbolOrderedMap::new(
                parent.map(|parent| &parent.symbol_signatures),
            ),
            self_ty_term: parent.map(|parent| parent.self_ty_term).flatten(),
            self_value_term: parent.map(|parent| parent.self_value_term).flatten(),
        }
    }

    pub(crate) fn init_self_ty_and_value(
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
        for current_symbol_signature in self.symbol_signatures.current_symbol_map().iter().copied()
        {
            self_ty = self_ty.apply(
                db,
                current_symbol_signature.symbol().expect("should have term"),
            )
        }
        self_ty
    }

    pub fn self_ty_term(&self) -> Option<RawTerm> {
        self.self_ty_term
    }

    pub fn self_value_term(&self) -> Option<RawTermSymbol> {
        self.self_value_term
    }

    fn parent_symbol_term(&self, parent_symbol_idx: ParentSymbolIdx) -> SymbolSignature {
        match parent_symbol_idx {
            ParentSymbolIdx::Inherited(inherited_symbol_idx) => {
                self.inherited_symbol_signature(inherited_symbol_idx)
            }
            ParentSymbolIdx::Current(current_symbol_idx) => self
                .current_symbol_signature(current_symbol_idx)
                .expect("should exist"),
        }
    }

    pub fn inherited_symbol_signature(
        &self,
        inherited_symbol_idx: InheritedSymbolIdx,
    ) -> SymbolSignature {
        self.symbol_signatures[inherited_symbol_idx]
    }

    pub fn current_symbol_signature(
        &self,
        current_symbol_idx: CurrentSymbolIdx,
    ) -> Option<SymbolSignature> {
        self.symbol_signatures
            .current_symbol_map()
            .get(current_symbol_idx.raw())
            .copied()
    }
}
