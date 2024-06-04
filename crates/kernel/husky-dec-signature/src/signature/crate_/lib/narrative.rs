use super::*;
use husky_syn_decl::decl::crate_::lib::narrative::{
    default_const_excludes::LibCrateSynDeclDefaultConstExcludes, LibCrateSynDeclNarrative,
};

impl<'db> LibCrateDecSignatureBuilder<'db> {
    pub(crate) fn build_narrative(
        &mut self,
        narrative: &LibCrateSynDeclNarrative,
    ) -> DecSignatureResult<()> {
        match narrative {
            LibCrateSynDeclNarrative::DefaultConstExcludes(excludes) => {
                self.build_default_const_excludes(excludes)
            }
        }
    }

    fn build_default_const_excludes(
        &mut self,
        excludes: &LibCrateSynDeclDefaultConstExcludes,
    ) -> DecSignatureResult<()> {
        debug_assert!(
            self.default_const_excludes.is_none(),
            "it should be guaranteed by syntax that this will not happen"
        );
        let default_const_excludes = excludes
            .excludes()
            .iter()
            .map(|exclude| self.expr_term(exclude.expr()))
            .collect::<SynExprDecTermResultRef<_>>()?;
        self.default_const_excludes = Some(default_const_excludes);
        Ok(())
    }
}
