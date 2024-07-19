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
    LinketImpls,
    FnLinketImpl,
    GnLinketImpl,
    EnumVariantConstructorLinketImpl,
    EnumVariantDestructorLinketImpl,
    EnumVariantDiscriminatorLinketImpl,
    EnumVariantFieldLinketImpl,
    StructDestructorLinketImpl,
    StructFieldLinketImpl,
    UnveilLinketImpl,
    StaticVarLinketImpl,
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
            RustMacroName::LinketImpls => "linket_impls!",
            RustMacroName::FnLinketImpl => "fn_linket_impl!",
            RustMacroName::StructDestructorLinketImpl => "struct_destructor_linket_impl!",
            RustMacroName::EnumVariantConstructorLinketImpl => {
                "enum_variant_constructor_linket_impl!"
            }
            RustMacroName::EnumVariantDestructorLinketImpl => {
                "enum_variant_destructor_linket_impl!"
            }
            RustMacroName::EnumVariantDiscriminatorLinketImpl => {
                "enum_variant_discriminator_linket_impl!"
            }
            RustMacroName::EnumVariantFieldLinketImpl => "enum_variant_field_linket_impl!",
            RustMacroName::UnveilLinketImpl => "unveil_fn_linket_impl!",
            RustMacroName::GnLinketImpl => "gn_linket_impl!",
            RustMacroName::StaticVarLinketImpl => "static_var_linket_impl!",
            RustMacroName::StructFieldLinketImpl => "struct_field_linket_impl!",
            RustMacroName::TypeDefault => "ty_default_linket_impl!",
            RustMacroName::EnumUnitPresenter => "enum_index_presenter_linket_impl!",
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
