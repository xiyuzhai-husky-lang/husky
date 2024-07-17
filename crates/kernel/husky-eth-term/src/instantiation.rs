use self::fmt::EthTermFmtContext;
use crate::fmt::with_eth_term_fmt_context;
use crate::{term::symbolic_variable::EthSymbolicVariable, *};
use context::EthTermContextItd;
use husky_entity_path::{
    path::{ItemPath, ItemPathId},
    region::RegionPath,
};
use husky_syn_decl::decl::HasSynDecl;
use maybe_result::*;
use salsa::fmt::WithFmtContext;
use term::trai_for_ty_item::EthTypeAsTraitItem;
use vec_like::{SmallVecPairMap, VecMap};

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EthInstantiation {
    path: ItemPath,
    context_itd: EthTermContextItd,
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
        let ctx = item_fmt_context(db, *self.path);
        with_eth_term_fmt_context(ctx, f, db)
    }
}

// todo: move to a crate::fmt
#[salsa::tracked]
pub fn item_fmt_context(db: &::salsa::Db, path_id: ItemPathId) -> EthTermFmtContext {
    use husky_dec_signature::engine::syn_expr_dec_term_region;

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

impl EthInstantiation {
    pub fn context_itd(&self) -> EthTermContextItd {
        self.context_itd
    }

    pub fn task_ty(&self, db: &::salsa::Db) -> Option<EthTerm> {
        self.context_itd.task_ty(db)
    }

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

    fn instantiate(
        self,
        instantiation: &EthInstantiation,
        ctx: impl IsEthTermContextRef,
        db: &::salsa::Db,
    ) -> Self::Output;
}

pub trait IsEthTermContextRef<'db>: Copy {
    fn reduce_ty_as_trai_item(&self, term: EthTypeAsTraitItem) -> EthTerm;
    /// ideally speaking we should returns Ok(None) if there is no dependency on the task type,
    /// but at this stage, it's impossible to reliably tell whether there is a dependency on the task type
    ///
    /// It will be deferred to the hir stage to remove unnecessary task type dependency
    fn task_ty(&self) -> Option<EthTerm>;
    fn context_itd(&self) -> EthTermContextItd;
}

impl<T> EthInstantiate for Option<T>
where
    T: EthInstantiate,
{
    type Output = Option<T::Output>;

    fn instantiate(
        self,
        instantiation: &EthInstantiation,
        ctx: impl IsEthTermContextRef,
        db: &::salsa::Db,
    ) -> Self::Output {
        self.map(|slf| slf.instantiate(instantiation, ctx, db))
    }
}

impl<T> EthInstantiate for &[T]
where
    T: EthInstantiate,
{
    type Output = Vec<T::Output>;

    fn instantiate(
        self,
        instantiation: &EthInstantiation,
        ctx: impl IsEthTermContextRef,
        db: &::salsa::Db,
    ) -> Self::Output {
        self.iter()
            .copied()
            .map(|elem| elem.instantiate(instantiation, ctx, db))
            .collect()
    }
}

pub trait EthTermInstantiateRef {
    type Target;

    fn instantiate(&self, db: &::salsa::Db, instantiation: &EthInstantiation) -> Self::Target;
}

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EthInstantiationBuilder {
    path: ItemPath,
    context_itd: EthTermContextItd,
    symbol_map: SmallVecPairMap<EthSymbolicVariable, Option<EthTerm>, 4>,
    /// indicates the separation for associated item template instantiation
    separator: Option<u8>,
}

pub struct GenericPackageEthSignatureData;

impl EthInstantiationBuilder {
    /// symbols must be unique
    pub(crate) fn new<'db>(
        path: ItemPath,
        symbols: impl Iterator<Item = EthSymbolicVariable>,
        is_associated: bool,
        ctx: impl IsEthTermContextRef,
    ) -> Self {
        let symbol_map: SmallVecPairMap<EthSymbolicVariable, Option<EthTerm>, 4> =
            symbols.map(|symbol| (symbol, None)).collect();
        Self {
            path,
            context_itd: ctx.context_itd(),
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
            EthTerm::SymbolicVariable(_) | EthTerm::LambdaVariable(_) => JustOk(()),
            _ => Nothing,
        };
        match src {
            EthTerm::SymbolicVariable(symbol) => self.try_add_symbol_rule(symbol, dst),
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
            | EthTerm::ItemPath(_)
            | EthTerm::Sort(_)
            | EthTerm::Universe(_)
            | EthTerm::LambdaVariable(_) => Nothing,
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
            context_itd: self.context_itd,
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
            context_itd: self.context_itd,
            symbol_map,
            separator: Some(len),
        }
    }
}
