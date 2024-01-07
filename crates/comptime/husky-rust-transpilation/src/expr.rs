pub(crate) mod pattern;
pub(crate) mod precedence;
pub(crate) mod site;
mod stmt;

pub(crate) use self::precedence::{RustPrecedence, RustPrecedenceRange};

use self::{precedence::hir_eager_expr_precedence, site::HirEagerExprSite};
use crate::{binding::RustBinding, *};
use husky_entity_kind::FugitiveKind;
use husky_entity_path::{MajorItemPath, PrincipalEntityPath};
use husky_hir_eager_expr::{
    HirEagerCondition, HirEagerElifBranch, HirEagerElseBranch, HirEagerExprData, HirEagerExprEntry,
    HirEagerExprIdx, HirEagerExprRegion, HirEagerIfBranch, HirEagerLetVariablesPattern,
    HirEagerPatternExpr, HirEagerPatternExprIdx, HirEagerRitchieParameterArgumentMatch,
    HirEagerStmtData, HirEagerStmtIdx, HirEagerStmtIdxRange,
};
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_hir_ty::{
    instantiation::HirTermSymbolResolution, place::HirPlace, ritchie::HirEagerContract,
    HirTemplateSymbol, HirTemplateSymbolClass,
};
use husky_opr::BinaryClosedOpr;
use smallvec::SmallVec;

impl TranspileToRustWith<HirEagerExprRegion> for (HirEagerExprIdx, HirEagerExprSite) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let (slf, mut site) = self;
        let db = builder.db();
        let entry = slf.entry(builder.hir_eager_expr_arena());
        let data = &entry.data;
        let precedence = hir_eager_expr_precedence(data);
        let needs_deref = site.hir_eager_expr_needs_deref(entry);
        if needs_deref {
            site.rust_bindings.push(RustBinding::Deref)
        }
        let mut wrap_in_some_flag = false;
        if let Some(rust_binding) = site.rust_bindings.last() {
            match rust_binding {
                RustBinding::Deref
                | RustBinding::DerefCustomed
                | RustBinding::Reref
                | RustBinding::RerefMut => {
                    site.rust_precedence_range = RustPrecedenceRange::Geq(RustPrecedence::Prefix)
                }
                RustBinding::SelfValue => (),
                RustBinding::WrapInSome => site.rust_precedence_range = RustPrecedenceRange::ANY,
            }
        }
        let needs_extra_pars = site
            .rust_bindings
            .first()
            .map(|rust_binding| match rust_binding {
                RustBinding::Deref
                | RustBinding::DerefCustomed
                | RustBinding::Reref
                | RustBinding::RerefMut => {
                    !site.rust_precedence_range.include(RustPrecedence::Prefix)
                }
                RustBinding::SelfValue | RustBinding::WrapInSome => false,
            })
            .unwrap_or(false);
        if needs_extra_pars {
            builder.lpar();
        }
        for &rust_binding in &*site.rust_bindings {
            match rust_binding {
                RustBinding::Deref | RustBinding::DerefCustomed => {
                    builder.punctuation(RustPunctuation::DerefStar)
                }
                RustBinding::Reref => builder.punctuation(RustPunctuation::Ambersand),
                RustBinding::RerefMut => {
                    builder.punctuation(RustPunctuation::Ambersand);
                    builder.keyword(RustKeyword::Mut)
                }
                RustBinding::SelfValue => (),
                RustBinding::WrapInSome => builder.wrap_in_some_left(&mut wrap_in_some_flag),
            }
        }
        if !site.rust_precedence_range.include(precedence) {
            builder.bracketed_list_with(RustBracket::Par, |builder| {
                site.transpile_hir_eager_expr_to_rust(data, precedence, builder)
            })
        } else {
            site.transpile_hir_eager_expr_to_rust(data, precedence, builder)
        }
        if wrap_in_some_flag {
            builder.wrap_in_some_right()
        }
        if needs_extra_pars {
            builder.rpar();
        }
    }
}

impl HirEagerExprSite {
    fn hir_eager_expr_needs_deref(&self, entry: &HirEagerExprEntry) -> bool {
        match entry.data {
            HirEagerExprData::Variable(_) => match entry.ty_place {
                HirPlace::Const | HirPlace::StackPure { .. } => !entry.is_ty_always_copyable,
                HirPlace::Ref { .. } => true,
                HirPlace::RefMut { .. } => true,
                _ => false,
            },
            HirEagerExprData::ConstSymbol { .. }
            | HirEagerExprData::FunctionFnCall { .. }
            | HirEagerExprData::AssociatedFunctionFnCall { .. }
            | HirEagerExprData::MemoizedField { .. }
            | HirEagerExprData::MethodFnCall { .. }
            | HirEagerExprData::Suffix { .. }
            | HirEagerExprData::Unveil { .. }
            | HirEagerExprData::Unwrap { .. } => match entry.ty_place {
                HirPlace::Const | HirPlace::StackPure { .. } => !entry.is_ty_always_copyable,
                ty_place => match ty_place.location() {
                    Some(location) => match self.location_contract(location) {
                        Some(contract) => match contract {
                            HirEagerContract::Pure | HirEagerContract::Const => {
                                !entry.is_ty_always_copyable
                            }
                            HirEagerContract::Move => false,
                            HirEagerContract::Borrow => true,
                            HirEagerContract::BorrowMut => true,
                            HirEagerContract::Leash => todo!(),
                            HirEagerContract::At => todo!(),
                        },
                        None => false,
                    },
                    None => false,
                },
            },
            _ => false,
        }
    }

    // `precedence` is guaranteed to equal `hir_eager_expr_precedence(data)`
    // passed to avoid recomputing it
    fn transpile_hir_eager_expr_to_rust(
        &self,
        data: &HirEagerExprData,
        precedence: RustPrecedence,
        builder: &mut RustTranspilationBuilder<HirEagerExprRegion>,
    ) {
        let geq = |slf: &Self| slf.subexpr(RustPrecedenceRange::Geq(precedence));
        let greater = |slf: &Self| slf.subexpr(RustPrecedenceRange::Greater(precedence));
        let db = builder.db();
        match *data {
            HirEagerExprData::Literal(term_literal) => term_literal.transpile_to_rust(builder),
            HirEagerExprData::PrincipalEntityPath(principal_entity_path) => {
                principal_entity_path.transpile_to_rust(builder);
                match principal_entity_path {
                    PrincipalEntityPath::Module(_) => unreachable!(),
                    PrincipalEntityPath::MajorItem(MajorItemPath::Fugitive(path)) => {
                        match path.fugitive_kind(db) {
                            FugitiveKind::FunctionFn => (),
                            FugitiveKind::Val => builder.bracketed(RustBracket::Par, |_| ()),
                            FugitiveKind::FunctionGn => unreachable!(),
                            FugitiveKind::AliasType => unreachable!(),
                        }
                    }
                    PrincipalEntityPath::TypeVariant(_) => (),
                    PrincipalEntityPath::MajorItem(_) => (),
                }
            }
            HirEagerExprData::ConstSymbol { ident, .. } => ident.transpile_to_rust(builder),
            HirEagerExprData::Variable(hir_eager_runtime_symbol_idx) => {
                hir_eager_runtime_symbol_idx.transpile_to_rust(builder)
            }
            HirEagerExprData::Binary { lopd, opr, ropd } => match opr {
                HirBinaryOpr::Closed(BinaryClosedOpr::RemEuclid) => {
                    (
                        lopd,
                        self.self_expr_on_site(HirPlace::Transient, HirEagerContract::Pure, true),
                    )
                        .transpile_to_rust(builder);
                    builder.punctuation(RustPunctuation::Dot);
                    builder.rem_eulid();
                    builder.bracketed_list_with(RustBracket::Par, |builder| {
                        (ropd, self.any_precedence()).transpile_to_rust(builder)
                    })
                }
                HirBinaryOpr::Closed(BinaryClosedOpr::Power) => {
                    (
                        lopd,
                        self.self_expr_on_site(HirPlace::Transient, HirEagerContract::Pure, true),
                    )
                        .transpile_to_rust(builder);
                    builder.punctuation(RustPunctuation::Dot);
                    builder.pow();
                    builder.bracketed_list_with(RustBracket::Par, |builder| {
                        (ropd, self.any_precedence()).transpile_to_rust(builder)
                    })
                }
                HirBinaryOpr::Assign
                | HirBinaryOpr::AssignClosed(_)
                | HirBinaryOpr::AssignShift(_) => {
                    (
                        lopd,
                        self.self_expr_on_site(
                            builder.hir_eager_expr_arena()[lopd].ty_place,
                            HirEagerContract::BorrowMut,
                            false,
                        ),
                    )
                        .transpile_to_rust(builder);
                    opr.transpile_to_rust(builder);
                    (ropd, greater(self)).transpile_to_rust(builder)
                }
                _ => {
                    (lopd, geq(self)).transpile_to_rust(builder);
                    opr.transpile_to_rust(builder);
                    (ropd, greater(self)).transpile_to_rust(builder)
                }
            },
            HirEagerExprData::Be { src: _, target: _ } => {
                builder.macro_name(RustMacroName::Matches)
            }
            HirEagerExprData::Prefix {
                opr,
                opd_hir_expr_idx,
            } => {
                match opr {
                    HirPrefixOpr::NotInt => builder.bracketed(RustBracket::Par, |builder| {
                        (
                            opd_hir_expr_idx,
                            self.subexpr(RustPrecedenceRange::Geq(RustPrecedence::EqComparison)),
                        )
                            .transpile_to_rust(builder);
                        builder.eq_zero()
                    }),
                    _ => {
                        // todo: check some details
                        opr.transpile_to_rust(builder);
                        (opd_hir_expr_idx, geq(self)).transpile_to_rust(builder)
                    }
                }
            }
            HirEagerExprData::Suffix {
                opd_hir_expr_idx,
                opr,
            } => match opr {
                HirSuffixOpr::Incr | HirSuffixOpr::Decr => {
                    (opd_hir_expr_idx, geq(self)).transpile_to_rust(builder);
                    opr.transpile_to_rust(builder)
                }
            },
            HirEagerExprData::Unveil {
                opd_hir_expr_idx,
                return_ty,
                unveil_associated_fn_path,
                ref instantiation,
            } => {
                builder.macro_name(RustMacroName::Unveil);
                builder.bracketed(RustBracket::Par, |builder| {
                    return_ty.transpile_to_rust(builder);
                    builder.punctuation(RustPunctuation::CommaSpaced);
                    (opd_hir_expr_idx, geq(self)).transpile_to_rust(builder);
                    builder.punctuation(RustPunctuation::CommaSpaced);
                    let runtime_constants: SmallVec<[HirTermSymbolResolution; 2]> = instantiation
                        .symbol_map()
                        .iter()
                        .filter_map(|&(symbol, resolution)| match symbol {
                            HirTemplateSymbol::Const(symbol) => (symbol.index(db).class()
                                == HirTemplateSymbolClass::Runtime)
                                .then_some(resolution),
                            _ => None,
                        })
                        .collect();
                    builder
                        .bracketed_comma_list_with_last_comma(RustBracket::Par, runtime_constants)
                })
            }
            HirEagerExprData::Unwrap { opd_hir_expr_idx } => {
                (opd_hir_expr_idx, geq(self)).transpile_to_rust(builder);
                builder.call_unwrap()
            }
            HirEagerExprData::TypeConstructorFnCall {
                path,
                ref instantiation,
                ref item_groups,
            } => {
                builder.ty_constructor_linkage(path);
                builder.bracketed_comma_list(
                    RustBracket::Par,
                    item_groups.iter().map(|item_group| (item_group, self)),
                )
            }
            HirEagerExprData::TypeVariantConstructorCall {
                path,
                ref instantiation,
                ref item_groups,
            } => {
                path.transpile_to_rust(builder);
                builder.bracketed_comma_list(
                    RustBracket::Par,
                    item_groups.iter().map(|item_group| (item_group, self)),
                )
            }
            HirEagerExprData::FunctionFnCall {
                path,
                ref instantiation,
                ref item_groups,
            } => {
                path.transpile_to_rust(builder);
                builder.bracketed_comma_list(
                    RustBracket::Par,
                    item_groups.iter().map(|item_group| (item_group, self)),
                )
            }
            HirEagerExprData::AssociatedFunctionFnCall {
                path,
                ref instantiation,
                ref item_groups,
            } => {
                path.transpile_to_rust(builder);
                builder.bracketed_comma_list(
                    RustBracket::Par,
                    item_groups.iter().map(|item_group| (item_group, self)),
                )
            }
            HirEagerExprData::PropsStructField {
                owner_hir_expr_idx,
                ident,
                field_ty,
            } => {
                (
                    owner_hir_expr_idx,
                    self.self_expr_on_site(
                        builder.hir_eager_expr_arena()[owner_hir_expr_idx].ty_place,
                        HirEagerContract::At,
                        true,
                    ),
                )
                    .transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                ident.transpile_to_rust(builder)
            }
            HirEagerExprData::MemoizedField {
                owner_hir_expr_idx,
                ident,
                ..
            } => {
                (owner_hir_expr_idx, geq(self)).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                ident.transpile_to_rust(builder);
                builder.bracketed(RustBracket::Par, |_| ())
            }
            HirEagerExprData::MethodFnCall {
                self_argument,
                self_contract,
                ident,
                path,
                ref instantiation,
                ref item_groups,
            } => {
                (
                    self_argument,
                    self.self_expr_on_site(
                        builder.hir_eager_expr_arena()[self_argument].ty_place,
                        self_contract,
                        true,
                    ),
                )
                    .transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                let places = instantiation.places();
                match places.len() {
                    0 => ident.transpile_to_rust(builder),
                    1 => match places[0].location() {
                        Some(location) => match self.location_contract(location) {
                            Some(contract) => match contract {
                                HirEagerContract::Pure => ident.transpile_to_rust(builder),
                                HirEagerContract::Move => todo!(),
                                HirEagerContract::Borrow => todo!(),
                                HirEagerContract::BorrowMut => builder.method_fn_ident_mut(ident),
                                HirEagerContract::Const => todo!(),
                                HirEagerContract::Leash => todo!(),
                                HirEagerContract::At => todo!(),
                            },
                            None => todo!(),
                        },
                        None => ident.transpile_to_rust(builder),
                    },
                    _ => todo!(),
                }
                builder.bracketed_comma_list(
                    RustBracket::Par,
                    item_groups.iter().map(|item_group| (item_group, self)),
                )
            }
            HirEagerExprData::NewTuple { items: _ } => {
                todo!()
            }
            HirEagerExprData::Index {
                owner_hir_expr_idx,
                ref items,
            } => {
                // ad hoc
                (
                    owner_hir_expr_idx,
                    self.self_expr_on_site(
                        builder.hir_eager_expr_arena()[owner_hir_expr_idx].ty_place,
                        HirEagerContract::At,
                        true,
                    ),
                )
                    .transpile_to_rust(builder);
                builder.bracketed(RustBracket::Box, |builder| {
                    (
                        items[0],
                        self.subexpr(RustPrecedenceRange::Geq(RustPrecedence::As)),
                    )
                        .transpile_to_rust(builder);
                    builder.keyword(RustKeyword::As);
                    builder.usize()
                })
            }
            HirEagerExprData::NewList { ref items, .. } => {
                builder.macro_name(RustMacroName::Vec);
                builder.bracketed_comma_list(
                    RustBracket::Box,
                    items
                        .iter()
                        .copied()
                        .map(|item| (item, self.any_precedence())),
                )
            }
            HirEagerExprData::Block { stmts } => stmts.transpile_to_rust(builder),
            HirEagerExprData::EmptyHtmlTag {
                function_ident: _,
                arguments: _,
            } =>
            /* ad hoc */
            {
                ()
            }
            HirEagerExprData::Todo => {
                builder.macro_name(RustMacroName::Todo);
                builder.bracketed(RustBracket::Par, |_| ())
            }
            HirEagerExprData::Unreachable => {
                builder.macro_name(RustMacroName::Unreachable);
                builder.bracketed(RustBracket::Par, |_| ())
            }
            HirEagerExprData::AssociatedFn {
                associated_item_path,
            } => associated_item_path.transpile_to_rust(builder),
            HirEagerExprData::As { opd, ty } => {
                (
                    opd,
                    self.subexpr(RustPrecedenceRange::Geq(RustPrecedence::As)),
                )
                    .transpile_to_rust(builder);
                builder.keyword(RustKeyword::As);
                ty.transpile_to_rust(builder)
            }
        }
    }
}
impl TranspileToRustWith<HirEagerExprRegion>
    for (&HirEagerRitchieParameterArgumentMatch, &HirEagerExprSite)
{
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let (slf, site) = self;
        match *slf {
            HirEagerRitchieParameterArgumentMatch::Regular(param, hir_eager_expr_idx, coersion) => {
                (
                    hir_eager_expr_idx,
                    site.regular_call_item(param, coersion, builder.db()),
                )
                    .transpile_to_rust(builder)
            }
            HirEagerRitchieParameterArgumentMatch::Variadic => todo!(),
            HirEagerRitchieParameterArgumentMatch::Keyed => todo!(),
        }
    }
}
