mod adversarial;
mod adversarial_generator;
mod adversarial_manager;
mod edit;

use self::adversarial::*;
use self::adversarial_generator::*;
use self::adversarial_manager::*;
use self::edit::*;
use std::panic::RefUnwindSafe;

use super::*;

pub(super) fn vfs_robustness_test<Db, U, R>(
    db: &mut Db,
    task_name: &str,
    package_adversarials_dir: &Path,
    unit: U,
    f: &(impl Fn(&Db, U) -> R),
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
    let module = unit.module().unwrap();
    let manager = VfsAdversarialManager::new(module, adversarial_path);
    manager.run(db, &|db| {
        f(db, unit);
    })
}
