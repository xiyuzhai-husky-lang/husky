mod spatial;

use husky_check_utils::should;
use husky_entity_tree::EntityTreeDb;
pub use spatial::*;
use thin_vec::thin_vec;

use husky_identifier::Identifier;
use husky_term::Ty;
use husky_text::{RangedIdentifier, TextRange};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    ranged_ident: RangedIdentifier,
    // ranged_liason: RangedParameterLiason,
    // ranged_book_ty: Ty,
    ty: Ty,
}

impl Parameter {
    pub fn new(
        db: &dyn EntityTreeDb,
        ranged_ident: RangedIdentifier,
        // ranged_liason: RangedParameterLiason,
        // ranged_raw_ty: Ty,
    ) -> Self {
        todo!()
        // let ty = Self::synthesize_ty(db, ranged_liason.liason, ranged_raw_ty.route);
        // assert!(ranged_liason.liason.is_compatible(ty));
        // Self {
        //     ty,
        //     ranged_ident,
        //     ranged_liason,
        //     // ranged_book_ty: ranged_raw_ty,
        // }
    }

    // fn synthesize_ty(db: &dyn EntityTreeDb, liason: ParameterModifier, raw_ty: Ty) -> Ty {
    //     todo!()
    //     // match liason {
    //     //     ParameterModifier::None => raw_ty,
    //     //     ParameterModifier::Owned => raw_ty,
    //     //     ParameterModifier::OwnedMut => raw_ty,
    //     //     ParameterModifier::MemberAccess => todo!(),
    //     //     ParameterModifier::EvalRef => todo!(),
    //     //     ParameterModifier::TempRef => todo!(),
    //     //     ParameterModifier::TempRefMut => db.route_call(
    //     //         RootBuiltinIdentifier::RefMut.into(),
    //     //         thin_vec![raw_ty.into()],
    //     //     ),
    //     // }
    // }

    // pub fn liason(&self) -> ParameterModifier {
    //     self.ranged_liason.liason
    // }
    // pub fn liason_range(&self) -> TextRange {
    //     self.ranged_liason.opt_range.unwrap()
    // }

    pub fn ranged_ident(&self) -> RangedIdentifier {
        self.ranged_ident
    }

    pub fn ident(&self) -> Identifier {
        self.ranged_ident.ident
    }

    pub fn raw_ty(&self) -> Ty {
        todo!()
        // self.ranged_book_ty.route
    }

    pub fn raw_ty_range(&self) -> TextRange {
        todo!()
        // self.ranged_book_ty.range
    }

    pub fn ty(&self) -> Ty {
        self.ty
    }

    pub fn from_field(
        db: &dyn EntityTreeDb,
        ranged_ident: RangedIdentifier,
        // modifier: MemberModifier,
        member_ty: Ty,
    ) -> Self {
        todo!()
        // should!(ranged_ident
        //     .ident
        //     .as_str()
        //     .chars()
        //     .next()
        //     .unwrap()
        //     .is_lowercase());
        // Parameter::new(
        //     db,
        //     ranged_ident,
        //     RangedParameterLiason {
        //         liason: ParameterModifier::from_field(modifier),
        //         opt_range: None,
        //     },
        //     Ty {
        //         route: member_ty,
        //         range: Default::default(),
        //     },
        // )
    }
}
