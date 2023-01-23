use husky_entity_path::TypePath;

use crate::*;

pub trait SignatureDb: salsa::DbWithJar<SignatureJar> + DeclDb + TermDb {
    fn signature(&self, decl: Decl) -> SignatureResultBorrowed<Signature>;
    fn ty_signature(&self, path: TypePath) -> SignatureResultBorrowed<TypeSignature>;
    fn trai_signature(&self, path: TraitPath) -> SignatureResultBorrowed<TraitSignature>;
}

impl<Db> SignatureDb for Db
where
    Db: salsa::DbWithJar<SignatureJar> + DeclDb + TermDb,
{
    fn signature(&self, decl: Decl) -> SignatureResultBorrowed<Signature> {
        signature(self, decl)
    }

    fn ty_signature(&self, path: TypePath) -> SignatureResultBorrowed<TypeSignature> {
        let Ok(decl) = self.ty_decl(path) else{todo!()};
        ty_signature(self, decl)
    }

    fn trai_signature(&self, path: TraitPath) -> SignatureResultBorrowed<TraitSignature> {
        let Ok(decl) = self.trai_decl(path) else{todo!()};
        trai_signature(self, decl).as_ref().map(|s| *s)
    }
}
