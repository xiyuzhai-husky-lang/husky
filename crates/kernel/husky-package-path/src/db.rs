use crate::*;
use husky_toolchain::ToolchainDb;
use husky_word::WordDb;
use salsa::DbWithJar;

pub trait PackagePathDb: DbWithJar<PackagePathJar> + ToolchainDb + WordDb {
    fn builtin_package_path(&self, ident: Identifier) -> Option<PackagePath>;
    fn package_path_menu(&self) -> &PackagePathMenu;
    fn package_path_data(&self, package: PackagePath) -> &PackagePathData;
    // fn package_ident(&self, package: PackagePath) -> Identifier;
    fn package_name(&self, package: PackagePath) -> &str;
    fn it_package_path(&self, data: PackagePathData) -> PackagePath;
}

impl<T> PackagePathDb for T
where
    T: DbWithJar<PackagePathJar> + ToolchainDb + WordDb,
{
    fn builtin_package_path(&self, ident: Identifier) -> Option<PackagePath> {
        let word_menu = self.word_menu();
        if ident == word_menu.core() {
            Some(self.package_path_menu().core())
        } else if ident == word_menu.std() {
            Some(self.package_path_menu().std())
        } else {
            None
        }
    }

    fn package_path_menu(&self) -> &PackagePathMenu {
        package_path_menu(self, self.toolchain())
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
