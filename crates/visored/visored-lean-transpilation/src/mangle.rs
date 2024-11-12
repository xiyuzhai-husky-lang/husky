use crate::*;
use lean_coword::ident::LnIdent;
use rustc_hash::FxHashMap;
use visored_hir_expr::symbol::local_defn::{
    storage::VdHirSymbolLocalDefnStorage, VdHirSymbolLocalDefnHead, VdHirSymbolLocalDefnIdx,
    VdHirSymbolLocalDefnOrderedMap,
};

pub struct VdLeanTranspilationMangler {
    local_defn_mangled_symbols: VdHirSymbolLocalDefnOrderedMap<LnIdent>,
}

impl VdLeanTranspilationMangler {
    pub(crate) fn new(storage: &VdHirSymbolLocalDefnStorage, db: &::salsa::Db) -> Self {
        let mut local_defn_mangled_symbols: VdHirSymbolLocalDefnOrderedMap<LnIdent> =
            Default::default();
        let mut disambiguator_map: FxHashMap<String, usize> = FxHashMap::default();
        for (idx, defn) in storage.defn_arena().indexed_iter() {
            let naive_ident = naive_ident(defn.head());
            let mangled_ident = mangle_naive_ident(naive_ident, &mut disambiguator_map, db);
            local_defn_mangled_symbols.insert_next(idx, mangled_ident);
        }
        Self {
            local_defn_mangled_symbols,
        }
    }

    pub(crate) fn mangled_symbol(&self, symbol_local_defn: VdHirSymbolLocalDefnIdx) -> LnIdent {
        self.local_defn_mangled_symbols[symbol_local_defn]
    }
}

fn naive_ident(head: &VdHirSymbolLocalDefnHead) -> String {
    match *head {
        VdHirSymbolLocalDefnHead::Letter(letter) => letter.to_string(),
    }
}

fn mangle_naive_ident(
    naive_ident: String,
    disambiguator_map: &mut FxHashMap<String, usize>,
    db: &::salsa::Db,
) -> LnIdent {
    // If the identifier hasn't been used before, use it as-is
    let mangled = if !disambiguator_map.contains_key(&naive_ident) {
        disambiguator_map.insert(naive_ident.clone(), 0);
        naive_ident
    } else {
        // Get the next number in sequence and increment it
        let next_num = disambiguator_map.get(&naive_ident).unwrap() + 1;
        disambiguator_map.insert(naive_ident.clone(), next_num);
        format!("{}{}", naive_ident, next_num)
    };

    LnIdent::from_owned(mangled, db)
}

#[test]
fn test_mangle_naive_ident() {
    use expect_test::expect;
    let mut disambiguator_map = FxHashMap::default();
    let db = &DB::default(); // Assuming you have a test database setup

    // First occurrence should be unchanged
    let result1 = mangle_naive_ident("x".to_string(), &mut disambiguator_map, db);
    expect!["x"].assert_eq(&result1.data(db));

    // Second occurrence should be x1
    let result2 = mangle_naive_ident("x".to_string(), &mut disambiguator_map, db);
    expect!["x1"].assert_eq(&result2.data(db));

    // Third occurrence should be x2
    let result3 = mangle_naive_ident("x".to_string(), &mut disambiguator_map, db);
    expect!["x2"].assert_eq(&result3.data(db));

    // Different letter should start fresh
    let result4 = mangle_naive_ident("y".to_string(), &mut disambiguator_map, db);
    expect!["y"].assert_eq(&result4.data(db));

    // Second occurrence of y should be y1
    let result5 = mangle_naive_ident("y".to_string(), &mut disambiguator_map, db);
    expect!["y1"].assert_eq(&result5.data(db));
}
