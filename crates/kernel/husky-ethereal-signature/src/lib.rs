#![feature(trait_upcasting)]
mod db;
mod signature;

pub use self::db::*;
pub use self::signature::*;

use husky_entity_path::*;
use husky_ethereal_term::*;
use husky_word::*;
use smallvec::*;

#[salsa::jar(db = EtherealSignatureDb)]
pub struct EtherealSignatureJar(
    // type items
    ty_method_ethereal_signature_templates_map,
    TypeMethodFnEtherealSignatureTemplate,
    ty_method_fn_ethereal_signature_template,
    TypeMethodFunctionEtherealSignatureTemplate,
);
