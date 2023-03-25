mod adversarial;
mod adversarial_generator;
mod adversarial_manager;
mod edit;

use self::adversarial::*;
use self::adversarial_generator::*;
use self::adversarial_manager::*;
use self::edit::*;
use super::*;
use serde::{Deserialize, Serialize};

use xrng::XRng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdversarialKind {
    Vfs,
    Token,
    Ast,
}

impl AdversarialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            AdversarialKind::Vfs => "vfs",
            AdversarialKind::Token => "token",
            AdversarialKind::Ast => "ast",
        }
    }
}

impl std::fmt::Display for AdversarialKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

pub(super) fn vfs_adversarial_test<Db, U, R>(
    db: &mut Db,
    task_name: &str,
    package_adversarials_dir: &Path,
    unit: U,
    f: &impl Fn(&Db, U) -> R,
) where
    Db: VfsDb + ?Sized,
    U: VfsTestUnit,
{
    let Some(  adversarial_path ) = unit.determine_adversarial_path(
        <Db as salsa::DbWithJar<VfsJar>>::as_jar_db(db),
        AdversarialKind::Vfs,
        task_name,
        package_adversarials_dir,
    ) else {
        return;
    };
    let module = unit.vfs_test_unit_downcast_as_module_path().unwrap();
    let manager = VfsAdversarialManager::new(module, adversarial_path);
    manager.run(db, &|db| {
        f(db, unit);
    })
}
