use crate::*;

pub trait HasSignature: Copy {
    type Signature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<Self::Signature>;
}

impl HasSignature for TypeItemDecl {
    type Signature = TypeItemSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<TypeItemSignature> {
        ty_item_signature_from_decl(db, self)
    }
}

impl HasSignature for TypeItemPath {
    type Signature = TypeItemSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<TypeItemSignature> {
        todo!()
    }
}

impl HasSignature for TypeAssociatedFnDecl {
    type Signature = TypeAssociatedFnSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<TypeAssociatedFnSignature> {
        ty_associated_fn_signature(db, self)
    }
}
