mod dense;
mod sparse;

use super::*;
use crate::helpers::tracker::VdLeanTranspilationTracker;
use eterned::db::EternerDb;
use latex_prelude::{
    helper::tracker::{LxDocumentBodyInput, LxDocumentInput, LxPageInput},
    mode::LxMode,
};
use latex_vfs::path::LxFilePath;
use std::path::PathBuf;
