pub mod error;
pub mod inlay_hints;
pub mod jar;

use self::error::*;
use self::jar::InlayHintsJar as Jar;
use husky_text_protocol::range::TextRange;
use husky_vfs::path::module_path::ModulePath;
use is::Is;
