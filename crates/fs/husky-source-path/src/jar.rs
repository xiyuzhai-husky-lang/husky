use crate::*;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

#[salsa::jar(db =SourcePathDb)]
pub struct SourcePathJar(SourcePath);
