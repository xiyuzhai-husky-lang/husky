mod flag;

use self::flag::*;
use crate::*;
use ad_hoc_task_dependency::val_control_flow::ValControlFlow;
use smallvec::SmallVec;

#[allow(warnings, non_snake_case)]
pub struct narrow_down<Label>(std::marker::PhantomData<Label>);

#[derive(Debug, PartialEq, Eq, Clone)]
#[value_conversion]
pub struct NarrowDownInternal<Label> {
    label0: Label,
    flag_ranges: SmallVec<[FlagRange; 4]>,
}

impl<Label> __IsGnItem for narrow_down<Label>
where
    Label: IsLabel,
{
    type LinkageImpl = __LinkageImpl;

    fn generic_pedestal(generic_pedestal: __Pedestal) -> __Pedestal {
        __Pedestal::Generic
    }

    type ValueAtGenericPedestal = NarrowDownInternal<Label>;

    fn train(
        val_domain_repr: __ValDomainReprInterface,
        val_argument_reprs: &[__ValArgumentReprInterface],
    ) -> __ValControlFlow<Self::ValueAtGenericPedestal> {
        debug_assert_eq!(val_argument_reprs.len(), 3);
        let __ValArgumentReprInterface::Variadic(ref features) = val_argument_reprs[0] else {
            unreachable!()
        };
        let __ValArgumentReprInterface::Keyed(skip) = val_argument_reprs[1] else {
            unreachable!()
        };
        let skip: i32 = match skip {
            Some(skip) => __eval_val_repr(skip)?,
            None => 5,
        };
        let __ValArgumentReprInterface::RuntimeConstants(ref runtime_constants) =
            val_argument_reprs[2]
        else {
            unreachable!()
        };
        debug_assert_eq!(runtime_constants.len(), 1);
        let label: Label = __eval_val_runtime_constant(runtime_constants[0]);
        let fvf = FlagVectorField::from_features(val_domain_repr, features, label)?;
        // let fvf = FlagVectorField::from_registers(&opds[0], &opds[2..], &labels)?;
        // let ntrim = opds[1].value().downcast_i32();
        __ValControlFlow::Continue(NarrowDownInternal {
            label0: fvf.label0(),
            flag_ranges: fvf.flag_ranges(skip, 0.1),
        })
    }

    type EvalOutput = OneVsAllResult;

    fn eval(
        val_argument_reprs: &[__ValArgumentReprInterface],
        value_at_generic_pedestal: &Self::ValueAtGenericPedestal,
    ) -> OneVsAllResult {
        let __ValArgumentReprInterface::Variadic(ref features) = val_argument_reprs[0] else {
            unreachable!()
        };
        for (&feature, flag_range) in
            std::iter::zip(features, &value_at_generic_pedestal.flag_ranges)
        {
            let v: f32 = match __eval_val_repr(feature) {
                ValControlFlow::Continue(v) => v,
                ValControlFlow::LoopContinue => todo!(),
                ValControlFlow::LoopBreak(_) => todo!(),
                ValControlFlow::Return(_) => todo!(),
                ValControlFlow::Undefined => todo!(),
                ValControlFlow::Err(_) => todo!(),
            };
            let v = NotNan::new(v).unwrap();
            let apply_result = flag_range.apply(v);
            if !apply_result.within_false_range() && apply_result.within_true_range() {
                return OneVsAllResult::ConfidentYes;
            } else if apply_result.within_false_range() && !apply_result.within_true_range() {
                // corresponds to `return Some(None)` in Rust
                return OneVsAllResult::ConfidentNo;
            }
        }
        OneVsAllResult::Unconfident
    }
}
