use floated_sequential::floated;
use smallvec::SmallVec;
use visored_entity_path::path::VdItemPath;
use visored_mir_expr::{expr::application::VdMirFunc, symbol::local_defn::VdMirSymbolLocalDefnIdx};
use visored_opr::separator::VdBaseSeparator;
use visored_signature::signature::separator::base::VdBaseSeparatorSignature;
use visored_term::term::literal::VdLiteral;

// #[floated]
// pub struct VdMirExprFld<'sess> {
//     #[return_ref]
//     data: VdMirExprFldData<'sess>,
// }

// Recursive expansion of floated macro
// =====================================

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct __VdMirExprFldData {
    data: VdMirExprFldData<'static>,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VdMirExprFld<'db>(::floated_sequential::Floated<'db, __VdMirExprFldData>);

impl<'db> VdMirExprFld<'db> {
    pub fn new(data: VdMirExprFldData<'db>, db: &'db ::floated_sequential::db::FloaterDb) -> Self {
        use ::floated_sequential::Floated;
        let data = __VdMirExprFldData {
            data: unsafe { std::mem::transmute(data) },
        };
        VdMirExprFld(db.float(data))
    }
    pub fn data(self) -> &'db VdMirExprFldData<'db> {
        unsafe { std::mem::transmute(&self.0 .0.value.data) }
    }
}
impl<Q: ?Sized> std::borrow::Borrow<Q> for __VdMirExprFldData
where
    VdMirExprFldData<'static>: std::borrow::Borrow<Q>,
{
    fn borrow(&self) -> &Q {
        self.data.borrow()
    }
}
impl<'a, Q: ?Sized> From<&'a Q> for __VdMirExprFldData
where
    VdMirExprFldData<'static>: From<&'a Q>,
{
    fn from(q: &'a Q) -> Self {
        Self { data: q.into() }
    }
}
impl<'db> VdMirExprFld<'db> {
    pub fn from_ref<Q: Eq + std::hash::Hash + ?Sized>(
        q: &Q,
        db: &'db ::floated_sequential::db::FloaterDb,
    ) -> Self
    where
        VdMirExprFldData<'static>: std::borrow::Borrow<Q> + for<'a> From<&'a Q>,
    {
        VdMirExprFld(db.float_ref::<__VdMirExprFldData, Q>(q))
    }
}

impl<'sess> std::fmt::Debug for VdMirExprFld<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.data())
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdMirExprFldData<'sess> {
    Literal(VdLiteral),
    Variable(VdMirSymbolLocalDefnIdx),
    Application {
        function: VdMirFunc,
        arguments: VdMirExprFlds<'sess>,
    },
    FoldingSeparatedList {
        leader: VdMirExprFld<'sess>,
        /// TODO: should we use VdBaseSeparatorSignature instead?
        followers: SmallVec<[(VdMirFunc, VdMirExprFld<'sess>); 4]>,
    },
    ChainingSeparatedList {
        leader: VdMirExprFld<'sess>,
        followers: SmallVec<[(VdMirFunc, VdMirExprFld<'sess>); 4]>,
        joined_separator_and_signature: Option<(VdBaseSeparator, VdBaseSeparatorSignature)>,
    },
    ItemPath(VdItemPath),
}

pub type VdMirExprFlds<'sess> = SmallVec<[VdMirExprFld<'sess>; 4]>;
