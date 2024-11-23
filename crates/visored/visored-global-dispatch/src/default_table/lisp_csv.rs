use super::*;
use ::lisp_csv::{file::LpCsvFile, parse_lp_csv_file, parse_lp_csv_filepath};
use std::path::Path;

impl VdDefaultGlobalDispatchTable {
    pub fn from_lisp_csv_file_dir(dir: &Path, db: &::salsa::Db) -> Self {
        todo!()
    }

    pub fn from_lisp_csv_file_paths(
        base_prefix_opr_file: &Path,
        base_binary_opr_file: &Path,
        base_separator_file: &Path,
        attach_file: &Path,
        base_sqrt_file: &Path,
        base_frac_file: &Path,
        db: &::salsa::Db,
    ) -> Self {
        let base_prefix_opr_file = parse_lp_csv_filepath(base_prefix_opr_file).unwrap();
        let base_binary_opr_file = parse_lp_csv_filepath(base_binary_opr_file).unwrap();
        let base_separator_file = parse_lp_csv_filepath(base_separator_file).unwrap();
        let attach_file = parse_lp_csv_filepath(attach_file).unwrap();
        let base_sqrt_file = parse_lp_csv_filepath(base_sqrt_file).unwrap();
        let base_frac_file = parse_lp_csv_filepath(base_frac_file).unwrap();
        Self::from_lisp_csv_files(
            &base_prefix_opr_file,
            &base_binary_opr_file,
            &base_separator_file,
            &attach_file,
            &base_sqrt_file,
            &base_frac_file,
            db,
        )
    }

    pub fn from_lisp_csv_files(
        base_prefix_opr_file: &LpCsvFile,
        base_binary_opr_file: &LpCsvFile,
        base_separator_file: &LpCsvFile,
        attach_file: &LpCsvFile,
        base_sqrt_file: &LpCsvFile,
        base_frac_file: &LpCsvFile,
        db: &::salsa::Db,
    ) -> Self {
        let base_prefix_opr_table = match base_prefix_opr_file {
            LpCsvFile::Rows(rows) => rows.iter().map(|_| todo!()),
        };
        let base_binary_opr_table = match base_binary_opr_file {
            LpCsvFile::Rows(rows) => rows.iter().map(|_| todo!()),
        };
        let base_separator_table = match base_separator_file {
            LpCsvFile::Rows(rows) => rows.iter().map(|_| todo!()),
        };
        let attach_table = match attach_file {
            LpCsvFile::Rows(rows) => rows.iter().map(|_| todo!()),
        };
        let base_sqrt_table = match base_sqrt_file {
            LpCsvFile::Rows(rows) => rows.iter().map(|_| todo!()),
        };
        let base_frac_table = match base_frac_file {
            LpCsvFile::Rows(rows) => rows.iter().map(|_| todo!()),
        };
        Self::new(
            base_prefix_opr_table,
            base_binary_opr_table,
            base_separator_table,
            attach_table,
            base_sqrt_table,
            base_frac_table,
        )
    }
}
