use crate::*;
use maybe_result::*;
use vec_like::SmallVecPairMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct EtherealInstantiation {
    symbol_map: SmallVecPairMap<EtherealTermSymbol, EtherealTerm, 4>,
    /// indicates the separation for associated item template instantiation
    separator: Option<u8>,
}

impl EtherealInstantiation {
    /// assume that symbol is in symbol_map
    /// panic otherwise
    pub fn symbol_mapped(&self, symbol: EtherealTermSymbol) -> EtherealTerm {
        *self
            .symbol_map
            .get_value(symbol)
            .expect("symbol should be in symbol_map")
    }

    pub fn symbol_map(&self) -> &[(EtherealTermSymbol, EtherealTerm)] {
        self.symbol_map.as_ref()
    }

    pub fn separator(&self) -> Option<u8> {
        self.separator
    }

    pub fn symbol_map_splitted(
        &self,
    ) -> (
        &[(EtherealTermSymbol, EtherealTerm)],
        Option<&[(EtherealTermSymbol, EtherealTerm)]>,
    ) {
        let symbol_map: &[_] = self.symbol_map.as_ref();
        match self.separator {
            Some(separator) => {
                let separator = separator as usize;
                (&symbol_map[0..separator], Some(&symbol_map[separator..]))
            }
            None => (symbol_map, None),
        }
    }
}

pub trait EtherealInstantiate: Copy {
    type Output;

    fn instantiate(self, db: &::salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output;
}

pub trait EtherealTermInstantiateRef {
    type Target;

    fn instantiate(&self, db: &::salsa::Db, instantiation: &EtherealInstantiation) -> Self::Target;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct EtherealInstantiationBuilder {
    symbol_map: SmallVecPairMap<EtherealTermSymbol, Option<EtherealTerm>, 4>,
    /// indicates the separation for associated item template instantiation
    separator: Option<u8>,
}

impl EtherealInstantiationBuilder {
    /// symbols must be unique
    pub(crate) fn new(symbols: impl Iterator<Item = EtherealTermSymbol>) -> Self {
        Self {
            symbol_map: symbols.map(|symbol| (symbol, None)).collect(),
            separator: None,
        }
    }

    /// JustOk(()) means rule is added and everything is compatible
    /// Nothing means something is incompatible
    /// JustErr(_) means something is wrong
    pub fn try_add_rules_from_application(
        &mut self,
        src: EtherealTerm,
        dst_arguments: &[EtherealTerm],
        db: &::salsa::Db,
    ) -> EtherealTermMaybeResult<()> {
        let src_application_expansion = src.application_expansion(db);
        if src_application_expansion.arguments(db).len() != dst_arguments.len() {
            todo!()
        }
        std::iter::zip(
            src_application_expansion.arguments(db).iter().copied(),
            dst_arguments.iter().copied(),
        )
        .try_for_each(|(src, dst)| self.try_add_rule(src, dst.into(), db))?;
        JustOk(())
    }

    /// JustOk(()) means rule is added and everything is compatible
    /// Nothing means something is incompatible
    /// JustErr(_) means something is wrong
    pub fn try_add_rule(
        &mut self,
        src: EtherealTerm,
        dst: EtherealTerm,
        db: &::salsa::Db,
    ) -> EtherealTermMaybeResult<()> {
        if src == dst {
            return JustOk(());
        }
        match src {
            EtherealTerm::Literal(_) => todo!(),
            EtherealTerm::Symbol(symbol) => self.try_add_symbol_rule(symbol, dst),
            EtherealTerm::Variable(_) => todo!(),
            EtherealTerm::EntityPath(_) => todo!(),
            EtherealTerm::Category(_) => todo!(),
            EtherealTerm::Universe(_) => todo!(),
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(_) => {
                let src_application_expansion = src.application_expansion(db);
                let dst_application_expansion = dst.application_expansion(db);
                if src_application_expansion.function() != dst_application_expansion.function() {
                    p!(
                        src_application_expansion.function().debug(db),
                        dst_application_expansion.function().debug(db)
                    );
                    todo!()
                }
                let src_application_arguments = src_application_expansion.arguments(db);
                let dst_application_arguments = dst_application_expansion.arguments(db);
                if src_application_arguments.len() != dst_application_arguments.len() {
                    todo!()
                }
                for (&src_arg, &dst_arg) in
                    std::iter::zip(src_application_arguments, dst_application_arguments)
                {
                    self.try_add_rule(src_arg, dst_arg, db)?
                }
                JustOk(())
            }
            EtherealTerm::Subitem(_) => todo!(),
            EtherealTerm::AsTraitSubitem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }

    pub fn try_add_symbol_rule(
        &mut self,
        symbol: EtherealTermSymbol,
        dst: EtherealTerm,
    ) -> EtherealTermMaybeResult<()> {
        if let Some((_, opt_dst0)) = self.symbol_map.get_entry_mut(symbol) {
            match opt_dst0 {
                Some(dst0) => {
                    if dst != *dst0 {
                        todo!()
                    } else {
                        return JustOk(());
                    }
                }
                None => {
                    *opt_dst0 = Some(dst);
                    JustOk(())
                }
            }
        } else {
            Nothing
        }
    }

    pub fn try_into_instantiation(&self) -> Option<EtherealInstantiation> {
        let mut symbol_map = SmallVecPairMap::<EtherealTermSymbol, EtherealTerm, 4>::default();
        for (symbol, mapped) in self.symbol_map.iter() {
            let mapped = (*mapped)?;
            unsafe { symbol_map.insert_new_unchecked((*symbol, mapped)) }
        }
        Some(EtherealInstantiation {
            symbol_map,
            separator: self.separator,
        })
    }

    pub fn merge_with_item_template_parameters(
        &self,
        template_parameters: &EtherealTermTemplateParameters,
    ) -> Self {
        let mut symbol_map = self.symbol_map.clone();
        let len = symbol_map.len().try_into().unwrap();
        for param in template_parameters.iter() {
            unsafe { symbol_map.insert_new_unchecked((param.symbol(), None)) }
        }
        Self {
            symbol_map,
            separator: Some(len),
        }
    }
}
