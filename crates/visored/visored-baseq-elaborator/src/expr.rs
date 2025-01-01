use floated_sequential::floated;
use smallvec::SmallVec;
use visored_entity_path::path::VdItemPath;
use visored_mir_expr::{expr::application::VdMirFunc, symbol::local_defn::VdMirSymbolLocalDefnIdx};
use visored_opr::separator::VdBaseSeparator;
use visored_term::term::literal::VdLiteral;

#[floated]
pub struct VdMirExprFld<'sess> {
    #[return_ref]
    data: VdMirExprFldData<'sess>,
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
