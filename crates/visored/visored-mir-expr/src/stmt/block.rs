use super::*;
use latex_environment::path::LxEnvironmentPath;
use visored_entity_path::environment::VdEnvironmentPath;
use visored_prelude::division::VdDivisionLevel;

#[derive(Debug, PartialEq, Eq)]
pub enum VdMirBlockMeta {
    Environment(LxEnvironmentPath, VdEnvironmentPath, VdModulePath),
    Division(VdDivisionLevel, VdModulePath),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdMirBlockKind {
    Environment,
    Division,
}

impl VdMirBlockMeta {
    pub fn kind(&self) -> VdMirBlockKind {
        match self {
            VdMirBlockMeta::Environment(..) => VdMirBlockKind::Environment,
            VdMirBlockMeta::Division(..) => VdMirBlockKind::Division,
        }
    }
}
