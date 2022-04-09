use infer_contract::*;
use infer_ty::*;

pub trait InferQueryGroup: InferContractQueryGroup + InferTyQueryGroup {}
