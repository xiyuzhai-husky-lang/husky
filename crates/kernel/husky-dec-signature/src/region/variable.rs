use husky_dec_term::name::DecSymbolicVariableNameMap;
use husky_entity_tree::*;
use husky_syn_expr::*;
use husky_term_prelude::symbol::SymbolName;
use husky_vfs::Toolchain;

use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct DecSymbolicVariableRegion {
    svar_registry: TermSvarRegistry,
    variable_signatures: SymbolOrderedMap<DecSymbolicVariableSignature>,
    /// used to format dec terms
    svar_name_map: DecSymbolicVariableNameMap,
    self_ty: Option<DecTerm>,
    self_value: Option<DecSymbolicVariable>,
    self_lifetime: Option<DecSymbolicVariable>,
    self_place: Option<DecSymbolicVariable>,
    /// things like `Self` in trait
    auto_template_variables: SmallVec<[DecSymbolicVariable; 1]>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DecSymbolicVariableSignature {
    kind: VariableSignatureKind,
    symbol: Option<DecSymbolicVariable>,
    modifier: VariableModifier,
    ty: DecTermSymbolicVariableTypeResult<DecTerm>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VariableSignatureKind {
    TemplateParameter,
    ParenateParameter,
    FieldVariable,
}

impl DecSymbolicVariableSignature {
    pub fn kind(self) -> VariableSignatureKind {
        self.kind
    }

    pub fn term_symbol(self) -> Option<DecSymbolicVariable> {
        self.symbol
    }

    pub fn modifier(&self) -> VariableModifier {
        self.modifier
    }

    pub fn ty(&self) -> DecTermSymbolicVariableTypeResult<DecTerm> {
        self.ty
    }
}

impl DecSymbolicVariableRegion {
    pub fn self_lifetime(&self) -> Option<DecSymbolicVariable> {
        self.self_lifetime
    }

    pub fn self_place(&self) -> Option<DecSymbolicVariable> {
        self.self_place
    }

    pub fn auto_template_parameter_symbols(&self) -> &[DecSymbolicVariable] {
        &self.auto_template_variables
    }

    pub(crate) fn svar_registry_mut(&mut self) -> &mut TermSvarRegistry {
        &mut self.svar_registry
    }

    #[inline(always)]
    pub(crate) fn add_new_template_parameter_symbol_signature(
        &mut self,
        db: &::salsa::Db,
        idx: CurrentVariableIdx,
        ty: DecTermSymbolicVariableTypeResult<DecTerm>,
        term_symbol: DecSymbolicVariable,
        name: SymbolName,
    ) {
        self.add_new_current_syn_symbol_signature(
            db,
            idx,
            DecSymbolicVariableSignature {
                kind: VariableSignatureKind::TemplateParameter,
                symbol: Some(term_symbol),
                ty,
                modifier: VariableModifier::Const,
            },
            name,
        )
    }

    #[inline(always)]
    pub(crate) fn add_new_parenate_parameter_symbol_signature(
        &mut self,
        db: &::salsa::Db,
        current_syn_symbol: CurrentVariableIdx,
        modifier: VariableModifier,
        ty: DecTermSymbolicVariableTypeResult<DecTerm>,
        name: SymbolName,
    ) {
        let symbol = match modifier {
            VariableModifier::Const => todo!(),
            _ => None,
        };
        self.add_new_current_syn_symbol_signature(
            db,
            current_syn_symbol,
            DecSymbolicVariableSignature {
                kind: VariableSignatureKind::ParenateParameter,
                modifier,
                ty,
                symbol,
            },
            name,
        )
    }

    #[inline(always)]
    pub(crate) fn add_new_field_variable_symbol_signature(
        &mut self,
        db: &::salsa::Db,
        current_syn_symbol: CurrentVariableIdx,
        ty: DecTermSymbolicVariableTypeResult<DecTerm>,
        ident: Ident,
    ) {
        self.add_new_current_syn_symbol_signature(
            db,
            current_syn_symbol,
            DecSymbolicVariableSignature {
                kind: VariableSignatureKind::FieldVariable,
                modifier: VariableModifier::Pure,
                ty,
                symbol: None,
            },
            ident.into(),
        )
    }

    #[inline(always)]
    fn add_new_current_syn_symbol_signature(
        &mut self,
        db: &::salsa::Db,
        idx: CurrentVariableIdx,
        signature: DecSymbolicVariableSignature,
        name: SymbolName,
    ) {
        if let Some(symbol) = signature.symbol {
            self.svar_name_map.add(symbol, name)
        }
        self.variable_signatures.insert_next(idx, signature)
    }

    pub fn symbol_name_map(&self) -> &DecSymbolicVariableNameMap {
        &self.svar_name_map
    }
}

impl DecSymbolicVariableRegion {
    /// will initialize `inherited_syn_symbol_terms`;
    /// but will leave current_syn_symbol_terms unintialized;
    /// `self_ty_term` is set to that of parent if parent exists, otherwise none;
    /// `self_value_term` is set to that of parent if parent exists, otherwise none
    pub(crate) fn new(
        parent: Option<&DecSymbolicVariableRegion>,
        syn_expr_region_data: &SynExprRegionData,
        dec_term_menu: &DecTermMenu,
    ) -> Self {
        let registry = parent.map_or(Default::default(), |parent| parent.svar_registry.clone());
        let implicit_self_lifetime = syn_expr_region_data
            .has_self_lifetime()
            .then_some(dec_term_menu.implicit_self_lifetime());
        let implicit_self_place = syn_expr_region_data
            .has_self_place()
            .then_some(dec_term_menu.implicit_self_place());
        let symbol_name_map =
            parent.map_or(Default::default(), |parent| parent.svar_name_map.clone());
        Self {
            svar_registry: registry,
            variable_signatures: SymbolOrderedMap::new(
                parent.map(|parent| &parent.variable_signatures),
            ),
            svar_name_map: symbol_name_map,
            self_ty: parent.map(|parent| parent.self_ty).flatten(),
            self_value: parent.map(|parent| parent.self_value).flatten(),
            self_lifetime: implicit_self_lifetime,
            self_place: implicit_self_place,
            auto_template_variables: implicit_self_lifetime
                .into_iter()
                .chain(implicit_self_place)
                .collect(),
        }
    }

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
                    MajorItemSynNodePath::Trait(_),
                )) => Some(self.new_self_ty_symbol(toolchain, db).into()),
                SynNodeRegionPath::Decl(ItemSynNodePath::MajorItem(
                    MajorItemSynNodePath::Type(ty_node_path),
                )) => Some(
                    self.ty_defn_self_ty_term(
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
                                TypeSketch::DeriveAny => {
                                    Some(self.new_self_ty_symbol(toolchain, db).into())
                                }
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
                    &mut self.svar_registry,
                    self.self_ty.expect("self type should exists"),
                )
                .into(),
            )
        }
    }
    fn new_self_ty_symbol(
        &mut self,
        toolchain: Toolchain,
        db: &::salsa::Db,
    ) -> DecSymbolicVariable {
        let var = DecSymbolicVariable::new_self_ty(db, toolchain, &mut self.svar_registry);
        self.auto_template_variables.push(var);
        var
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
    fn ty_defn_self_ty_term(&self, db: &::salsa::Db, ty_path: TypePath) -> DecTerm {
        let mut self_ty: DecTerm = DecItemPath::Type(ty_path.into()).into();
        for current_syn_symbol_signature in self
            .variable_signatures
            .current_syn_symbol_map()
            .iter()
            .copied()
        {
            match current_syn_symbol_signature.kind {
                VariableSignatureKind::TemplateParameter => {
                    let argument = current_syn_symbol_signature
                        .term_symbol()
                        .expect("should have term");
                    self_ty = self_ty.apply(db, argument)
                }
                VariableSignatureKind::ParenateParameter => unreachable!(),
                VariableSignatureKind::FieldVariable => break,
            }
        }
        self_ty
    }

    pub fn self_ty(&self) -> Option<DecTerm> {
        self.self_ty
    }

    pub(crate) fn set_self_ty(&mut self, self_ty: Option<DecTerm>) {
        debug_assert!(self.self_ty.is_none());
        self.self_ty = self_ty
    }

    pub fn self_value(&self) -> Option<DecSymbolicVariable> {
        self.self_value
    }

    pub fn inherited_variable_signature(
        &self,
        inherited_variable_idx: InheritedSymbolicVariableIdx,
    ) -> DecSymbolicVariableSignature {
        self.variable_signatures[inherited_variable_idx]
    }

    /// None for variables defined in the body
    pub fn current_parameter_variable_signature(
        &self,
        current_variable_idx: CurrentVariableIdx,
    ) -> Option<DecSymbolicVariableSignature> {
        self.variable_signatures
            .current_syn_symbol_map()
            .get(current_variable_idx.index())
            .copied()
    }
}
