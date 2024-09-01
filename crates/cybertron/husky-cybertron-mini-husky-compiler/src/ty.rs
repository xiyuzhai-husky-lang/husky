use crate::token::ident::Ident;

pub mod checking;
pub mod signature;

pub enum Type {
    Ident(Ident),
    Option(Ident),
    Vec(Ident),
}
