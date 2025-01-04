use floated_sequential::floated;
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;
use visored_mir_expr::symbol::local_defn::VdMirSymbolLocalDefnIdx;

pub type VdMirTermFld<'sess> = &'sess ();

pub enum VdMirTerm<'sess> {
    Rational(RationalVdBaseqTerm),
    Irrational(IrrationalVdBaseqTerm<'sess>),
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum RationalVdBaseqTerm {
    Nat128(u128),
    Int128(i128),
    BigInt(/* TODO */),
    Rat128(i128, u128),
}

#[floated]
pub struct IrrationalVdBaseqTerm<'sess> {
    #[return_ref]
    pub data: IrrationalVdBaseqTermData<'sess>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum IrrationalVdBaseqTermData<'sess> {
    Atom,
    Product {
        rational: RationalVdBaseqTerm,
        irrational_atom_exponentials: IrrationalAtomExponentials<'sess>,
    },
    Sum {
        constant_term: RationalVdBaseqTerm,
        irrational_monomial_coefficients: IrrationalMonomialCoefficients<'sess>,
    },
    Variable(VdMirSymbolLocalDefnIdx),
}

pub type IrrationalMonomialCoefficients<'sess> = IrrationalTermMap<'sess, RationalVdBaseqTerm>;
pub type IrrationalAtomExponentials<'sess> = IrrationalTermMap<'sess, IrrationalVdBaseqTerm<'sess>>;

pub type IrrationalTermMap<'sess, T> = OrderedSmallVecPairMap<IrrationalVdBaseqTerm<'sess>, T, 4>;

impl<'sess> std::fmt::Debug for IrrationalVdBaseqTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
