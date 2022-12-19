use crate::*;
use husky_toolchain::ToolchainDb;
use husky_word::WordDb;
use salsa::DbWithJar;

pub trait PackagePathDb: DbWithJar<PackagePathJar> + ToolchainDb + WordDb {
    fn builtin_package_path(&self, toolchain: Toolchain, ident: Identifier) -> Option<PackagePath>;
    fn package_path_menu(&self, toolchain: Toolchain) -> &PackagePathMenu;
    fn package_path_data(&self, package: PackagePath) -> &PackagePathData;
    // fn package_ident(&self, package: PackagePath) -> Identifier;
    fn package_name(&self, package: PackagePath) -> &str;
    fn it_package_path(&self, data: PackagePathData) -> PackagePath;
}

impl<T> PackagePathDb for T
where
    T: DbWithJar<PackagePathJar> + ToolchainDb + WordDb,
{
    fn builtin_package_path(&self, toolchain: Toolchain, ident: Identifier) -> Option<PackagePath> {
        let word_menu = self.word_menu();
        let package_path_menu = self.package_path_menu(toolchain);
        if ident == word_menu.core() {
            Some(package_path_menu.core())
        } else if ident == word_menu.std() {
            Some(package_path_menu.std())
        } else {
            None
        }
    }

    fn package_path_menu(&self, toolchain: Toolchain) -> &PackagePathMenu {
        package_path_menu(self, toolchain)
    }

    fn package_path_data(&self, package: PackagePath) -> &PackagePathData {
        package.data(self)
    }

    fn package_name(&self, package: PackagePath) -> &str {
        package_name(self, package)
    }

    fn it_package_path(&self, data: PackagePathData) -> PackagePath {
        PackagePath::new(self, data)
    }
}
// pub(crate) fn builtin_package_path(
//     db: &dyn PackagePathDb,
//     toolchain: Toolchain,
//     ident: Identifier,
// ) -> Option<PackagePath> {
//     let word_menu = db.word_menu();
//     let word = ident.word();
//     if word == word_menu.core() || word == word_menu.std() {
//         Some(PackagePath::new(
//             db,
//             ident,
//             PackagePathData::Builtin { toolchain },
//         ))
//     } else {
//         None
//     }
// }
