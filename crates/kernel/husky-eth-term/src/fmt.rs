use self::term::symbolic_variable::EthSymbolicVariable;
use crate::{term::symbolic_variable::EthTermSymbolIndexImpl, *};
use husky_entity_path::{
    path::{ItemPath, ItemPathId},
    region::RegionPath,
};
use husky_term_prelude::symbol::SymbolName;
use vec_like::{VecMap, VecPairMap};

#[salsa::tracked]
pub struct EthTermFmtContext {
    #[id]
    path: RegionPath,
    #[return_ref]
    symbol_names: VecPairMap<EthSymbolicVariable, SymbolName>,
}

thread_local! {
    static ETH_TERM_FMT_CONTEXT_CELL: std::cell::Cell<Option<EthTermFmtContext>> = Default::default();
}

pub(crate) fn symbol_name(symbol: EthSymbolicVariable, db: &::salsa::Db) -> SymbolName {
    let ctx = ETH_TERM_FMT_CONTEXT_CELL.get().unwrap();
    if !ctx.symbol_names(db).contains(symbol) {
        return match symbol.index(db).inner() {
            EthTermSymbolIndexImpl::SelfType => SymbolName::SelfType,
            EthTermSymbolIndexImpl::SelfValue => SymbolName::SelfValue,
            EthTermSymbolIndexImpl::SelfLifetime => SymbolName::SelfLifetime,
            EthTermSymbolIndexImpl::SelfPlace => SymbolName::SelfPlace,
            _ => {
                use husky_print_utils::p;
                p!(ctx.path(db).debug(db));
                p!(ctx
                    .symbol_names(db)
                    .iter()
                    .map(|&(symbol1, name)| (symbol1 == symbol, symbol1.index(db), name))
                    .collect::<Vec<_>>()
                    .debug(db));
                p!(symbol.index(db));
                panic!()
            }
        };
    }
    ctx.symbol_names(db)[symbol].1
}

pub fn with_item_eth_term_fmt_context<R>(
    item_path: ItemPath,
    f: impl FnOnce() -> R,
    db: &::salsa::Db,
) -> R {
    let ctx = item_fmt_context(db, *item_path);
    with_eth_term_fmt_context(ctx, f)
}

fn with_eth_term_fmt_context<R>(ctx: EthTermFmtContext, f: impl FnOnce() -> R) -> R {
    let old_ctx = ETH_TERM_FMT_CONTEXT_CELL.get();
    ETH_TERM_FMT_CONTEXT_CELL.set(Some(ctx));
    let result = f();
    ETH_TERM_FMT_CONTEXT_CELL.set(old_ctx);
    result
}

#[salsa::tracked]
fn item_fmt_context(db: &::salsa::Db, path_id: ItemPathId) -> EthTermFmtContext {
    use husky_dec_signature::engine::syn_expr_dec_term_region;
    use husky_syn_decl::decl::HasSynDecl;

    let path = path_id.item_path(db);
    let Some(syn_expr_region) = path.syn_decl(db).unwrap().syn_expr_region(db) else {
        return EthTermFmtContext::new(db, RegionPath::ItemDecl(path), Default::default());
    };
    let symbol_name_map = syn_expr_dec_term_region(db, syn_expr_region)
        .symbolic_variable_region()
        .symbol_name_map();
    let symbol_names = VecMap::from_iter_assuming_no_repetitions(
        symbol_name_map
            .data()
            .iter()
            .map(|&(symbol, name)| (EthSymbolicVariable::from_dec(db, symbol).expect("ok"), name)),
    )
    .expect("no repetitions");
    EthTermFmtContext::new(db, RegionPath::ItemDecl(path), symbol_names)
}
