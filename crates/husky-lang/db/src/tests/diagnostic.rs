use crate::*;

use common::test_utils::assert_test_env;

use diagnostic::DiagnosticQuery;
use file::LiveFiles;

#[cfg(test)]
#[test]
fn test_diagnostics() {
    let mut db = HuskyLangDatabase::new();
    db.set_live_file_text("play/main.hsk".into(), "struct A {}\n".into());
    db.set_live_file_text("play/main.hsk".into(), "s truct A {}\n".into());
    let modules = db.all_modules();
    assert_eq!(modules.len(), 1);
    let the_module = modules[0];
    // {
    //     let diagnostic_reserve = db.diagnostic_reserve(the_module);
    //     ep!(diagnostic_reserve);
    //     assert_eq!(diagnostic_reserve.diagnostics_ref().len(), 0);
    // }
    {
        let diagnostic_reserve = db.diagnostic_reserve(the_module);
        p!(diagnostic_reserve);
        assert_eq!(diagnostic_reserve.diagnostics_ref().len(), 1);
    }
}
