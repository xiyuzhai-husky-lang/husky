use self::fmt::EthTermFmtContext;
use crate::fmt::with_eth_term_fmt_context;
use crate::{term::svar::EthSymbolicVariable, *};
use husky_entity_path::region::RegionPath;
use husky_syn_decl::decl::HasSynDecl;
use maybe_result::*;
use salsa::fmt::WithFmtContext;
use vec_like::{SmallVecPairMap, VecMap};

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EthInstantiation {
    path: ItemPath,
    symbol_map: SmallVecPairMap<EthSymbolicVariable, EthTerm, 4>,
    /// indicates the separation for associated item template instantiation
    separator: Option<u8>,
}

impl WithFmtContext for EthInstantiation {
    fn with_fmt_context(
        &self,
        f: impl FnOnce() -> std::fmt::Result,
        db: &salsa::Db,
    ) -> std::fmt::Result {
        let ctx = instantiation_eth_term_fmt_context(db, *self.path);
        with_eth_term_fmt_context(ctx, f, db)
    }
}

#[salsa::tracked(jar = EthTermJar)]
pub fn instantiation_eth_term_fmt_context(
    db: &::salsa::Db,
    path_id: ItemPathId,
) -> EthTermFmtContext {
    use husky_dec_signature::engine::syn_expr_dec_term_region;

    let path = path_id.item_path(db);
    let symbol_name_map =
        syn_expr_dec_term_region(db, path.syn_decl(db).unwrap().syn_expr_region(db).unwrap())
            .dec_symbol_region()
            .symbol_name_map();
    let symbol_names = VecMap::from_iter_assuming_no_repetitions(
        symbol_name_map
            .data()
            .iter()
            .map(|&(symbol, name)| (EthSymbolicVariable::from_dec(db, symbol).expect("ok"), name)),
    )
    .expect("no repetitions");
    EthTermFmtContext::new(db, RegionPath::Decl(path), symbol_names)
}

impl EthInstantiation {
    pub fn symbol_map(&self) -> &[(EthSymbolicVariable, EthTerm)] {
        self.symbol_map.as_ref()
    }

    pub fn separator(&self) -> Option<u8> {
        self.separator
    }

    /// assume that symbol is in symbol_map
    /// panic otherwise
    pub fn symbol_instantiation(&self, symbol: EthSymbolicVariable) -> EthTerm {
        *self
            .symbol_map
            .get_value(symbol)
            .expect("symbol should be in symbol_map")
    }

    pub fn symbol_map_splitted(
        &self,
    ) -> (
        &[(EthSymbolicVariable, EthTerm)],
        Option<&[(EthSymbolicVariable, EthTerm)]>,
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

    pub fn path(&self) -> ItemPath {
        self.path
    }
}

pub trait EthInstantiate: Copy {
    type Output;

    fn instantiate(self, db: &::salsa::Db, instantiation: &EthInstantiation) -> Self::Output;
}

impl<T> EthInstantiate for Option<T>
where
    T: EthInstantiate,
{
    type Output = Option<T::Output>;

    fn instantiate(self, db: &salsa::Db, instantiation: &EthInstantiation) -> Self::Output {
        self.map(|slf| slf.instantiate(db, instantiation))
    }
}

impl<T> EthInstantiate for &[T]
where
    T: EthInstantiate,
{
    type Output = Vec<T::Output>;

    fn instantiate(self, db: &salsa::Db, instantiation: &EthInstantiation) -> Self::Output {
        self.iter()
            .copied()
            .map(|elem| elem.instantiate(db, instantiation))
            .collect()
    }
}

pub trait EthTermInstantiateRef {
    type Target;

    fn instantiate(&self, db: &::salsa::Db, instantiation: &EthInstantiation) -> Self::Target;
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EtherealInstantiationBuilder {
    path: ItemPath,
    symbol_map: SmallVecPairMap<EthSymbolicVariable, Option<EthTerm>, 4>,
    /// indicates the separation for associated item template instantiation
    separator: Option<u8>,
}

impl EtherealInstantiationBuilder {
    /// symbols must be unique
    pub(crate) fn new(
        path: ItemPath,
        symbols: impl Iterator<Item = EthSymbolicVariable>,
        is_associated: bool,
    ) -> Self {
        let symbol_map: SmallVecPairMap<EthSymbolicVariable, Option<EthTerm>, 4> =
            symbols.map(|symbol| (symbol, None)).collect();
        Self {
            path,
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
        let is_symbol_or_hvar: EthTermMaybeResult<()> = match dst {
            EthTerm::Symbol(_) | EthTerm::Hvar(_) => JustOk(()),
            _ => Nothing,
        };
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
            | EthTerm::EntityPath(_)
            | EthTerm::Category(_)
            | EthTerm::Universe(_)
            | EthTerm::Hvar(_) => Nothing,
            EthTerm::Curry(_) => {
                todo!("dependent type")
                // let EthTerm::TraitConstraint(dst) = dst else {
                //     // todo: check dst is of type Prop
                //     return is_symbol_or_hvar;
                // };
                // self.try_add_rule(src.ty(db), dst.ty(db), db)?;
                // self.try_add_rule(src.trai(db), dst.trai(db), db)
            }
            EthTerm::Ritchie(_) => todo!(),
            EthTerm::Abstraction(_) | EthTerm::TypeAsTraitItem(_) => JustOk(()),
            EthTerm::TraitConstraint(src) => {
                let EthTerm::TraitConstraint(dst) = dst else {
                    // todo: check dst is of type Prop
                    return is_symbol_or_hvar;
                };
                self.try_add_rule(src.ty(db), dst.ty(db), db)?;
                self.try_add_rule(src.trai(db), dst.trai(db), db)
            }
        }
    }

    pub fn try_add_symbol_rule(
        &mut self,
        symbol: EthSymbolicVariable,
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

    pub fn try_into_instantiation(&self) -> Option<EthInstantiation> {
        let mut symbol_map = SmallVecPairMap::<EthSymbolicVariable, EthTerm, 4>::default();
        for (symbol, mapped) in self.symbol_map.iter() {
            let mapped = (*mapped)?;
            unsafe { symbol_map.insert_new_unchecked((*symbol, mapped)) }
        }
        Some(EthInstantiation {
            path: self.path,
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
            path: self.path,
            symbol_map,
            separator: Some(len),
        }
    }
}
