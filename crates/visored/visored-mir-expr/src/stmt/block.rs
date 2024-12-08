use super::*;
use latex_environment::path::LxEnvironmentPath;
use visored_entity_path::environment::VdEnvironmentPath;
use visored_prelude::division::VdDivisionLevel;

#[derive(Debug, PartialEq, Eq)]
pub enum VdMirBlockMeta {
    Paragraph,
    Sentence,
    Environment(LxEnvironmentPath, VdEnvironmentPath, VdModulePath),
    Division(VdDivisionLevel, VdModulePath),
}
