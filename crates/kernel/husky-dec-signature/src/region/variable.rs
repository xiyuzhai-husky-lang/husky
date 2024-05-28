use super::*;
use husky_dec_term::name::DecSymbolicVariableNameMap;
use husky_entity_path::path::{
    impl_block::TypeSketch,
    major_item::{trai::TraitPath, ty::TypePath},
};
use husky_entity_tree::*;
use husky_syn_expr::*;
use husky_term_prelude::symbol::SymbolName;
use husky_vfs::Toolchain;
use vec_like::{SmallVecPairMap, SmallVecSet};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct DecSymbolicVariableRegion {
    registry: DecSymbolicVariableRegistry,
    signatures: SymbolOrderedMap<DecSymbolicVariableSignature>,
    /// used to format dec terms
    names: DecSymbolicVariableNameMap,
    self_ty: Option<DecTerm>,
    self_value: Option<DecSymbolicVariable>,
    self_lifetime: Option<DecSymbolicVariable>,
    self_place: Option<DecSymbolicVariable>,
    /// things like `Self` in trait
    autos: SmallVec<[DecSymbolicVariable; 1]>,
    obvious_trais_map: SmallVecPairMap<
        DecSymbolicVariable,
        DerivedSynExprDecTermResult<SmallVecSet<DecTerm, 2>>,
        4,
    >,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DecSymbolicVariableSignature {
    kind: SymbolicVariableSignatureKind,
    term: Option<DecSymbolicVariable>,
    modifier: VariableModifier,
    ty: DecSymbolicVariableTypeResult<DecTerm>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolicVariableSignatureKind {
    TemplateParameter,
    ParenateParameter,
    FieldVariable,
}

impl DecSymbolicVariableSignature {
    pub fn kind(self) -> SymbolicVariableSignatureKind {
        self.kind
    }

    pub fn term(self) -> Option<DecSymbolicVariable> {
        self.term
    }

    pub fn modifier(&self) -> VariableModifier {
        self.modifier
    }

    pub fn ty(&self) -> DecSymbolicVariableTypeResult<DecTerm> {
        self.ty
    }
}

/// # constructors

impl DecSymbolicVariableRegion {
    /// will initialize `inherited_syn_symbol_terms`;
    /// but will leave current_variable_terms unintialized;
    /// `self_ty_term` is set to that of parent if parent exists, otherwise none;
    /// `self_value_term` is set to that of parent if parent exists, otherwise none
    pub(crate) fn new(
        parent: Option<&DecSymbolicVariableRegion>,
        syn_expr_region_data: &SynExprRegionData,
        dec_term_menu: &DecTermMenu,
    ) -> Self {
        let registry = parent.map_or(Default::default(), |parent| parent.registry.clone());
        let self_lifetime = syn_expr_region_data
            .has_self_lifetime()
            .then_some(dec_term_menu.auto_self_lifetime());
        let self_place = syn_expr_region_data
            .has_self_place()
            .then_some(dec_term_menu.auto_self_place());
        let names = parent.map_or(Default::default(), |parent| parent.names.clone());
        let symbolic_variable_obvious_trais = parent.map_or(Default::default(), |parent| {
            parent.obvious_trais_map.clone()
        });
        let self_value = parent.map(|parent| parent.self_value).flatten();
        let self_ty = parent.map(|parent| parent.self_ty).flatten();
        let autos = self_lifetime.into_iter().chain(self_place).collect();
        let signatures = SymbolOrderedMap::new(parent.map(|parent| &parent.signatures));
        Self {
            registry,
            signatures,
            names,
            self_ty,
            self_value,
            self_lifetime,
            self_place,
            autos,
            obvious_trais_map: symbolic_variable_obvious_trais,
        }
    }
}

/// # getters

impl DecSymbolicVariableRegion {
    pub fn self_lifetime(&self) -> Option<DecSymbolicVariable> {
        self.self_lifetime
    }

    pub fn self_place(&self) -> Option<DecSymbolicVariable> {
        self.self_place
    }

    pub fn auto_template_parameter_symbols(&self) -> &[DecSymbolicVariable] {
        &self.autos
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
    fn ty_item_self_ty_term(&self, db: &::salsa::Db, ty_path: TypePath) -> DecTerm {
        // construction of the application
        // start with the type path
        let mut self_ty: DecTerm = DecItemPath::Type(ty_path).into();
        // recursively apply the arguments
        for current_variable_signature in self.signatures.current_variable_map().iter().copied() {
            match current_variable_signature.kind {
                SymbolicVariableSignatureKind::TemplateParameter => {
                    let argument = current_variable_signature.term().expect("should have term");
                    self_ty = self_ty.apply(db, argument)
                }
                SymbolicVariableSignatureKind::ParenateParameter => unreachable!(),
                SymbolicVariableSignatureKind::FieldVariable => break,
            }
        }
        self_ty
    }

    /// this only works on trait definitions
    ///
    /// example:
    /// ```husky
    /// enum Animal<T> where
    /// | Dog
    /// | Cat
    /// ```
    ///
    /// then self type term is `Animal T`
    fn trait_item_self_ty_term(&self, db: &::salsa::Db, trai_path: TraitPath) -> DecTerm {
        // construction of the application
        // start with the type path
        let mut self_ty: DecTerm = DecItemPath::Trait(trai_path).into();
        // recursively apply the arguments
        for current_variable_signature in self.signatures.current_variable_map().iter().copied() {
            match current_variable_signature.kind {
                SymbolicVariableSignatureKind::TemplateParameter => {
                    let argument = current_variable_signature.term().expect("should have term");
                    self_ty = self_ty.apply(db, argument)
                }
                SymbolicVariableSignatureKind::ParenateParameter => unreachable!(),
                SymbolicVariableSignatureKind::FieldVariable => break,
            }
        }
        self_ty
    }

    pub fn self_ty(&self) -> Option<DecTerm> {
        self.self_ty
    }

    pub fn symbol_name_map(&self) -> &DecSymbolicVariableNameMap {
        &self.names
    }

    pub fn self_value(&self) -> Option<DecSymbolicVariable> {
        self.self_value
    }

    pub fn inherited_variable_signature(
        &self,
        inherited_variable_idx: InheritedSymbolicVariableIdx,
    ) -> DecSymbolicVariableSignature {
        self.signatures[inherited_variable_idx]
    }

    /// None for variables defined in the body
    pub fn current_parameter_variable_signature(
        &self,
        current_variable_idx: CurrentVariableIdx,
    ) -> Option<DecSymbolicVariableSignature> {
        self.signatures
            .current_variable_map()
            .get(current_variable_idx.index())
            .copied()
    }

    pub fn symbolic_variable_obvious_trais(
        &self,
        svar: DecSymbolicVariable,
    ) -> DerivedSynExprDecTermResult<&[DecTerm]> {
        match self.obvious_trais_map.get_value(svar) {
            Some(symbolic_variable_obvious_trais) => match *symbolic_variable_obvious_trais {
                Ok(ref symbolic_variable_obvious_trais) => Ok(symbolic_variable_obvious_trais),
                Err(e) => Err(e),
            },
            None => Ok(&[]),
        }
    }

    pub fn obvious_trais_map(
        &self,
    ) -> &SmallVecPairMap<
        DecSymbolicVariable,
        DerivedSynExprDecTermResult<SmallVecSet<DecTerm, 2>>,
        4,
    > {
        &self.obvious_trais_map
    }
}

/// # getters mut

impl DecSymbolicVariableRegion {
    pub(crate) fn registry_mut(&mut self) -> &mut DecSymbolicVariableRegistry {
        &mut self.registry
    }
}

/// # actions

impl DecSymbolicVariableRegion {
    pub(crate) fn infer_self_ty_parameter_and_self_value_parameter(
        &mut self,
        db: &::salsa::Db,
        toolchain: Toolchain,
        region_path: SynNodeRegionPath,
        symbol_region: &VariableRegionData,
    ) {
        if symbol_region.allow_self_ty().to_bool() && self.self_ty.is_none() {
            self.self_ty = match region_path {
                SynNodeRegionPath::Decl(ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Trait(trai_path),
                )) => Some(
                    self.add_trai_item_self_ty_symbol(
                        toolchain,
                        trai_path
                            .unambiguous_item_path(db)
                            .expect("ambiguous path shouldn't be touched for dec signature"),
                        db,
                    )
                    .into(),
                ),
                SynNodeRegionPath::Decl(ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(ty_node_path),
                )) => Some(
                    self.ty_item_self_ty_term(
                        db,
                        ty_node_path
                            .unambiguous_item_path(db)
                            .expect("should have valid item path"),
                    ),
                ),
                SynNodeRegionPath::Decl(ItemSynNodePath::ImplBlock(syn_node_path)) => {
                    match syn_node_path {
                        ImplBlockSynNodePath::TypeImplBlock(syn_node_path) => {
                            None // reserved for later stage
                        }
                        ImplBlockSynNodePath::TraitForTypeImplBlock(impl_block_path) => {
                            match impl_block_path.ty_sketch(db) {
                                TypeSketch::DeriveAny => Some(
                                    self.trai_for_ty_impl_block_derive_any_self_ty_symbol(
                                        toolchain, db,
                                    )
                                    .into(),
                                ),
                                TypeSketch::Path(ty_path) => None, // reserved for later stage
                            }
                        }
                        ImplBlockSynNodePath::IllFormedImplBlock(_) => None,
                    }
                }
                _ => unreachable!(),
            }
        }
        if symbol_region.allow_self_value().to_bool() && self.self_value.is_none() {
            self.self_value = Some(
                DecSymbolicVariable::new_self_value(
                    db,
                    toolchain,
                    &mut self.registry,
                    self.self_ty.expect("self type should exists"),
                )
                .into(),
            )
        }
    }

    fn add_trai_item_self_ty_symbol(
        &mut self,
        toolchain: Toolchain,
        trai_path: TraitPath,
        db: &::salsa::Db,
    ) -> DecSymbolicVariable {
        let var = DecSymbolicVariable::new_self_ty(db, toolchain, &mut self.registry);
        self.autos.push(var);
        self.obvious_trais_map
            .insert_new((
                var,
                Ok(SmallVecSet::new_one_elem_set(
                    self.trait_item_self_ty_term(db, trai_path),
                )),
            ))
            .unwrap();
        var
    }

    #[deprecated(note = "todo: add obvious trai")]
    fn trai_for_ty_impl_block_derive_any_self_ty_symbol(
        &mut self,
        toolchain: Toolchain,
        db: &::salsa::Db,
    ) -> DecSymbolicVariable {
        let var = DecSymbolicVariable::new_self_ty(db, toolchain, &mut self.registry);
        self.autos.push(var);
        var
    }

    pub(crate) fn set_self_ty(&mut self, self_ty: Option<DecTerm>) {
        debug_assert!(self.self_ty.is_none());
        self.self_ty = self_ty
    }

    #[inline(always)]
    pub(crate) fn add_new_template_variable(
        &mut self,
        db: &::salsa::Db,
        idx: CurrentVariableIdx,
        ty: DecSymbolicVariableTypeResult<DecTerm>,
        var: DecSymbolicVariable,
        name: SymbolName,
        trai_expr_idxs: DerivedSynExprDecTermResult<Vec<DecTerm>>,
    ) {
        self.add_new_current_variable(
            db,
            idx,
            DecSymbolicVariableSignature {
                kind: SymbolicVariableSignatureKind::TemplateParameter,
                term: Some(var),
                ty,
                modifier: VariableModifier::Compterm,
            },
            name,
            trai_expr_idxs,
        )
    }

    #[inline(always)]
    pub(crate) fn add_new_parenate_variable(
        &mut self,
        db: &::salsa::Db,
        current_variable: CurrentVariableIdx,
        modifier: VariableModifier,
        ty: DecSymbolicVariableTypeResult<DecTerm>,
        name: SymbolName,
    ) {
        let symbol = match modifier {
            VariableModifier::Compterm => todo!(),
            _ => None,
        };
        self.add_new_current_variable(
            db,
            current_variable,
            DecSymbolicVariableSignature {
                kind: SymbolicVariableSignatureKind::ParenateParameter,
                modifier,
                ty,
                term: symbol,
            },
            name,
            Ok(vec![]),
        )
    }

    #[inline(always)]
    pub(crate) fn add_new_field_variable_symbol_signature(
        &mut self,
        db: &::salsa::Db,
        current_variable: CurrentVariableIdx,
        ty: DecSymbolicVariableTypeResult<DecTerm>,
        ident: Ident,
    ) {
        self.add_new_current_variable(
            db,
            current_variable,
            DecSymbolicVariableSignature {
                kind: SymbolicVariableSignatureKind::FieldVariable,
                modifier: VariableModifier::Pure,
                ty,
                term: None,
            },
            ident.into(),
            Ok(vec![]),
        )
    }

    #[inline(always)]
    fn add_new_current_variable(
        &mut self,
        db: &::salsa::Db,
        idx: CurrentVariableIdx,
        signature: DecSymbolicVariableSignature,
        name: SymbolName,
        symbolic_variable_obvious_trais: DerivedSynExprDecTermResult<Vec<DecTerm>>,
    ) {
        if let Some(var) = signature.term {
            self.names.add(var, name);
            match symbolic_variable_obvious_trais {
                Ok(symbolic_variable_obvious_trais) => {
                    for trai in symbolic_variable_obvious_trais {
                        self.obvious_trais_map.update_value_or_insert_with(
                            var,
                            |trais| match trais {
                                Ok(trais) => trais.insert(trai),
                                Err(_) => (),
                            },
                            || Ok(SmallVecSet::new_one_elem_set(trai)),
                        )
                    }
                }
                Err(_) => todo!(),
            }
        }
        self.signatures.insert_next(idx, signature)
    }
}
