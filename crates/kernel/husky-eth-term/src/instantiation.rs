use crate::{term::symbol::EthSymbol, *};
use maybe_result::*;
use vec_like::SmallVecPairMap;

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EtherealInstantiation {
    symbol_map: SmallVecPairMap<EthSymbol, EthTerm, 4>,
    /// indicates the separation for associated item template instantiation
    separator: Option<u8>,
}

impl EtherealInstantiation {
    pub fn symbol_map(&self) -> &[(EthSymbol, EthTerm)] {
        self.symbol_map.as_ref()
    }

    pub fn separator(&self) -> Option<u8> {
        self.separator
    }

    /// assume that symbol is in symbol_map
    /// panic otherwise
    pub fn symbol_instantiation(&self, symbol: EthSymbol) -> EthTerm {
        *self
            .symbol_map
            .get_value(symbol)
            .expect("symbol should be in symbol_map")
    }

    pub fn symbol_map_splitted(
        &self,
    ) -> (&[(EthSymbol, EthTerm)], Option<&[(EthSymbol, EthTerm)]>) {
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

pub trait EthTermInstantiate: Copy {
    type Output;

    fn instantiate(self, db: &::salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output;
}

impl<T> EthTermInstantiate for Option<T>
where
    T: EthTermInstantiate,
{
    type Output = Option<T::Output>;

    fn instantiate(self, db: &salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output {
        self.map(|slf| slf.instantiate(db, instantiation))
    }
}

impl<T> EthTermInstantiate for &[T]
where
    T: EthTermInstantiate,
{
    type Output = Vec<T::Output>;

    fn instantiate(self, db: &salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output {
        self.iter()
            .copied()
            .map(|elem| elem.instantiate(db, instantiation))
            .collect()
    }
}

pub trait EthTermInstantiateRef {
    type Target;

    fn instantiate(&self, db: &::salsa::Db, instantiation: &EtherealInstantiation) -> Self::Target;
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EtherealInstantiationBuilder {
    symbol_map: SmallVecPairMap<EthSymbol, Option<EthTerm>, 4>,
    /// indicates the separation for associated item template instantiation
    separator: Option<u8>,
}

impl EtherealInstantiationBuilder {
    /// symbols must be unique
    pub(crate) fn new(symbols: impl Iterator<Item = EthSymbol>, is_associated: bool) -> Self {
        let symbol_map: SmallVecPairMap<EthSymbol, Option<EthTerm>, 4> =
            symbols.map(|symbol| (symbol, None)).collect();
        Self {
            separator: is_associated.then_some(symbol_map.len().try_into().unwrap()),
            symbol_map,
        }
    }

    /// `JustOk(())` means rule is added and everything is compatible.
    ///
    /// `Nothing` means something is incompatible.
    ///
    /// `JustErr(e)` means something is wrong.
    pub fn try_add_rules_from_application(
        &mut self,
        src: EthTerm,
        dst_arguments: &[EthTerm],
        db: &::salsa::Db,
    ) -> EthTermMaybeResult<()> {
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

    /// `JustOk(())` means rule is added and everything is compatible.
    ///
    /// `Nothing` means something is incompatible.
    ///
    /// `JustErr(e)` means something is wrong.
    pub fn try_add_rule(
        &mut self,
        src: EthTerm,
        dst: EthTerm,
        db: &::salsa::Db,
    ) -> EthTermMaybeResult<()> {
        if src == dst {
            return JustOk(());
        }
        match src {
            EthTerm::Symbol(symbol) => self.try_add_symbol_rule(symbol, dst),
            EthTerm::Application(_) => {
                let src_application_expansion = src.application_expansion(db);
                let dst_application_expansion = dst.application_expansion(db);
                if src_application_expansion.function() != dst_application_expansion.function() {
                    use husky_print_utils::p;
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
            EthTerm::Literal(_)
            | EthTerm::Rune(_)
            | EthTerm::EntityPath(_)
            | EthTerm::Category(_)
            | EthTerm::Universe(_) => Nothing,
            EthTerm::Curry(_) => todo!(),
            EthTerm::Ritchie(_) => todo!(),
            EthTerm::Abstraction(_) => todo!(),
            EthTerm::TypeAsTraitItem(_) => todo!(),
            EthTerm::TraitConstraint(_) => todo!(),
        }
    }

    pub fn try_add_symbol_rule(
        &mut self,
        symbol: EthSymbol,
        dst: EthTerm,
    ) -> EthTermMaybeResult<()> {
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
        let mut symbol_map = SmallVecPairMap::<EthSymbol, EthTerm, 4>::default();
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
        template_parameters: &EthTemplateParameters,
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
