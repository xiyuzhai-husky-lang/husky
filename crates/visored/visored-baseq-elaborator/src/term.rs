pub mod builder;
pub mod comnum;
pub mod litnum;
pub mod num;
pub mod prop;

use self::{comnum::*, litnum::*, num::*, prop::*};
use crate::{
    elaborator::VdBsqElaboratorInner,
    expr::{VdBsqExprFld, VdBsqExprFldData},
    foundations::opr::separator::relation::comparison::VdBsqComparisonOpr,
};
use bigint::VdBsqBigInt;
use builder::{product::VdBsqProductBuilder, sum::VdBsqSumBuilder};
use either::*;
use floated_sequential::db::FloaterDb;
use floated_sequential::floated;
use product::VdBsqProductStem;
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;
use visored_mir_expr::{
    expr::{application::VdMirFunc, VdMirExprData, VdMirExprEntry},
    symbol::local_defn::{
        storage::VdMirSymbolLocalDefnStorage, VdMirSymbolLocalDefnHead, VdMirSymbolLocalDefnIdx,
    },
};
use visored_mir_opr::{
    opr::{binary::VdMirBaseBinaryOpr, prefix::VdMirBasePrefixOpr},
    separator::VdMirBaseSeparator,
};
use visored_opr::precedence::VdPrecedenceRange;
use visored_term::{
    term::{literal::VdLiteralData, VdTermData},
    ty::VdType,
};

#[enum_class::from_variants]
#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqTerm<'sess> {
    Litnum(VdBsqLitnumTerm<'sess>),
    Comnum(VdBsqComnumTerm<'sess>),
    Prop(VdBsqPropTerm<'sess>),
}

impl<'sess> VdBsqNumTerm<'sess> {
    pub fn product_or_non_product(
        self,
    ) -> Either<(VdBsqLitnumTerm<'sess>, VdBsqProductStem<'sess>), VdBsqNonProductNumTerm<'sess>>
    {
        match self {
            VdBsqNumTerm::Litnum(term) => todo!(),
            VdBsqNumTerm::Comnum(term) => match term {
                VdBsqComnumTerm::Atom(term) => Right(VdBsqNonProductNumTerm::Atom(term)),
                VdBsqComnumTerm::Sum(term) => Right(VdBsqNonProductNumTerm::Sum(term)),
                VdBsqComnumTerm::Product(product) => {
                    Left((product.litnum_factor(), product.base()))
                }
            },
        }
    }
}

impl<'sess> VdBsqTerm<'sess> {
    pub fn num(self) -> Option<VdBsqNumTerm<'sess>> {
        match self {
            VdBsqTerm::Litnum(litnum) => Some(VdBsqNumTerm::Litnum(litnum)),
            VdBsqTerm::Comnum(comnum) => Some(VdBsqNumTerm::Comnum(comnum)),
            VdBsqTerm::Prop(_) => None,
        }
    }
}

impl<'sess> std::fmt::Debug for VdBsqTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("VdBsqTerm(`")?;
        self.show_fmt(VdPrecedenceRange::Any, f)?;
        f.write_str("`)")
    }
}

impl<'sess> VdBsqTerm<'sess> {
    pub fn show_fmt(
        self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            VdBsqTerm::Litnum(litnum) => litnum.show_fmt(precedence_range, f),
            VdBsqTerm::Comnum(comnum) => comnum.show_fmt(precedence_range, f),
            VdBsqTerm::Prop(prop) => prop.show_fmt(precedence_range, f),
        }
    }
}

impl<'sess> std::fmt::Debug for VdBsqNumTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("VdBsqNumTerm(`")?;
        self.show_fmt(VdPrecedenceRange::Any, f)?;
        f.write_str("`)")
    }
}

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub fn calc_expr_term(&self, expr: &VdBsqExprFldData<'sess>, ty: VdType) -> VdBsqTerm<'sess> {
        let db = self.floater_db();
        match *expr {
            VdBsqExprFldData::Literal(vd_literal) => match *vd_literal.data() {
                VdLiteralData::Int128(i) => VdBsqTerm::Litnum(VdBsqLitnumTerm::Int128(i)),
                VdLiteralData::BigInt(ref n) => {
                    VdBsqTerm::Litnum(VdBsqLitnumTerm::BigInt(VdBsqBigInt::new(n.clone(), db)))
                }
                VdLiteralData::Float(_) => todo!(),
                VdLiteralData::SpecialConstant(vd_special_constant) => todo!(),
            },
            VdBsqExprFldData::Variable(lx_math_letter, local_defn_idx) => {
                if ty.is_numeric(self.eterner_db()) {
                    if let Some(_) = self.eval_variable() {
                        todo!()
                    } else {
                        VdBsqTerm::new_numeric_variable(
                            lx_math_letter,
                            local_defn_idx,
                            self.floater_db(),
                        )
                    }
                } else {
                    todo!()
                }
            }
            VdBsqExprFldData::Application {
                function,
                ref arguments,
            } => match function {
                VdMirFunc::NormalBasePrefixOpr(signature) => match signature.opr {
                    VdMirBasePrefixOpr::RingPos => arguments[0].term(),
                    VdMirBasePrefixOpr::RingNeg => arguments[0]
                        .term()
                        .num()
                        .unwrap()
                        .neg(self.floater_db())
                        .into(),
                    _ => todo!(),
                },
                VdMirFunc::NormalBaseSeparator(signature) => todo!(),
                VdMirFunc::NormalBaseBinaryOpr(signature) => match signature.opr {
                    VdMirBaseBinaryOpr::CommRingSub => {
                        let lopd = arguments[0].term().num().unwrap();
                        let ropd = arguments[1].term().num().unwrap();
                        lopd.sub(ropd, self.floater_db()).into()
                    }
                    VdMirBaseBinaryOpr::CommFieldDiv => {
                        let lopd = arguments[0].term().num().unwrap();
                        let ropd = arguments[1].term().num().unwrap();
                        lopd.div(ropd, self.floater_db()).into()
                    }
                },
                VdMirFunc::Power(signature) => {
                    assert_eq!(arguments.len(), 2);
                    let Some(base) = arguments[0].term().num() else {
                        todo!()
                    };
                    let Some(exponent) = arguments[1].term().num() else {
                        todo!()
                    };
                    match base.product_or_non_product() {
                        Either::Left(base) => todo!(),
                        Either::Right(base) => {
                            VdBsqTerm::new_power(base, exponent, self.floater_db())
                        }
                    }
                }
                VdMirFunc::InSet => todo!(),
                VdMirFunc::NormalBaseSqrt(vd_base_sqrt_signature) => todo!(),
            },
            VdBsqExprFldData::FoldingSeparatedList {
                leader,
                ref followers,
            } => {
                let (func, follower) = *followers.first().unwrap();
                let num_relationship = |slf: &Self, kind| {
                    VdBsqTerm::new_num_relationship(
                        leader.term().num().unwrap(),
                        kind,
                        follower.term().num().unwrap(),
                        self.floater_db(),
                    )
                };
                match func {
                    VdMirFunc::NormalBasePrefixOpr(signature) => todo!(),
                    VdMirFunc::NormalBaseSeparator(signature) => match signature.opr() {
                        VdMirBaseSeparator::CommRingAdd => {
                            let mut builder = VdBsqSumBuilder::new(self.floater_db());
                            builder.add_num(leader.term().num().unwrap());
                            for &(_, follower) in followers.iter() {
                                builder.add_num(follower.term().num().unwrap());
                            }
                            builder.finish().into()
                        }
                        VdMirBaseSeparator::CommRingMul => {
                            let mut builder = VdBsqProductBuilder::new(self.floater_db());
                            builder.mul_num(leader.term().num().unwrap());
                            for &(_, follower) in followers.iter() {
                                builder.mul_num(follower.term().num().unwrap());
                            }
                            builder.finish().into()
                        }
                        VdMirBaseSeparator::Eq => todo!(),
                        VdMirBaseSeparator::Ne => todo!(),
                        VdMirBaseSeparator::Lt => todo!(),
                        VdMirBaseSeparator::Gt => todo!(),
                        VdMirBaseSeparator::Le => todo!(),
                        VdMirBaseSeparator::Ge => todo!(),
                        VdMirBaseSeparator::Subset => todo!(),
                        VdMirBaseSeparator::Supset => todo!(),
                        VdMirBaseSeparator::Subseteq => todo!(),
                        VdMirBaseSeparator::Supseteq => todo!(),
                        VdMirBaseSeparator::Subseteqq => todo!(),
                        VdMirBaseSeparator::Supseteqq => todo!(),
                        VdMirBaseSeparator::Subsetneq => todo!(),
                        VdMirBaseSeparator::Supsetneq => todo!(),
                        VdMirBaseSeparator::In => todo!(),
                        VdMirBaseSeparator::Notin => todo!(),
                        VdMirBaseSeparator::SetTimes => todo!(),
                        VdMirBaseSeparator::TensorOtimes => todo!(),
                    },
                    VdMirFunc::NormalBaseBinaryOpr(signature) => todo!(),
                    VdMirFunc::Power(signature) => todo!(),
                    VdMirFunc::InSet => todo!(),
                    VdMirFunc::NormalBaseSqrt(vd_base_sqrt_signature) => todo!(),
                }
            }
            VdBsqExprFldData::ChainingSeparatedList {
                leader,
                ref followers,
                joined_signature: joined_separator_and_signature,
            } => match joined_separator_and_signature {
                Some(joined_separator_and_signature) => todo!(),
                None => {
                    use VdBsqComparisonOpr::*;

                    let (func, follower) = *followers.first().unwrap();
                    let num_relationship = |slf: &Self, kind| {
                        VdBsqTerm::new_num_relationship(
                            leader.term().num().unwrap(),
                            kind,
                            follower.term().num().unwrap(),
                            self.floater_db(),
                        )
                    };
                    match func {
                        VdMirFunc::NormalBasePrefixOpr(signature) => todo!(),
                        VdMirFunc::NormalBaseSeparator(signature) => match signature.opr() {
                            VdMirBaseSeparator::CommRingAdd => todo!(),
                            VdMirBaseSeparator::CommRingMul => todo!(),
                            VdMirBaseSeparator::Eq => {
                                num_relationship(self, VdBsqComparisonOpr::EQ)
                            }
                            VdMirBaseSeparator::Ne => {
                                num_relationship(self, VdBsqComparisonOpr::NE)
                            }
                            VdMirBaseSeparator::Lt => {
                                num_relationship(self, VdBsqComparisonOpr::LT)
                            }
                            VdMirBaseSeparator::Gt => {
                                num_relationship(self, VdBsqComparisonOpr::GT)
                            }
                            VdMirBaseSeparator::Le => {
                                num_relationship(self, VdBsqComparisonOpr::LE)
                            }
                            VdMirBaseSeparator::Ge => {
                                num_relationship(self, VdBsqComparisonOpr::GE)
                            }
                            VdMirBaseSeparator::Subset => todo!(),
                            VdMirBaseSeparator::Supset => todo!(),
                            VdMirBaseSeparator::Subseteq => todo!(),
                            VdMirBaseSeparator::Supseteq => todo!(),
                            VdMirBaseSeparator::Subseteqq => todo!(),
                            VdMirBaseSeparator::Supseteqq => todo!(),
                            VdMirBaseSeparator::Subsetneq => todo!(),
                            VdMirBaseSeparator::Supsetneq => todo!(),
                            VdMirBaseSeparator::In => todo!(),
                            VdMirBaseSeparator::Notin => todo!(),
                            VdMirBaseSeparator::SetTimes => todo!(),
                            VdMirBaseSeparator::TensorOtimes => todo!(),
                        },
                        VdMirFunc::NormalBaseBinaryOpr(signature) => todo!(),
                        VdMirFunc::Power(signature) => todo!(),
                        VdMirFunc::InSet => todo!(),
                        VdMirFunc::NormalBaseSqrt(vd_base_sqrt_signature) => todo!(),
                    }
                }
            },
            VdBsqExprFldData::ItemPath(vd_item_path) => todo!(),
        }
    }
}
