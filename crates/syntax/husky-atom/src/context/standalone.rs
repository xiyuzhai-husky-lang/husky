use super::*;

pub struct AtomContextStandalone<'a> {
    pub opt_file: Option<FileItd>,
    pub db: &'a dyn EntitySyntaxQueryGroup,
    pub opt_this_ty: Option<Ty>,
    pub opt_this_contract: Option<ParameterModifier>,
    pub symbols: Cow<'a, [Symbol]>,
    pub kind: AtomContextKind<'a>,
}

impl<'a> AtomContext<'a> for AtomContextStandalone<'a> {
    fn entity_syntax_db(&self) -> &'a dyn EntitySyntaxQueryGroup {
        self.db
    }

    fn opt_this_ty(&self) -> Option<Ty> {
        self.opt_this_ty
    }

    fn opt_this_liason(&self) -> Option<ParameterModifier> {
        self.opt_this_contract
    }

    fn symbols(&self) -> &[Symbol] {
        &self.symbols
    }

    fn kind(&self) -> AtomContextKind {
        self.kind
    }

    fn push_abs_semantic_token(&mut self, _new_token: AbsSemanticToken) {}

    fn as_dyn_mut(&mut self) -> &mut dyn AtomContext<'a> {
        self
    }

    fn save_state(&self) -> AtomContextState {
        Default::default()
    }

    fn rollback(&mut self, _state: AtomContextState) {}

    fn push_symbol(&mut self, _new_symbol: Symbol) {
        todo!()
    }

    fn file(&self) -> FileItd {
        self.opt_file.unwrap()
    }
}
