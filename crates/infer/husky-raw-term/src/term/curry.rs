pub use context::*;

use crate::*;

/// representing term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = TermDb, jar = TermJar)]
pub struct TermCurry {
    pub curry_kind: TermCurryKind,
    pub variance: Variance,
    /// a
    pub input_symbol: Option<TermSymbol>,
    /// X
    pub input_ty: Term,
    /// Y
    pub return_ty: Term,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TermCurryKind {
    Explicit,
    Implicit,
}

impl TermCurry {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        let input_symbol = self.input_symbol(db);
        if input_symbol.is_some() {
            f.write_str("(")?
        }
        f.write_str(match self.variance(db) {
            Independent => "independent ",
            Covariant => "covariant ",
            Contravariant => "contravariant ",
            Invariant => "invariant ",
        })?;
        if let Some(input_symbol) = input_symbol {
            ctx.fmt_with_symbol(db, input_symbol, |ctx| {
                ctx.fmt_symbol(db, input_symbol, f);
                f.write_str(": ")?;
                self.input_ty(db).show_with_db_fmt(f, db, ctx)?;
                f.write_str(") -> ")?;
                self.return_ty(db).show_with_db_fmt(f, db, ctx)
            })
        } else {
            self.input_ty(db).show_with_db_fmt(f, db, ctx)?;
            f.write_str(" -> ")?;
            self.return_ty(db).show_with_db_fmt(f, db, ctx)
        }
    }
}

impl<Db: TermDb + ?Sized> salsa::DisplayWithDb<Db> for TermCurry {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl TermRewriteCopy for TermCurry {
    fn substitute_copy(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self {
        todo!()
    }
}

use Variance::*;

/// Describes attribute for a curry `f: X -> Y`.
///
/// If `X` is not a type type, `Variance` can only be invariant.
///
/// If `X` is a type type,
/// - `Independent` implies that `f x` is independent of `x`;
/// - `Covariant` implies that if `f x1` is a subtype of `f x2`,
/// then `x1` is a subtype of `x2`;
/// - `Contravariant` implies that if `f x1` is a subtype of `f x2`,
/// then `x2` is a subtype of `x1`;
/// - `Invariant` implies that if `f x1` is a subtype of `f x2`,
/// then `x1` is the same type as `x2`.
///
/// Examples of covariant:
///
/// ```husky
/// Vec: ∀ u, Type u -> Type u
/// X -> _: ∀ u, Type u -> Type u
/// ```
///
/// Examples of contravariant:
///
/// ```husky
/// _ -> X: ∀ u, Type u -> Type u
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Variance {
    Independent = 0b00,
    Covariant = 0b01,
    Contravariant = 0b10,
    Invariant = 0b11,
}

impl Variance {
    pub fn merge(self, other: Self) -> Self {
        unsafe { std::mem::transmute((self as u8) | (other as u8)) }
    }
    /// self \circ other
    /// what is the variance?
    pub fn compose(self, other: Self) -> Self {
        match self {
            Independent => Independent,
            Covariant => other,
            Contravariant => match other {
                Independent => Independent,
                Covariant => Contravariant,
                Contravariant => Covariant,
                Invariant => Invariant,
            },
            Invariant => match other {
                Independent => Independent,
                Covariant | Contravariant | Invariant => Invariant,
            },
        }
    }
}

static ALL_VARIANCES: [Variance; 4] = [Invariant, Covariant, Contravariant, Independent];

impl std::ops::BitOr for Variance {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.merge(rhs)
    }
}

impl std::ops::Mul for Variance {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.compose(rhs)
    }
}

#[test]
fn variance_merge_works() {
    assert_eq!(Invariant | Invariant, Invariant);
    assert_eq!(Invariant | Covariant, Invariant);
    assert_eq!(Invariant | Contravariant, Invariant);
    assert_eq!(Invariant | Independent, Invariant);
    assert_eq!(Covariant | Invariant, Invariant);
    assert_eq!(Covariant | Covariant, Covariant);
    assert_eq!(Covariant | Contravariant, Invariant);
    assert_eq!(Covariant | Independent, Covariant);
    assert_eq!(Contravariant | Invariant, Invariant);
    assert_eq!(Contravariant | Covariant, Invariant);
    assert_eq!(Contravariant | Contravariant, Contravariant);
    assert_eq!(Contravariant | Independent, Contravariant);
    assert_eq!(Independent | Invariant, Invariant);
    assert_eq!(Independent | Covariant, Covariant);
    assert_eq!(Independent | Contravariant, Contravariant);
    assert_eq!(Independent | Independent, Independent);
}

#[test]
fn variance_merge_is_commutative_works() {
    for u in ALL_VARIANCES {
        for v in ALL_VARIANCES {
            assert_eq!(u | v, v | u)
        }
    }
}

#[test]
fn variance_merge_is_nondecreasing_works() {
    for u in ALL_VARIANCES {
        for v in ALL_VARIANCES {
            assert!(((u | v) as u8) >= u as u8)
        }
    }
}

#[test]
fn variance_compose_is_commutative_works() {
    for u in ALL_VARIANCES {
        for v in ALL_VARIANCES {
            assert_eq!(u * v, v * u)
        }
    }
}

#[test]
fn variance_merge_compose_distribution_works() {
    for u in ALL_VARIANCES {
        for v in ALL_VARIANCES {
            for w in ALL_VARIANCES {
                assert_eq!(u * (v | w), (u * v) | (u * w))
            }
        }
    }
}
