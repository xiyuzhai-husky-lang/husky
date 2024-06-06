mod adversarial;
mod adversarial_generator;
mod adversarial_manager;
mod edit;

use self::adversarial::*;
use self::adversarial_generator::*;
use self::adversarial_manager::*;
use self::edit::*;
use super::*;
use husky_rng_utils::XRng;
use salsa::DebugWithDb;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdversarialKind {
    Vfs,
    TokenData,
    Ast,
}

impl AdversarialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            AdversarialKind::Vfs => "vfs",
            AdversarialKind::TokenData => "token",
            AdversarialKind::Ast => "ast",
        }
    }
}

impl std::fmt::Display for AdversarialKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

pub(super) fn vfs_adversarial_test<U, R>(
    db: &mut Db,
    package_adversarials_dir: &Path,
    unit: U,
    f: &impl Fn(&::salsa::Db, U) -> R,
    config: &VfsTestConfig,
    paths_used: &mut HashMap<PathBuf, PathUsage<U>>,
) where
    U: IsVfsTestUnit + ::salsa::DebugWithDb,
{
    let Some(adversarial_path) =
        unit.determine_adversarial_path(db, AdversarialKind::Vfs, package_adversarials_dir, config)
    else {
        return;
    };
    if let Some(old_usage) =
        paths_used.insert(adversarial_path.clone(), PathUsage::Adversarial(unit))
    {
        panic!(
            r#"Detect conflicting path for unit `{:?}` while doing adversarial testing!
Old usage is `{:?}`.
The conflicting path is `{adversarial_path:?}`"#,
            unit.debug(db),
            old_usage.debug(db),
        )
    }
    let module = unit.vfs_test_unit_downcast_as_module_path().unwrap();
    let manager = VfsAdversarialManager::new(module, adversarial_path);
    manager.run(db, &|db| {
        f(db, unit);
    })
}
