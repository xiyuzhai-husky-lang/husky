use crate::*;

// ad hoc
#[derive(Debug, PartialEq, Eq)]
pub struct VMStackIdx;

// ad hoc
#[derive(Debug, PartialEq, Eq)]
pub struct Binding;

// ad hoc
#[derive(Debug, PartialEq, Eq)]
pub struct __RegularValue;

// ad hoc
#[derive(Debug, PartialEq, Eq)]
pub struct __ResolvedLinkage;

// ad hoc
#[derive(Debug, PartialEq, Eq)]
pub struct VMLoopKind;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    PushVariable {
        stack_idx: VMStackIdx,
        binding: Binding,
        ty: EtherealTerm,
        varname: Ident,
        explicit: bool,
    },
    PushLiteralValue {
        value: __RegularValue,
        ty: EtherealTerm,
        explicit: bool,
    },
    WrapInSome {
        number_of_somes: u8,
    },
    CallRoutine {
        resolved_linkage: __ResolvedLinkage,
        nargs: u8,
        return_ty: EtherealTerm,
        discard: bool,
    },
    CallInterpreted {
        // routine_uid: EntityUid,
        nargs: u8,
        return_ty: EtherealTerm,
        discard: bool,
    },
    VirtualStructField {
        field_idx: u8,
        field_binding: Binding,
        field_ty: EtherealTerm,
    },
    NewVirtualStruct {
        ty: EtherealTerm,
        fields: Vec<Ident>,
    },
    Loop {
        body: Instructions,
        loop_kind: VMLoopKind,
    },
    Return {
        return_ty: EtherealTerm,
    },
    BreakIfFalse,
    Break,
    Assert,
    Require,
    ConditionFlow {
        // branches: Avec<VMConditionBranch>,
    },
    PatternMatch {
        // branches: Avec<VMPatternBranch>,
    },
    EntityFeature {
        // feature_uid: EntityUid,
        ty: EtherealTerm,
    },
    PushEntityFp {
        // opt_linkage: Option<__Linkage>,
        ty: EtherealTerm,
        opt_instruction_sheet: Option<Instructions>,
    },
}
