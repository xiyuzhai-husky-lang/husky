use crate::*;
use husky_word::Identifier;
use vec_like::AsVecMapEntry;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct LocalSymbolSheet {
    variables: Vec<VariableSymbol>,
}

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VariableSymbol {
    ident: Identifier,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VariableIdx(usize);

pub enum VariableData {
    Normal { ident: Identifier },
    FrameVariable { ident: Identifier },
    ThisValue { ident: Identifier },
    ThisMethod { ident: Identifier },
    ThisField { ident: Identifier },
}

impl LocalSymbolSheet {
    pub fn define_variable(&mut self, _symbol: Symbol) {
        todo!()
        // self.symbols.push(symbol)
    }

    pub fn resolve_ident(&self, ident: Identifier) -> Option<Symbol> {
        let mut symbol: Option<(usize, &VariableSymbol)> = None;
        for variable in self.variables.iter() {
            todo!()
        }
        // let symbols: Vec<_> = self
        //     .symbols
        //     .iter()
        //     .filter(|entry| entry.symbol.ident == ident)
        //     .collect();
        // if symbols.len() == 0 {
        //     return None;
        // }
        if let Some((i, _)) = symbol {
            Some(Symbol::Variable(VariableIdx(i)))
        } else {
            None
        }
    }

    fn try1() {
        let _haha = Haha::default();

        #[derive(Default)]
        struct Haha;
    }
}
