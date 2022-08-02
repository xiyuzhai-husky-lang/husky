use std::path::PathBuf;

use husky_debugger::*;
use husky_root_static_defn::{__Linkage, __StaticLinkageKey};
use libloading::{Library, Symbol};

#[tokio::main]
async fn main() {
    let flags = HuskyDebuggerFlags::from_env().expect("invalid arguments");
    let opt_library: Option<Library> = if let Some(cdylib) = flags.cdylib {
        Some(unsafe { Library::new(cdylib) }.expect("it should work"))
    } else {
        None
    };
    let opt_get_linkages_from_cdylib: Option<Symbol<GetLinkagesFromCDylib>> = opt_library
        .as_ref()
        .map(|library| unsafe { library.get(b"get_linkages") }.expect("what"));
    let mode: Mode = flags.mode.into();
    let package_dir: PathBuf = flags.package_dir.unwrap().into();
    mode.apply(
        &package_dir,
        opt_get_linkages_from_cdylib
            .as_ref()
            .map(|e| e as &GetLinkagesFromCDylib),
    )
    .await
}
