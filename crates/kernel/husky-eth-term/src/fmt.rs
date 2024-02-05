use self::term::symbol::EthSymbol;
use crate::{term::symbol::EthTermSymbolIndexImpl, *};
use husky_entity_path::region::RegionPath;
use husky_term_prelude::symbol::SymbolName;
use vec_like::VecPairMap;

#[salsa::tracked(jar = EthTermJar)]
pub struct EthTermFmtContext {
    #[id]
    path: RegionPath,
    #[return_ref]
    symbol_names: VecPairMap<EthSymbol, SymbolName>,
}

thread_local! {
    static ETH_TERM_FMT_CONTEXT_CELL: std::cell::Cell<Option<EthTermFmtContext>> = Default::default();
}

pub(crate) fn symbol_name(symbol: EthSymbol, db: &::salsa::Db) -> SymbolName {
    let ctx = ETH_TERM_FMT_CONTEXT_CELL.get().unwrap();
    use husky_print_utils::p;
    if !ctx.symbol_names(db).contains(symbol) {
        return match symbol.index(db).inner() {
            EthTermSymbolIndexImpl::SelfType => SymbolName::SelfType,
            EthTermSymbolIndexImpl::SelfValue => SymbolName::SelfValue,
            EthTermSymbolIndexImpl::SelfLifetime => SymbolName::SelfLifetime,
            EthTermSymbolIndexImpl::SelfPlace => SymbolName::SelfPlace,
            _ => {
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

pub fn with_eth_term_fmt_context(
    ctx: EthTermFmtContext,
    f: impl FnOnce() -> std::fmt::Result,
    db: &::salsa::Db,
) -> std::fmt::Result {
    let old_ctx = ETH_TERM_FMT_CONTEXT_CELL.get();
    ETH_TERM_FMT_CONTEXT_CELL.set(Some(ctx));
    let result = f();
    ETH_TERM_FMT_CONTEXT_CELL.set(old_ctx);
    result
}
