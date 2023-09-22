use husky_entity_syn_tree::*;
use husky_syn_expr::*;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb)]
pub struct SymbolDeclarativeTermRegion {
    symbol_registry: TermSymbolRegistry,
    symbol_signatures: SymbolOrderedMap<SymbolSignature>,
    self_ty_term: Option<DeclarativeTerm>,
    self_value_term: Option<DeclarativeTermSymbol>,
    implicit_template_parameter_symbols: SmallVec<[DeclarativeTermSymbol; 1]>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SymbolSignature {
    kind: SymbolSignatureKind,
    term_symbol: Option<DeclarativeTermSymbol>,
    modifier: SymbolModifier,
    ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolSignatureKind {
    TemplateParameter,
    ParenateParameter,
    FieldVariable,
}

impl SymbolSignature {
    pub fn kind(self) -> SymbolSignatureKind {
        self.kind
    }

    pub fn term_symbol(self) -> Option<DeclarativeTermSymbol> {
        self.term_symbol
    }

    pub fn modifier(&self) -> SymbolModifier {
        self.modifier
    }

    pub fn ty(&self) -> DeclarativeTermSymbolTypeResult<DeclarativeTerm> {
        self.ty
    }
}

impl SymbolDeclarativeTermRegion {
    pub(crate) fn symbol_registry_mut(&mut self) -> &mut TermSymbolRegistry {
        &mut self.symbol_registry
    }

    #[inline(always)]
    pub(crate) fn add_new_template_parameter_symbol_signature(
        &mut self,
        db: &dyn DeclarativeSignatureDb,
        idx: CurrentSynSymbolIdx,
        ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
        term_symbol: DeclarativeTermSymbol,
    ) {
        self.add_new_current_symbol_signature(
            db,
            idx,
            SymbolSignature {
                kind: SymbolSignatureKind::TemplateParameter,
                term_symbol: Some(term_symbol),
                ty,
                modifier: SymbolModifier::Const,
            },
        )
    }

    #[inline(always)]
    pub(crate) fn add_new_parenate_parameter_symbol_signature(
        &mut self,
        db: &dyn DeclarativeSignatureDb,
        current_symbol: CurrentSynSymbolIdx,
        modifier: SymbolModifier,
        ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
    ) {
        let symbol = match modifier {
            SymbolModifier::Const => todo!(),
            SymbolModifier::None | SymbolModifier::Mut | SymbolModifier::RefMut => None,
            SymbolModifier::Ambersand(_) => todo!(),
            SymbolModifier::AmbersandMut(_) => todo!(),
            SymbolModifier::Le => todo!(),
            SymbolModifier::Tilde => todo!(),
        };
        self.add_new_current_symbol_signature(
            db,
            current_symbol,
            SymbolSignature {
                kind: SymbolSignatureKind::ParenateParameter,
                modifier,
                ty,
                term_symbol: symbol,
            },
        )
    }

    #[inline(always)]
    pub(crate) fn add_new_field_variable_symbol_signature(
        &mut self,
        db: &dyn DeclarativeSignatureDb,
        current_symbol: CurrentSynSymbolIdx,
        ty: DeclarativeTermSymbolTypeResult<DeclarativeTerm>,
    ) {
        self.add_new_current_symbol_signature(
            db,
            current_symbol,
            SymbolSignature {
                kind: SymbolSignatureKind::FieldVariable,
                modifier: SymbolModifier::None,
                ty,
                // ad hoc
                term_symbol: None,
            },
        )
    }

    #[inline(always)]
    fn add_new_current_symbol_signature(
        &mut self,
        db: &dyn DeclarativeSignatureDb,
        idx: CurrentSynSymbolIdx,
        signature: SymbolSignature,
    ) {
        self.symbol_signatures.insert_next(idx, signature)
    }

    pub fn implicit_template_parameter_symbols(&self) -> &[DeclarativeTermSymbol] {
        &self.implicit_template_parameter_symbols
    }
}

impl SymbolDeclarativeTermRegion {
    /// will initialize `inherited_symbol_terms`;
    /// but will leave current_symbol_terms unintialized;
    /// `self_ty_term` is set to that of parent if parent exists, otherwise none;
    /// `self_value_term` is set to that of parent if parent exists, otherwise none
    pub(crate) fn new(
        parent: Option<&SymbolDeclarativeTermRegion>,
        symbol_region: &SynSymbolRegion,
    ) -> Self {
        let registry = parent.map_or(Default::default(), |parent| parent.symbol_registry.clone());
        Self {
            symbol_registry: registry,
            symbol_signatures: SymbolOrderedMap::new(
                parent.map(|parent| &parent.symbol_signatures),
            ),
            self_ty_term: parent.map(|parent| parent.self_ty_term).flatten(),
            self_value_term: parent.map(|parent| parent.self_value_term).flatten(),
            implicit_template_parameter_symbols: Default::default(),
        }
    }

    pub(crate) fn infer_self_ty_parameter_and_self_value_parameter(
        &mut self,
        db: &dyn DeclarativeSignatureDb,
        region_path: RegionPath,
        symbol_region: &SynSymbolRegion,
    ) {
        if symbol_region.allow_self_ty().to_bool() && self.self_ty_term.is_none() {
            self.self_ty_term = match region_path {
                RegionPath::Decl(ItemSynNodePath::MajorItem(MajorItemSynNodePath::Trait(_))) => {
                    Some(self.new_self_ty_symbol(db).into())
                }
                RegionPath::Decl(ItemSynNodePath::MajorItem(MajorItemSynNodePath::Type(
                    ty_node_path,
                ))) => Some(self.ty_defn_self_ty_term(
                    db,
                    ty_node_path.path(db).expect("should have valid item path"),
                )),
                RegionPath::Decl(ItemSynNodePath::ImplBlock(syn_node_path)) => {
                    match syn_node_path {
                        ImplBlockSynNodePath::TypeImplBlock(syn_node_path) => {
                            None // reserved for later stage
                        }
                        ImplBlockSynNodePath::TraitForTypeImplBlock(impl_block_path) => {
                            match impl_block_path.ty_sketch(db) {
                                TypeSketch::DeriveAny => Some(self.new_self_ty_symbol(db).into()),
                                TypeSketch::Path(ty_path) => None, // reserved for later stage
                            }
                        }
                        ImplBlockSynNodePath::IllFormedImplBlock(_) => None,
                    }
                }
                _ => unreachable!(),
            }
        }
        if symbol_region.allow_self_value().to_bool() && self.self_value_term.is_none() {
            self.self_value_term = Some(
                DeclarativeTermSymbol::new_self_value(
                    db,
                    &mut self.symbol_registry,
                    self.self_ty_term.expect("self type should exists"),
                )
                .into(),
            )
        }
    }
    fn new_self_ty_symbol(&mut self, db: &dyn DeclarativeSignatureDb) -> DeclarativeTermSymbol {
        let symbol = DeclarativeTermSymbol::new_self_ty(db, &mut self.symbol_registry);
        self.implicit_template_parameter_symbols.push(symbol);
        symbol
    }

    /// this only works on type definitions
    ///
    /// example:
    /// ```husky
    /// enum Animal<T> where
    /// | Dog
    /// | Cat
    /// ```
    ///
    /// then self type term is `Animal T`
    fn ty_defn_self_ty_term(
        &self,
        db: &dyn DeclarativeSignatureDb,
        ty_path: TypePath,
    ) -> DeclarativeTerm {
        let mut self_ty: DeclarativeTerm = DeclarativeTermEntityPath::Type(ty_path.into()).into();
        for current_symbol_signature in self.symbol_signatures.current_symbol_map().iter().copied()
        {
            match current_symbol_signature.kind {
                SymbolSignatureKind::TemplateParameter => {
                    let argument = current_symbol_signature
                        .term_symbol()
                        .expect("should have term");
                    self_ty = self_ty.apply(db, argument)
                }
                SymbolSignatureKind::ParenateParameter => unreachable!(),
                SymbolSignatureKind::FieldVariable => break,
            }
        }
        self_ty
    }

    pub fn self_ty_term(&self) -> Option<DeclarativeTerm> {
        self.self_ty_term
    }

    pub(crate) fn set_self_ty_term(&mut self, self_ty_term: Option<DeclarativeTerm>) {
        debug_assert!(self.self_ty_term.is_none());
        self.self_ty_term = self_ty_term
    }

    pub fn self_value_term(&self) -> Option<DeclarativeTermSymbol> {
        self.self_value_term
    }

    fn parent_symbol_term(&self, parent_symbol_idx: ParentSynSymbolIdx) -> SymbolSignature {
        match parent_symbol_idx {
            ParentSynSymbolIdx::Inherited(inherited_symbol_idx) => {
                self.inherited_symbol_signature(inherited_symbol_idx)
            }
            ParentSynSymbolIdx::Current(current_symbol_idx) => self
                .current_symbol_signature(current_symbol_idx)
                .expect("should exist"),
        }
    }

    pub fn inherited_symbol_signature(
        &self,
        inherited_symbol_idx: InheritedSynSymbolIdx,
    ) -> SymbolSignature {
        self.symbol_signatures[inherited_symbol_idx]
    }

    pub fn current_symbol_signature(
        &self,
        current_symbol_idx: CurrentSynSymbolIdx,
    ) -> Option<SymbolSignature> {
        self.symbol_signatures
            .current_symbol_map()
            .get(current_symbol_idx.index())
            .copied()
    }
}
