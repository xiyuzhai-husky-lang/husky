use infer_contract::*;
use infer_entity_route::*;

pub trait InferQueryGroup: InferContractQueryGroup + InferTyQueryGroup {}
