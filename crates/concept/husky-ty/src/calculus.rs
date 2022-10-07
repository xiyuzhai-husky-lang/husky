use crate::*;

impl Ty {
    pub fn ty(&self) -> Self {
        match self {
            Ty::Entity(_)
            | Ty::TemplateInstantiation { .. }
            | Ty::ThickFp { .. }
            | Ty::Template { .. }
            | Ty::Prop => Ty::Type(Universe::zero()),
            Ty::Type(u) => Ty::Type(u.next()),
        }
    }
}
