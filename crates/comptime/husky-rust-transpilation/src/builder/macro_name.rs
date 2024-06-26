use super::*;

pub enum RustMacroName {
    Vec,
    Println,
    Todo,
    Unreachable,
    Panic,
    Require,
    Assert,
    Matches,
    Unveil,
    LinkageImpls,
    FnLinkageImpl,
    GnLinkageImpl,
    EnumVariantConstructorLinkageImpl,
    EnumVariantDestructorLinkageImpl,
    EnumVariantDiscriminatorLinkageImpl,
    EnumVariantFieldLinkageImpl,
    StructDestructorLinkageImpl,
    StructFieldLinkageImpl,
    UnveilLinkageImpl,
    TypeDefault,
    EnumUnitPresenter,
    HtmlTag(husky_coword::Ident),
}

impl RustMacroName {}

impl<'a, 'b, E> RustTranspilationBuilder<'a, 'b, E> {
    pub(crate) fn macro_name(&mut self, macro_name: RustMacroName) {
        let db = self.db;
        if self.result.ends_with(|c: char| c.is_alphabetic()) {
            self.write_str(" ")
        }
        self.write_str(match macro_name {
            RustMacroName::Vec => "vec!",
            RustMacroName::Println => "println!",
            RustMacroName::Todo => "todo!",
            RustMacroName::Unreachable => "unreachable!",
            RustMacroName::Panic => "panic!",
            RustMacroName::Require => "require!",
            RustMacroName::Assert => "assert!",
            RustMacroName::Matches => "matches!",
            RustMacroName::Unveil => "unveil!",
            RustMacroName::LinkageImpls => "linkage_impls!",
            RustMacroName::FnLinkageImpl => "fn_linkage_impl!",
            RustMacroName::StructDestructorLinkageImpl => "struct_destructor_linkage_impl!",
            RustMacroName::EnumVariantConstructorLinkageImpl => {
                "enum_variant_constructor_linkage_impl!"
            }
            RustMacroName::EnumVariantDestructorLinkageImpl => {
                "enum_variant_destructor_linkage_impl!"
            }
            RustMacroName::EnumVariantDiscriminatorLinkageImpl => {
                "enum_variant_discriminator_linkage_impl!"
            }
            RustMacroName::EnumVariantFieldLinkageImpl => "enum_variant_field_linkage_impl!",
            RustMacroName::UnveilLinkageImpl => "unveil_linkage_impl!",
            RustMacroName::GnLinkageImpl => "gn_linkage_impl!",
            RustMacroName::StructFieldLinkageImpl => "struct_field_linkage_impl!",
            RustMacroName::TypeDefault => "ty_default_linkage_impl!",
            RustMacroName::EnumUnitPresenter => "enum_index_presenter_linkage_impl!",
            RustMacroName::HtmlTag(ident) => {
                self.result += ident.data(db);
                self.result += "!";
                return;
            }
        })
    }

    pub(crate) fn macro_call(&mut self, macro_name: RustMacroName, f: impl FnOnce(&mut Self)) {
        self.macro_name(macro_name);
        self.delimited(RustDelimiter::Par, f)
    }
}
