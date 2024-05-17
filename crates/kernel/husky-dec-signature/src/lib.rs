pub mod engine;
pub mod error;
pub mod jar;
pub mod parameter;
pub mod region;
pub mod signature;
#[cfg(test)]
mod tests;

use self::error::*;
use self::jar::DecSignatureJar as Jar;
use self::parameter::*;
use self::region::error::*;
use self::region::{
    syn_expr_dec_term_region,
    variable::{DecSymbolicVariableRegion, DecSymbolicVariableSignature},
    SynExprDecTermRegion,
};
use husky_coword::*;
use husky_dec_term::jar::DecTermDb;
use husky_dec_term::{term::*, *};
use husky_entity_path::*;
use husky_syn_decl::{decl::*, *};
use husky_term_prelude::*;
use smallvec::*;

type SmallVecImpl<T> = SmallVec<[T; 2]>;
