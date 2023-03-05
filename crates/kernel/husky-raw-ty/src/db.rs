use crate::*;

pub trait RawTypeDb: salsa::DbWithJar<RawTypeJar> + SignatureDb {
    fn ty_method_raw_ty(
        &self,
        raw_ty: RawTerm,
        ident: Identifier,
    ) -> RawTypeResult<Option<RawTerm>>;
    fn field_raw_ty(&self, raw_ty: RawTerm, ident: Identifier) -> RawTypeResult<Option<RawTerm>>;
    fn raw_term_application_expansion(&self, reduced_raw_term: RawTerm) -> ApplicationExpansion;
    fn raw_ty_call_raw_ty(
        &self,
        raw_term: RawTerm,
        toolchain: Toolchain,
        reduced_raw_term_menu: RawTermMenu,
    ) -> RawTypeResult<RawTerm>;
    fn raw_term_contains_symbol(&self, raw_term: RawTerm, symbol: RawTermSymbol) -> bool;
    fn ty_path_raw_ty(
        &self,
        path: TypePath,
        disambiguation: TypePathDisambiguation,
    ) -> RawTypeResult<RawTerm>;
    fn trai_path_raw_ty(&self, trai_path: TraitPath) -> RawTypeResult<RawTerm>;
    fn form_path_raw_ty(&self, form_path: FormPath) -> RawTypeResult<RawTerm>;
}

impl<Db> RawTypeDb for Db
where
    Db: salsa::DbWithJar<RawTypeJar> + SignatureDb,
{
    fn ty_method_raw_ty(
        &self,
        raw_ty: RawTerm,
        ident: Identifier,
    ) -> RawTypeResult<Option<RawTerm>> {
        raw_ty_method_raw_ty(self, raw_ty, ident)
    }

    fn field_raw_ty(&self, raw_ty: RawTerm, ident: Identifier) -> RawTypeResult<Option<RawTerm>> {
        field_raw_ty(self, raw_ty, ident)
    }

    // fn intrinsic_raw_ty(&self, raw_ty:  RawTerm) -> IntrinsicRawType {
    //     intrinsic_raw_ty(self, raw_ty)
    // }

    fn raw_term_application_expansion(&self, reduced_raw_term: RawTerm) -> ApplicationExpansion {
        application_expansion(self, reduced_raw_term)
    }

    fn raw_ty_call_raw_ty(
        &self,
        raw_ty_raw_term: RawTerm,
        toolchain: Toolchain,
        reduced_raw_term_menu: RawTermMenu,
    ) -> RawTypeResult<RawTerm> {
        raw_ty_call_raw_ty(self, raw_ty_raw_term, toolchain, reduced_raw_term_menu)
    }

    fn raw_term_contains_symbol(&self, raw_term: RawTerm, symbol: RawTermSymbol) -> bool {
        calc_raw_term_symbols(self, raw_term)
            .map(|raw_term_symbols| raw_term_symbols.contains(self, symbol))
            .unwrap_or_default()
    }
    fn ty_path_raw_ty(
        &self,
        path: TypePath,
        disambiguation: TypePathDisambiguation,
    ) -> RawTypeResult<RawTerm> {
        ty_path_raw_ty(self, path, disambiguation)
    }

    fn trai_path_raw_ty(&self, trai_path: TraitPath) -> RawTypeResult<RawTerm> {
        trai_path_raw_ty(self, trai_path)
    }

    fn form_path_raw_ty(&self, form_path: FormPath) -> RawTypeResult<RawTerm> {
        form_path_raw_ty(self, form_path)
    }
}
