mod closure;
mod html;
pub(crate) mod pattern;
pub(crate) mod precedence;
pub(crate) mod site;
mod stmt;

pub(crate) use self::precedence::{RustPrecedence, RustPrecedenceRange};

use self::{precedence::hir_eager_expr_precedence, site::HirEagerExprSite};
use crate::{binding::RustBinding, *};
use either::*;
use husky_entity_kind::MajorFormKind;
use husky_entity_path::path::{
    assoc_item::AssocItemPath,
    major_item::{ty::PreludeTypePath, MajorItemPath},
    ItemPath, PrincipalEntityPath,
};
use husky_hir_eager_expr::{
    coercion::HirEagerCoercion, emit_note_on_hir_eager_expr_codespan, HirEagerCondition,
    HirEagerElifBranch, HirEagerElseBranch, HirEagerExprData, HirEagerExprEntry, HirEagerExprIdx,
    HirEagerExprRegion, HirEagerIfBranch, HirEagerPatternData, HirEagerPatternIdx,
    HirEagerRitchieArgument, HirEagerStmtData, HirEagerStmtIdx, HirEagerStmtIdxRange,
};
use husky_hir_opr::{binary::HirBinaryOpr, prefix::HirPrefixOpr, suffix::HirSuffixOpr};
use husky_hir_ty::{
    instantiation::HirTermSymbolicVariableResolution, place_contract_site::HirPlaceContractSite,
    quary::HirQuary, ritchie::HirContract, HirTemplateVariable, HirTemplateVariableClass, HirType,
};
use husky_opr::BinaryClosedOpr;
use salsa::DebugWithDb;
use smallvec::SmallVec;

impl TranspileToRustWith<HirEagerExprRegion> for (HirEagerExprIdx, HirEagerExprSite) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let (slf, mut site) = self;
        let entry = slf.entry(builder.hir_eager_expr_arena());
        let data = entry.data();
        let place_contract_site = entry.place_contract_site();
        let db = builder.db;
        let precedence = hir_eager_expr_precedence(data);
        let needs_deref = site.hir_eager_expr_needs_deref(entry);
        if needs_deref {
            site.rust_bindings.push(RustBinding::Deref)
        }
        // it should be noted that block expressions never need releash
        let needs_releash = match entry.coercion() {
            Some(coercion) => match coercion {
                HirEagerCoercion::Trivial(_) => false,
                HirEagerCoercion::Never => false,
                HirEagerCoercion::WrapInSome => false,
                HirEagerCoercion::Releash => true,
                HirEagerCoercion::Deref(_) => false,
            },
            None => false,
        };
        if needs_releash {
            site.rust_bindings.push(RustBinding::Releash)
        }
        use husky_print_utils::p;
        p!(site.rust_bindings);
        let mut releash_flag = false;
        let mut wrap_in_some_flag = false;
        if let Some(rust_binding) = site.rust_bindings.last() {
            match rust_binding {
                RustBinding::Deref
                | RustBinding::DerefCustomed
                | RustBinding::Reref
                | RustBinding::RerefMut
                | RustBinding::Releash => {
                    site.rust_precedence_range = RustPrecedenceRange::Geq(RustPrecedence::Prefix)
                }
                /// both has the precedence of suffix
                RustBinding::Deleash | RustBinding::SelfValue => (),
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
                RustBinding::Deleash
                | RustBinding::Releash
                | RustBinding::SelfValue
                | RustBinding::WrapInSome => false,
            })
            .unwrap_or(false);
        if needs_extra_pars {
            builder.lpar();
        }
        for (i, rust_binding) in site.rust_bindings.iter().copied().enumerate() {
            match rust_binding {
                RustBinding::Deref | RustBinding::DerefCustomed => {
                    builder.punctuation(RustPunctuation::DerefStar)
                }
                RustBinding::Deleash => {
                    if i < site.rust_bindings.len() - 1 {
                        match site.rust_bindings[i + 1] {
                            RustBinding::Deref => todo!(),
                            RustBinding::DerefCustomed => todo!(),
                            RustBinding::Deleash => todo!(),
                            RustBinding::Reref => todo!(),
                            RustBinding::RerefMut => todo!(),
                            RustBinding::Releash => todo!(),
                            RustBinding::SelfValue => (),
                            RustBinding::WrapInSome => todo!(),
                        }
                    }
                }
                RustBinding::Reref => builder.punctuation(RustPunctuation::Ambersand),
                RustBinding::RerefMut => {
                    builder.punctuation(RustPunctuation::Ambersand);
                    builder.keyword(RustKeyword::Mut)
                }
                RustBinding::Releash => builder.releash_left(&mut releash_flag),
                RustBinding::SelfValue => (),
                RustBinding::WrapInSome => builder.wrap_in_some_left(&mut wrap_in_some_flag),
            }
        }
        if !site.rust_precedence_range.include(precedence) {
            builder.delimited_heterogeneous_list_with(RustDelimiter::Par, |builder| {
                transpile_hir_eager_expr_to_rust(data, place_contract_site, precedence, builder)
            })
        } else {
            transpile_hir_eager_expr_to_rust(data, place_contract_site, precedence, builder)
        }
        for (i, rust_binding) in site.rust_bindings.iter().copied().enumerate() {
            match rust_binding {
                RustBinding::Deref | RustBinding::DerefCustomed => (),
                RustBinding::Deleash => {
                    if i < site.rust_bindings.len() - 1 {
                        match site.rust_bindings[i + 1] {
                            RustBinding::Deref => todo!(),
                            RustBinding::DerefCustomed => todo!(),
                            RustBinding::Deleash => todo!(),
                            RustBinding::Reref => todo!(),
                            RustBinding::RerefMut => todo!(),
                            RustBinding::Releash => todo!(),
                            RustBinding::SelfValue => (),
                            RustBinding::WrapInSome => todo!(),
                        }
                    }
                    builder.deleash()
                }
                RustBinding::Reref => (),
                RustBinding::RerefMut => (),
                RustBinding::Releash => (),
                RustBinding::SelfValue => (),
                RustBinding::WrapInSome => (),
            }
        }
        if releash_flag {
            builder.releash_right()
        }
        if wrap_in_some_flag {
            builder.wrap_in_some_right()
        }
        if needs_extra_pars {
            builder.rpar();
        }
    }
}

// `precedence` is guaranteed to equal `hir_eager_expr_precedence(data)`
// passed to avoid recomputing it
fn transpile_hir_eager_expr_to_rust(
    data: &HirEagerExprData,
    place_contract_site: &HirPlaceContractSite,
    precedence: RustPrecedence,
    builder: &mut RustTranspilationBuilder<HirEagerExprRegion>,
) {
    let subexpr_geq = || HirEagerExprSite::subexpr(RustPrecedenceRange::Geq(precedence));
    let subexpr_greater = || HirEagerExprSite::subexpr(RustPrecedenceRange::Greater(precedence));
    let db = builder.db();
    match *data {
        HirEagerExprData::Literal(term_literal) => term_literal.transpile_to_rust(builder),
        HirEagerExprData::PrincipalEntityPath(principal_entity_path) => {
            principal_entity_path.transpile_to_rust(builder);
            match principal_entity_path {
                PrincipalEntityPath::Module(_) => unreachable!(),
                PrincipalEntityPath::MajorItem(MajorItemPath::Form(path)) => match path.kind(db) {
                    MajorFormKind::Ritchie(_) => (),
                    MajorFormKind::TypeAlias => (),
                    MajorFormKind::TypeVar => (),
                    MajorFormKind::Val | MajorFormKind::StaticVar => {
                        builder.delimited(RustDelimiter::Par, |_| ())
                    }
                    MajorFormKind::StaticMut => todo!(),
                    MajorFormKind::Compterm => (),
                    MajorFormKind::Conceptual => todo!(),
                },
                PrincipalEntityPath::TypeVariant(_) => (),
                PrincipalEntityPath::MajorItem(_) => (),
            }
        }
        HirEagerExprData::ConstVariable { ident, .. } => ident.transpile_to_rust(builder),
        HirEagerExprData::Variable(hir_eager_runtime_symbol_idx) => {
            hir_eager_runtime_symbol_idx.transpile_to_rust(builder)
        }
        HirEagerExprData::Binary { lopd, opr, ropd } => match opr {
            HirBinaryOpr::Closed(BinaryClosedOpr::RemEuclid) => {
                (lopd, HirEagerExprSite::simple_self_argument(false)).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                builder.rem_eulid();
                builder.delimited_heterogeneous_list_with(RustDelimiter::Par, |builder| {
                    (ropd, HirEagerExprSite::any_precedence()).transpile_to_rust(builder)
                })
            }
            HirBinaryOpr::Closed(BinaryClosedOpr::Power) => {
                (lopd, HirEagerExprSite::simple_self_argument(false)).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::Dot);
                builder.pow();
                builder.delimited_heterogeneous_list_with(RustDelimiter::Par, |builder| {
                    (ropd, HirEagerExprSite::any_precedence()).transpile_to_rust(builder)
                })
            }
            HirBinaryOpr::Assign | HirBinaryOpr::AssignClosed(_) | HirBinaryOpr::AssignShift(_) => {
                (lopd, HirEagerExprSite::simple_self_argument(false)).transpile_to_rust(builder);
                opr.transpile_to_rust(builder);
                (ropd, subexpr_greater()).transpile_to_rust(builder)
            }
            _ => {
                (lopd, subexpr_geq()).transpile_to_rust(builder);
                opr.transpile_to_rust(builder);
                (ropd, subexpr_greater()).transpile_to_rust(builder)
            }
        },
        HirEagerExprData::Be { src: _, pattern: _ } => builder.macro_name(RustMacroName::Matches),
        HirEagerExprData::Prefix { opr, opd } => {
            match opr {
                HirPrefixOpr::NotInt => builder.delimited(RustDelimiter::Par, |builder| {
                    (
                        opd,
                        HirEagerExprSite::subexpr(RustPrecedenceRange::Geq(
                            RustPrecedence::EqComparison,
                        )),
                    )
                        .transpile_to_rust(builder);
                    builder.eq_zero()
                }),
                _ => {
                    // todo: check some details
                    opr.transpile_to_rust(builder);
                    (opd, subexpr_geq()).transpile_to_rust(builder)
                }
            }
        }
        HirEagerExprData::Suffix { opd, opr } => match opr {
            HirSuffixOpr::Incr | HirSuffixOpr::Decr => {
                (opd, subexpr_geq()).transpile_to_rust(builder);
                opr.transpile_to_rust(builder)
            }
        },
        HirEagerExprData::Unveil {
            opd,
            return_ty,
            ref instantiation,
            ..
        } => {
            builder.macro_name(RustMacroName::Unveil);
            builder.delimited(RustDelimiter::Par, |builder| {
                return_ty.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::CommaSpaced);
                (opd, subexpr_geq()).transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::CommaSpaced);
                let runtime_constants: SmallVec<[HirTermSymbolicVariableResolution; 2]> =
                    instantiation
                        .symbol_map()
                        .iter()
                        .filter_map(|&(symbol, resolution)| match symbol {
                            HirTemplateVariable::Compterm(symbol) => (symbol.index(db).class()
                                == HirTemplateVariableClass::Poly)
                                .then_some(resolution),
                            _ => None,
                        })
                        .collect();
                builder.delimited_comma_list_with_last_comma(RustDelimiter::Par, runtime_constants)
            })
        }
        HirEagerExprData::Unwrap { opd } => {
            (opd, HirEagerExprSite::simple_self_argument(false)).transpile_to_rust(builder);
            builder.call_unwrap()
        }
        HirEagerExprData::TypeConstructorCall {
            path,
            instantiation: _,
            ref arguments,
        } => {
            builder.struct_ty_constructor_path(path);
            builder.delimited_comma_list(RustDelimiter::Par, arguments)
        }
        HirEagerExprData::TypeVariantConstructorCall {
            path,
            instantiation: _,
            ref arguments,
        } => {
            path.transpile_to_rust(builder);
            builder.delimited_comma_list(RustDelimiter::Par, arguments)
        }
        HirEagerExprData::FunctionRitchieCall {
            path,
            instantiation: _,
            ref arguments,
        } => {
            path.transpile_to_rust(builder);
            builder.delimited_comma_list(RustDelimiter::Par, arguments)
        }
        HirEagerExprData::AssocFunctionRitchieCall {
            path,
            ref arguments,
            ..
        } => {
            path.transpile_to_rust(builder);
            builder.delimited_comma_list(RustDelimiter::Par, arguments)
        }
        HirEagerExprData::PropsStructField {
            self_argument,
            self_ty,
            ident,
            ref indirections,
            ..
        } => {
            if ident.data(db) == "points"
                && (self_ty.is_always_leashed(db))
                && (indirections.is_empty())
            {
                use husky_print_utils::p;
                use salsa::DebugWithDb;

                p!(self_ty.debug(db), indirections.debug(db));
                todo!()
            }
            (
                self_argument,
                HirEagerExprSite::self_argument_with_indirections(indirections),
            )
                .transpile_to_rust(builder);
            builder.punctuation(RustPunctuation::Dot);
            ident.transpile_to_rust(builder)
        }
        HirEagerExprData::MemoizedField {
            self_argument,
            self_argument_ty,
            ident,
            ref indirections,
            ref instantiation,
            ..
        } => {
            let ItemPath::AssocItem(path) = instantiation.path() else {
                unreachable!()
            };
            let self_ty = self_argument_ty.deref_if_leashed(db);
            match path {
                AssocItemPath::TypeItem(_) => {
                    builder.delimited(RustDelimiter::Angle, |builder| {
                        self_ty.transpile_to_rust(builder)
                    });
                }
                AssocItemPath::TraitItem(_) => todo!(),
                AssocItemPath::TraitForTypeItem(_) => todo!(),
            }
            builder.punctuation(RustPunctuation::ColonColon);
            ident.transpile_to_rust(builder);
            builder.delimited(RustDelimiter::Par, |builder| {
                (
                    self_argument,
                    HirEagerExprSite::memoized_field_self_argument(
                        self_argument_ty,
                        indirections,
                        db,
                    ),
                )
                    .transpile_to_rust(builder)
            });
        }
        HirEagerExprData::MethodRitchieCall {
            self_argument,
            self_contract,
            ident,
            path,
            ref indirections,
            ref instantiation,
            ref arguments,
        } => {
            (
                self_argument,
                HirEagerExprSite::self_argument_with_indirections(indirections),
            )
                .transpile_to_rust(builder);
            builder.punctuation(RustPunctuation::Dot);
            let contracted_quaries = instantiation.contracted_quaries();
            match contracted_quaries.len() {
                0 => ident.transpile_to_rust(builder),
                1 => match contracted_quaries[0].quary().place() {
                    Some(place) => match place_contract_site[place] {
                        HirContract::Pure => ident.transpile_to_rust(builder),
                        HirContract::Move => todo!(),
                        HirContract::Borrow => todo!(),
                        HirContract::BorrowMut => builder.method_ritchie_ident_mut(ident),
                        HirContract::Compterm => todo!(),
                        HirContract::Leash => todo!(),
                        HirContract::At => todo!(),
                    },
                    None => ident.transpile_to_rust(builder),
                },
                _ => todo!(),
            }
            match path.ident(db).unwrap().data(db) {
                // ad hoc, should use path menu instead or refinement
                "visualize" => builder.delimited(RustDelimiter::Par, |builder| {
                    builder.visual_synchrotron_argument()
                }),
                _ => builder.delimited_comma_list(RustDelimiter::Par, arguments),
            }
        }
        HirEagerExprData::NewTuple { items: _ } => {
            todo!()
        }
        HirEagerExprData::Index { owner, ref items } => {
            // ad hoc
            (owner, HirEagerExprSite::simple_self_argument(true)).transpile_to_rust(builder);
            builder.delimited(RustDelimiter::Box, |builder| {
                (
                    items[0],
                    HirEagerExprSite::subexpr(RustPrecedenceRange::Geq(RustPrecedence::As)),
                )
                    .transpile_to_rust(builder);
                builder.keyword(RustKeyword::As);
                builder.usize()
            })
        }
        HirEagerExprData::NewList {
            exprs: ref items, ..
        } => {
            builder.macro_name(RustMacroName::Vec);
            builder.delimited_comma_list(
                RustDelimiter::Box,
                items
                    .iter()
                    .copied()
                    .map(|item| (item, HirEagerExprSite::any_precedence())),
            )
        }
        HirEagerExprData::Block { stmts } => stmts.transpile_to_rust(builder),
        HirEagerExprData::EmptyHtmlTag {
            function_ident,
            ref arguments,
        } => {
            let macro_name = RustMacroName::HtmlTag(function_ident);
            builder.macro_name(macro_name);
            builder.delimited_heterogeneous_list_with(RustDelimiter::Par, |builder| {
                builder.heterogeneous_comma_list_items(arguments.iter());
                builder.heterogeneous_comma_list_item_with(|builder| {
                    builder.visual_synchrotron_argument()
                })
            })
        }
        HirEagerExprData::Todo => {
            builder.macro_name(RustMacroName::Todo);
            builder.delimited(RustDelimiter::Par, |_| ())
        }
        HirEagerExprData::Unreachable => {
            builder.macro_name(RustMacroName::Unreachable);
            builder.delimited(RustDelimiter::Par, |_| ())
        }
        HirEagerExprData::AssocRitchie { assoc_item_path } => {
            assoc_item_path.transpile_to_rust(builder)
        }
        HirEagerExprData::As { opd, ty } => {
            (
                opd,
                HirEagerExprSite::subexpr(RustPrecedenceRange::Geq(RustPrecedence::As)),
            )
                .transpile_to_rust(builder);
            builder.keyword(RustKeyword::As);
            ty.transpile_to_rust(builder)
        }
        HirEagerExprData::Closure {
            ref parameters,
            return_ty,
            body,
            ..
        } => {
            builder.delimited_comma_list(RustDelimiter::Vert, parameters);
            match return_ty {
                Some(return_ty) => {
                    builder.punctuation(RustPunctuation::LightArrow);
                    return_ty.transpile_to_rust(builder);
                    builder.delimited(RustDelimiter::Curl, |builder| {
                        (body, HirEagerExprSite::subexpr(RustPrecedenceRange::Any))
                            .transpile_to_rust(builder)
                    })
                }
                None => {
                    (
                        body,
                        HirEagerExprSite::subexpr(RustPrecedenceRange::Geq(
                            RustPrecedence::Closure,
                        )),
                    )
                        .transpile_to_rust(builder);
                }
            }
        }
    }
}

impl HirEagerExprSite {
    fn hir_eager_expr_needs_deref(&self, entry: &HirEagerExprEntry) -> bool {
        match *entry.data() {
            HirEagerExprData::Variable(_) => match entry.quary() {
                HirQuary::Compterm | HirQuary::StackPure { .. } => !entry.is_always_copyable(),
                HirQuary::Ref { .. } => true,
                HirQuary::RefMut { .. } => true,
                _ => false,
            },
            HirEagerExprData::ConstVariable { .. }
            | HirEagerExprData::FunctionRitchieCall { .. }
            | HirEagerExprData::AssocFunctionRitchieCall { .. }
            | HirEagerExprData::MemoizedField { .. }
            | HirEagerExprData::MethodRitchieCall { .. }
            | HirEagerExprData::Suffix { .. }
            | HirEagerExprData::Unveil { .. }
            | HirEagerExprData::Unwrap { .. } => match entry.quary() {
                HirQuary::Compterm | HirQuary::StackPure { .. } => !entry.is_always_copyable(),
                quary => match quary.place() {
                    Some(place) => match entry.place_contract_site().get(place) {
                        Some(contract) => match contract {
                            HirContract::Pure | HirContract::Compterm => {
                                !entry.is_always_copyable()
                            }
                            HirContract::Move => false,
                            HirContract::Borrow => true,
                            HirContract::BorrowMut => true,
                            HirContract::Leash => todo!(),
                            HirContract::At => todo!(),
                        },
                        None => false,
                    },
                    None => false,
                },
            },
            _ => false,
        }
    }

    fn hir_eager_expr_needs_releash(&self, entry: &HirEagerExprEntry) -> bool {
        todo!()
    }
}
impl TranspileToRustWith<HirEagerExprRegion> for &HirEagerRitchieArgument {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        match *self {
            HirEagerRitchieArgument::Simple(param, hir_eager_expr_idx, coercion) => (
                hir_eager_expr_idx,
                HirEagerExprSite::regular_call_item(param, coercion, builder.db()),
            )
                .transpile_to_rust(builder),
            HirEagerRitchieArgument::Variadic => todo!(),
            HirEagerRitchieArgument::Keyed => todo!(),
        }
    }
}
