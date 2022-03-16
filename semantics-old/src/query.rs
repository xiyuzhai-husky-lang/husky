mod config;
mod entity;
pub(crate) mod infer;
mod instruction;
mod main;
mod module;

use std::sync::Arc;

pub use config::{ConfigQueryGroup, ConfigQueryGroupStorage};
pub use entity::{EntityQueryGroup, EntityQueryGroupStorage};
pub use infer::{InferQueryGroup, InferQueryGroupStorage};
pub use instruction::{InstructionQueryGroup, InstructionQueryGroupStorage};
pub use main::{MainQueryGroup, MainQueryGroupStorage};
use unique_allocator::UniqueAllocatorPtr;

use crate::{Package, SemanticResultArc};

#[salsa::query_group(SemanticQueryGroupStorage)]
pub trait SemanticQueryGroup:
    MainQueryGroup + EntityQueryGroup + ConfigQueryGroup + InstructionQueryGroup
{
    fn package(&self, main_file: file::FilePtr) -> SemanticResultArc<Package>;
}

fn package(this: &dyn SemanticQueryGroup, main_file: file::FilePtr) -> SemanticResultArc<Package> {
    let module = this.module_from_file_id(main_file)?;
    Ok(Arc::new(Package {
        ident: match module.route {
            scope::ScopeRoute::Package { ident, .. } => ident,
            _ => panic!(),
        },
        subentities: this.subentities(module)?,
        main: this.main(main_file)?,
        config: this.config(main_file)?,
    }))
}
