use infer_contract::InferContractQueryGroup;
use infer_ty::InferTyQueryGroup;

pub trait InferQueryGroup: InferContractQueryGroup + InferTyQueryGroup {}
