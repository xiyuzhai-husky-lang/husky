use crate::signature::VdSignature;
use crate::*;
use lisp_csv::parse_lp_csv_file;
use lisp_csv::{expr::LpCsvExprData, file::LpCsvFile, row::LpCsvRow};
use menu::{vd_signature_menu, VdSignatureMenu};
use rustc_hash::FxHashMap;
use salsa::DebugWithDb;

#[salsa::derive_debug_with_db]
#[derive(Debug)]
pub struct VdSignatureTable {
    table: FxHashMap<String, VdSignature>,
}

impl std::ops::Deref for VdSignatureTable {
    type Target = FxHashMap<String, VdSignature>;

    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl VdSignatureTable {
    pub fn new(data: impl IntoIterator<Item = (String, VdSignature)>) -> Self {
        let mut table = FxHashMap::default();
        for (key, signature) in data {
            if table.contains_key(&key) {
                todo!()
            } else {
                table.insert(key, signature);
            }
        }
        Self { table }
    }

    pub fn from_lp_csv_file(file: &LpCsvFile, db: &::salsa::Db) -> Self {
        match file {
            LpCsvFile::Rows(rows) => Self::from_lp_csv_rows(rows, db),
        }
    }

    fn from_lp_csv_rows(rows: &[LpCsvRow], db: &::salsa::Db) -> Self {
        let mut table: FxHashMap<String, VdSignature> = FxHashMap::default();
        assert!(!rows.is_empty());
        for row in rows {
            let (ident, signature) = match row {
                LpCsvRow::Expr(expr) => todo!(),
                LpCsvRow::SeparatedExprs(exprs) => {
                    assert_eq!(exprs.len(), 3);
                    let ident = match exprs[0].data {
                        LpCsvExprData::Ident(ref ident) => ident.to_string(),
                        _ => todo!(),
                    };
                    let signature = VdSignature::from_lp_csv_exprs(&exprs[1..], db);
                    (ident, signature)
                }
            };
            if table.contains_key(&ident) {
                todo!()
            } else {
                table.insert(ident, signature);
            }
        }
        assert!(!table.is_empty());
        Self { table }
    }
}

#[test]
fn vd_signature_table_from_lp_csv_rows_works() {
    use husky_path_utils::HuskyLangDevPaths;

    let db = &DB::default();
    let dev_dirs = HuskyLangDevPaths::new();
    let file =
        std::fs::read_to_string(dev_dirs.specs_dir().join("visored/instances.lisp-csv")).unwrap();
    let file = parse_lp_csv_file(&file).unwrap();
    let table = VdSignatureTable::from_lp_csv_file(&file, db);
    expect![[r#"
        VdSignatureTable {
            table: {},
        }
    "#]]
    .assert_debug_eq(&table.debug(db));
    let VdSignatureMenu {
        int_pos,
        rat_pos,
        real_pos,
        complex_pos,
        int_neg,
        rat_neg,
        real_neg,
        complex_neg,
        int_sub,
        rat_sub,
        real_sub,
        complex_sub,
        rat_div,
        real_div,
        complex_div,
        nat_add,
        int_add,
        rat_add,
        real_add,
        complex_add,
        nat_mul,
        int_mul,
        rat_mul,
        real_mul,
        complex_mul,
        nat_to_the_power_of_nat,
        int_to_the_power_of_nat,
        rat_to_the_power_of_nat,
        real_to_the_power_of_nat,
        complex_to_the_power_of_nat,
        nat_eq,
        int_eq,
        rat_eq,
        real_eq,
        complex_eq,
        nat_ne,
        int_ne,
        rat_ne,
        real_ne,
        complex_ne,
        nat_lt,
        int_lt,
        rat_lt,
        real_lt,
        nat_gt,
        int_gt,
        rat_gt,
        real_gt,
        nat_le,
        int_le,
        rat_le,
        real_le,
        nat_ge,
        int_ge,
        rat_ge,
        real_ge,
        real_sqrt,
    } = *vd_signature_menu(db);
    for (key, ident) in [
        ("int_pos", int_pos),
        ("rat_pos", rat_pos),
        ("real_pos", real_pos),
        ("complex_pos", complex_pos),
        ("int_neg", int_neg),
    ] {
        assert_eq!(table["int_pos"], int_pos.into());
    }
}
