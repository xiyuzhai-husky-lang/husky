use super::*;
use husky_entity_tree::RegionPath;
use husky_expr::{
    CurrentSymbolIdx, ExprIdx, ExprMap, ExprRegion, InheritedSymbolIdx, ParentSymbolIdx,
    SymbolRegion,
};

#[derive(Debug, PartialEq, Eq)]
pub struct SignatureTermRegion {
    path: RegionPath,
    term_symbol_region: RawTermSymbolRegion,
    expr_terms: ExprMap<SignatureRawTermResult<RawTerm>>,
}

impl SignatureTermRegion {
    pub fn new(
        path: RegionPath,
        term_symbol_region: RawTermSymbolRegion,
        expr_terms: ExprMap<SignatureRawTermResult<RawTerm>>,
    ) -> Self {
        Self {
            path,
            term_symbol_region,
            expr_terms,
        }
    }

    pub fn term_symbol_region(&self) -> &RawTermSymbolRegion {
        &self.term_symbol_region
    }

    pub fn current_symbol_term(
        &self,
        current_symbol_idx: CurrentSymbolIdx,
    ) -> Option<RawTermSymbol> {
        self.term_symbol_region
            .current_symbol_term(current_symbol_idx)
    }

    pub fn expr_term(&self, expr: ExprIdx) -> SignatureRawTermResultBorrowed<RawTerm> {
        self.expr_terms[expr].as_ref().copied()
    }

    pub fn path(&self) -> RegionPath {
        self.path
    }
}

#[salsa::tracked(jar = SignatureJar, return_ref)]
pub(crate) fn signature_term_region(
    db: &dyn SignatureDb,
    expr_region: ExprRegion,
) -> SignatureTermRegion {
    let expr_region_data = expr_region.data(db);
    let parent_expr_region = expr_region_data.parent();
    let parent_term_symbol_region =
        parent_expr_region.map(|r| signature_term_region(db, r).term_symbol_region());
    SignatureRawTermEngine::new(db, expr_region, parent_term_symbol_region).finish()
}

#[derive(Debug, PartialEq, Eq)]
pub struct RawTermSymbolRegion {
    registry: TermSymbolRegistry,
    inherited_symbol_terms: Vec<RawTermSymbol>,
    current_symbol_terms: Vec<RawTermSymbol>,
}

impl RawTermSymbolRegion {
    pub(super) fn new(parent: Option<&RawTermSymbolRegion>, symbol_region: &SymbolRegion) -> Self {
        let registry = parent.map_or(Default::default(), |parent| parent.registry.clone());
        let inherited_symbol_terms = symbol_region
            .inherited_symbol_iter()
            .map(|symbol| {
                parent
                    .unwrap()
                    .parent_symbol_term(symbol.parent_symbol_idx())
            })
            .collect();
        Self {
            registry,
            inherited_symbol_terms,
            current_symbol_terms: Default::default(),
        }
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
        self.inherited_symbol_terms[inherited_symbol_idx.raw()]
    }

    pub fn current_symbol_term(
        &self,
        current_symbol_idx: CurrentSymbolIdx,
    ) -> Option<RawTermSymbol> {
        self.current_symbol_terms
            .get(current_symbol_idx.raw())
            .copied()
    }

    pub(super) fn new_symbol(
        &mut self,
        db: &dyn SignatureDb,
        ty: Result<RawTerm, RawTermSymbolTypeErrorKind>,
    ) {
        self.current_symbol_terms
            .push(self.registry.new_symbol(db, ty))
    }
}
