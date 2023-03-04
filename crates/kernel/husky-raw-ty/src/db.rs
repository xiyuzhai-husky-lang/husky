use crate::*;

pub trait RawTypeDb: salsa::DbWithJar<RawTypeJar> + SignatureDb {
    fn ty_method_raw_ty(
        &self,
        raw_ty: ReducedRawTerm,
        ident: Identifier,
    ) -> RawTypeResult<Option<ReducedRawTerm>>;
    fn field_raw_ty(
        &self,
        raw_ty: ReducedRawTerm,
        ident: Identifier,
    ) -> RawTypeResult<Option<ReducedRawTerm>>;
    fn reduced_raw_term(&self, raw_term: RawTerm) -> ReducedRawTerm;
    // fn intrinsic_raw_ty(&self, raw_ty: ReducedRawTerm) -> IntrinsicRawType;
    fn reduced_raw_term_menu<'a>(
        &'a self,
        toolchain: Toolchain,
    ) -> Result<ReducedRawTermMenu<'a>, &'a RawTermError>;
    fn raw_term_application_expansion(
        &self,
        reduced_raw_term: ReducedRawTerm,
    ) -> ApplicationExpansion;
    fn raw_ty_call_raw_ty(
        &self,
        raw_term: ReducedRawTerm,
        toolchain: Toolchain,
        reduced_raw_term_menu: ReducedRawTermMenu,
    ) -> RawTypeResult<ReducedRawTerm>;
    fn raw_term_contains_symbol(&self, raw_term: RawTerm, symbol: RawTermSymbol) -> bool;
    fn raw_ty_path_raw_ty(
        &self,
        path: TypePath,
        disambiguation: TypePathDisambiguation,
    ) -> RawTypeResult<ReducedRawTerm>;
    fn trai_path_raw_ty(&self, trai_path: TraitPath) -> RawTypeResult<ReducedRawTerm>;
    fn form_path_raw_ty(&self, form_path: FormPath) -> RawTypeResult<ReducedRawTerm>;
}

impl<Db> RawTypeDb for Db
where
    Db: salsa::DbWithJar<RawTypeJar> + SignatureDb,
{
    fn ty_method_raw_ty(
        &self,
        raw_ty: ReducedRawTerm,
        ident: Identifier,
    ) -> RawTypeResult<Option<ReducedRawTerm>> {
        raw_ty_method_raw_ty(self, raw_ty, ident)
    }

    fn reduced_raw_term(&self, raw_term: RawTerm) -> ReducedRawTerm {
        calc_reduced_raw_term(self, raw_term)
    }

    fn reduced_raw_term_menu<'a>(
        &'a self,
        toolchain: Toolchain,
    ) -> Result<ReducedRawTermMenu<'a>, &'a RawTermError> {
        let raw_term_menu = self.raw_term_menu(toolchain).as_ref()?;
        Ok(ReducedRawTermMenu::new(raw_term_menu))
    }

    fn field_raw_ty(
        &self,
        raw_ty: ReducedRawTerm,
        ident: Identifier,
    ) -> RawTypeResult<Option<ReducedRawTerm>> {
        field_raw_ty(self, raw_ty, ident)
    }

    // fn intrinsic_raw_ty(&self, raw_ty: ReducedRawTerm) -> IntrinsicRawType {
    //     intrinsic_raw_ty(self, raw_ty)
    // }

    fn raw_term_application_expansion(
        &self,
        reduced_raw_term: ReducedRawTerm,
    ) -> ApplicationExpansion {
        application_expansion(self, reduced_raw_term)
    }

    fn raw_ty_call_raw_ty(
        &self,
        raw_ty_raw_term: ReducedRawTerm,
        toolchain: Toolchain,
        reduced_raw_term_menu: ReducedRawTermMenu,
    ) -> RawTypeResult<ReducedRawTerm> {
        raw_ty_call_raw_ty(self, raw_ty_raw_term, toolchain, reduced_raw_term_menu)
    }

    fn raw_term_contains_symbol(&self, raw_term: RawTerm, symbol: RawTermSymbol) -> bool {
        calc_raw_term_symbols(self, raw_term)
            .map(|raw_term_symbols| raw_term_symbols.contains(self, symbol))
            .unwrap_or_default()
    }
    fn raw_ty_path_raw_ty(
        &self,
        path: TypePath,
        disambiguation: TypePathDisambiguation,
    ) -> RawTypeResult<ReducedRawTerm> {
        raw_ty_path_raw_ty(self, path, disambiguation)
    }

    fn trai_path_raw_ty(&self, trai_path: TraitPath) -> RawTypeResult<ReducedRawTerm> {
        trai_path_raw_ty(self, trai_path)
    }

    fn form_path_raw_ty(&self, form_path: FormPath) -> RawTypeResult<ReducedRawTerm> {
        form_path_raw_ty(self, form_path)
    }
}
