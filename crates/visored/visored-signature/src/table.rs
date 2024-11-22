use crate::signature::VdSignature;
use crate::*;
use lisp_csv::parse_lp_csv_file;
use lisp_csv::{expr::LpCsvExprData, file::LpCsvFile, row::LpCsvRow};
use rustc_hash::FxHashMap;
use salsa::DebugWithDb;

#[salsa::derive_debug_with_db]
#[derive(Debug)]
pub struct VdSignatureTable {
    table: FxHashMap<String, VdSignature>,
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
            table: {
                "complex_div": VdSignature::BinaryOpr(
                    VdBinaryOprSignature::Base(
                        VdBaseBinaryOprSignature {
                            instantiation: VdInstantiation {
                                path: TraitItem(
                                    FieldDiv,
                                ),
                                arguments: [
                                    VdTerm::ItemPath(
                                        VdItemPathTerm(
                                            VdTermId {
                                                data: ItemPath(
                                                    VdItemPathTermData {
                                                        item_path: Set(
                                                            Prelude(
                                                                ComplexNumber,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                ],
                            },
                            lopd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 8,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            ropd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 8,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 8,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                "int_pos": VdSignature::PrefixOpr(
                    Base(
                        VdBasePrefixOprSignature {
                            instantiation: VdInstantiation(
                                Id {
                                    value: 1,
                                },
                            ),
                            opd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                "rat_pos": VdSignature::PrefixOpr(
                    Base(
                        VdBasePrefixOprSignature {
                            instantiation: VdInstantiation(
                                Id {
                                    value: 2,
                                },
                            ),
                            opd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                "real_pos": VdSignature::PrefixOpr(
                    Base(
                        VdBasePrefixOprSignature {
                            instantiation: VdInstantiation(
                                Id {
                                    value: 3,
                                },
                            ),
                            opd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 7,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 7,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                "real_neg": VdSignature::PrefixOpr(
                    Base(
                        VdBasePrefixOprSignature {
                            instantiation: VdInstantiation(
                                Id {
                                    value: 7,
                                },
                            ),
                            opd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 7,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 7,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                "real_div": VdSignature::BinaryOpr(
                    VdBinaryOprSignature::Base(
                        VdBaseBinaryOprSignature {
                            instantiation: VdInstantiation {
                                path: TraitItem(
                                    FieldDiv,
                                ),
                                arguments: [
                                    VdTerm::ItemPath(
                                        VdItemPathTerm(
                                            VdTermId {
                                                data: ItemPath(
                                                    VdItemPathTermData {
                                                        item_path: Set(
                                                            Prelude(
                                                                RealNumber,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                ],
                            },
                            lopd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 7,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            ropd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 7,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 7,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                "rat_div": VdSignature::BinaryOpr(
                    VdBinaryOprSignature::Base(
                        VdBaseBinaryOprSignature {
                            instantiation: VdInstantiation {
                                path: TraitItem(
                                    FieldDiv,
                                ),
                                arguments: [
                                    VdTerm::ItemPath(
                                        VdItemPathTerm(
                                            VdTermId {
                                                data: ItemPath(
                                                    VdItemPathTermData {
                                                        item_path: Set(
                                                            Prelude(
                                                                RationalNumber,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                ],
                            },
                            lopd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            ropd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                "int_neg": VdSignature::PrefixOpr(
                    Base(
                        VdBasePrefixOprSignature {
                            instantiation: VdInstantiation(
                                Id {
                                    value: 5,
                                },
                            ),
                            opd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                "rat_sub": VdSignature::BinaryOpr(
                    VdBinaryOprSignature::Base(
                        VdBaseBinaryOprSignature {
                            instantiation: VdInstantiation {
                                path: TraitItem(
                                    RingSub,
                                ),
                                arguments: [
                                    VdTerm::ItemPath(
                                        VdItemPathTerm(
                                            VdTermId {
                                                data: ItemPath(
                                                    VdItemPathTermData {
                                                        item_path: Set(
                                                            Prelude(
                                                                RationalNumber,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                ],
                            },
                            lopd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            ropd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                "int_add": VdSignature::Separator(
                    Base(
                        VdBaseSeparatorSignature {
                            instantiation: VdInstantiation(
                                Id {
                                    value: 16,
                                },
                            ),
                            item_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                "complex_pos": VdSignature::PrefixOpr(
                    Base(
                        VdBasePrefixOprSignature {
                            instantiation: VdInstantiation(
                                Id {
                                    value: 4,
                                },
                            ),
                            opd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 8,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 8,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                "complex_neg": VdSignature::PrefixOpr(
                    Base(
                        VdBasePrefixOprSignature {
                            instantiation: VdInstantiation(
                                Id {
                                    value: 8,
                                },
                            ),
                            opd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 8,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 8,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                "int_sub": VdSignature::BinaryOpr(
                    VdBinaryOprSignature::Base(
                        VdBaseBinaryOprSignature {
                            instantiation: VdInstantiation {
                                path: TraitItem(
                                    RingSub,
                                ),
                                arguments: [
                                    VdTerm::ItemPath(
                                        VdItemPathTerm(
                                            VdTermId {
                                                data: ItemPath(
                                                    VdItemPathTermData {
                                                        item_path: Set(
                                                            Prelude(
                                                                Integer,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                ],
                            },
                            lopd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            ropd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                "real_sub": VdSignature::BinaryOpr(
                    VdBinaryOprSignature::Base(
                        VdBaseBinaryOprSignature {
                            instantiation: VdInstantiation {
                                path: TraitItem(
                                    RingSub,
                                ),
                                arguments: [
                                    VdTerm::ItemPath(
                                        VdItemPathTerm(
                                            VdTermId {
                                                data: ItemPath(
                                                    VdItemPathTermData {
                                                        item_path: Set(
                                                            Prelude(
                                                                RealNumber,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                ],
                            },
                            lopd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 7,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            ropd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 7,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 7,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                "int_mul": VdSignature::Separator(
                    Base(
                        VdBaseSeparatorSignature {
                            instantiation: VdInstantiation(
                                Id {
                                    value: 17,
                                },
                            ),
                            item_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                "rat_neg": VdSignature::PrefixOpr(
                    Base(
                        VdBasePrefixOprSignature {
                            instantiation: VdInstantiation(
                                Id {
                                    value: 6,
                                },
                            ),
                            opd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
                "complex_sub": VdSignature::BinaryOpr(
                    VdBinaryOprSignature::Base(
                        VdBaseBinaryOprSignature {
                            instantiation: VdInstantiation {
                                path: TraitItem(
                                    RingSub,
                                ),
                                arguments: [
                                    VdTerm::ItemPath(
                                        VdItemPathTerm(
                                            VdTermId {
                                                data: ItemPath(
                                                    VdItemPathTermData {
                                                        item_path: Set(
                                                            Prelude(
                                                                ComplexNumber,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                ],
                            },
                            lopd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 8,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            ropd_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 8,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_ty: VdType(
                                ItemPath(
                                    VdItemPathTerm(
                                        VdTermId(
                                            Id {
                                                value: 8,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            },
        }
    "#]]
    .assert_debug_eq(&table.debug(db));
}
