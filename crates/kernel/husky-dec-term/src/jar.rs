use crate::*;

pub trait DecTermDb {
    fn dec_term_menu(&self, toolchain: Toolchain) -> DecTermResultRef<&DecTermMenu>;
}

impl DecTermDb for ::salsa::Db {
    fn dec_term_menu(&self, toolchain: Toolchain) -> DecTermResultRef<&DecTermMenu> {
        dec_term_menu(self, toolchain).as_ref()
    }
}

#[salsa::jar]
pub struct DecTermJar(
    DecSymbolicVariable,
    DecTermSymbols,
    declarative_term_curry_symbols,
    declarative_term_ritchie_symbols,
    application_declarative_term_symbols,
    DecLambdaVariable,
    DecTermHvars,
    declarative_term_curry_placeholders,
    declarative_term_ritchie_hvars,
    declarative_term_application_hvars,
    DecCurry,
    curry_parameter_count,
    DecRitchie,
    crate::term::abstraction::DecAbstraction,
    crate::term::application::DecApplication,
    crate::term::application_or_ritchie_call::DecApplicationOrRitchieCall,
    crate::term::assoc_item::DecAssocItem,
    crate::term::ty_as_trai_item::DecTypeAsTraitItem,
    crate::term::constraint::DecTraitConstraint,
    crate::term::list::DecList,
    dec_term_menu,
    DecWrapper,
);
