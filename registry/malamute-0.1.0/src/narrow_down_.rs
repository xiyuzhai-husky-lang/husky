mod flag;

use self::flag::*;
use crate::*;
use ml_task::IsMlTask;
use smallvec::SmallVec;

#[allow(warnings, non_camel_case_types)]
pub struct narrow_down<Task, Label>(std::marker::PhantomData<(Task, Label)>);

impl<Task: IsMlTask<__VarId>, Label> narrow_down<Task, Label>
where
    Label: IsLabel
        + __Boiled<Thawed = Label>
        + __Thawed<Frozen = Label>
        + __Frozen<Thawed = Label>
        + __Serialize,
{
    pub fn gn_ki_wrapper(
        ki_repr_interface: __KiReprInterface,
        ki_domain_repr_interface: __KiDomainReprInterface,
        mut pedestal: __Pedestal,
        ki_argument_repr_interfaces: &[__KiArgumentReprInterface],
    ) -> __KiControlFlow {
        let generic_pedestal = pedestal.exclude::<Task::INPUT>();
        __eval_ki_domain_repr_interface(ki_domain_repr_interface)?;
        let internal: Leash<NarrowDownInternal<Label>> =
            __eval_generic_gn_with(ki_repr_interface, generic_pedestal, || {
                Self::train(ki_domain_repr_interface, ki_argument_repr_interfaces)
                    .map(__IntoValue::into_value)
            })?;
        __KiControlFlow::Continue(
            Self::eval(ki_argument_repr_interfaces, internal.deleash()).into_value(),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, __Serialize)]
// #[value_conversion]
pub struct NarrowDownInternal<Label> {
    label0: Label,
    flag_ranges: SmallVec<[Option<FlagRange>; 4]>,
}
impl<Label> __Boiled for NarrowDownInternal<Label>
where
    Label: __Boiled<Thawed = Label>
        + __Thawed<Frozen = Label>
        + __Frozen<Thawed = Label>
        + __Serialize,
{
    type Thawed = NarrowDownInternal<Label>;
    unsafe fn into_thawed(self) -> Self::Thawed {
        self
    }
}
impl<Label> __Thawed for NarrowDownInternal<Label>
where
    Label: __Thawed<Frozen = Label> + __Frozen<Thawed = Label> + __Serialize,
{
    type Frozen = NarrowDownInternal<Label>;
    unsafe fn freeze(&self) -> Self::Frozen {
        todo!()
    }

    fn serialize_to_value(&self) -> __JsonValue {
        __to_json_value(self).unwrap()
    }

    fn visualize_or_void(&self, visual_synchrotron: &mut __VisualSynchrotron) -> __Visual {
        // ad hoc
        __Visual::Void
    }

    fn is_copyable() -> bool {
        false
    }

    fn try_copy(&self) -> Option<__Value> {
        None
    }
}
impl<Label> __Frozen for NarrowDownInternal<Label>
where
    Label: __Frozen<Thawed = Label> + __Thawed<Frozen = Label> + __Serialize,
{
    type Thawed = NarrowDownInternal<<Label as __Frozen>::Thawed>;

    type Slush = ();

    fn thaw(&self) -> (Option<Self::Slush>, Self::Thawed) {
        todo!()
    }
}
impl<Label> __FromValue for NarrowDownInternal<Label>
where
    Label: __Boiled<Thawed = Label> + __Thawed,
{
    fn from_value_aux(value: __Value, _slush_values: Option<&mut __SlushValues>) -> Self {
        value.into_owned()
    }
}
impl<Label> __IntoValue for NarrowDownInternal<Label>
where
    Label: __Boiled<Thawed = Label>
        + __Thawed<Frozen = Label>
        + __Frozen<Thawed = Label>
        + __Serialize,
{
    fn into_value(self) -> __Value {
        __Value::from_owned(self)
    }
}

impl<Task, Label> __IsGnItem for narrow_down<Task, Label>
where
    Task: IsMlTask<__VarId>,
    Label: IsLabel,
{
    type LinketImpl = __LinketImpl;

    fn generic_pedestal(generic_pedestal: __Pedestal) -> __Pedestal {
        todo!()
        // __Pedestal::Generic
    }

    type ValueAtGenericPedestal = NarrowDownInternal<Label>;

    fn train(
        ki_domain_repr: __KiDomainReprInterface,
        ki_argument_reprs: &[__KiArgumentReprInterface],
    ) -> __KiControlFlow<Self::ValueAtGenericPedestal> {
        debug_assert_eq!(ki_argument_reprs.len(), 3);
        let __KiArgumentReprInterface::Variadic(ref features) = ki_argument_reprs[0] else {
            unreachable!()
        };
        let __KiArgumentReprInterface::Keyed(skip) = ki_argument_reprs[1] else {
            unreachable!()
        };
        let skip: i32 = match skip {
            Some(skip) => __eval_ki_repr_interface(skip, None)?,
            None => 5,
        };
        let __KiArgumentReprInterface::RuntimeConstants(ref runtime_constants) =
            ki_argument_reprs[2]
        else {
            unreachable!()
        };
        debug_assert_eq!(runtime_constants.len(), 1);
        let label: Label = __eval_val_runtime_constant(runtime_constants[0]);
        let fvf = FlagVectorField::from_features::<Task>(ki_domain_repr, features, label)?;
        let flag_ranges = fvf.flag_ranges(skip, 0.1);
        debug_assert_eq!(features.len(), flag_ranges.len());
        __KiControlFlow::Continue(NarrowDownInternal {
            label0: fvf.label0(),
            flag_ranges,
        })
    }

    type EvalOutput = OneVsAllResult;

    fn eval(
        ki_argument_reprs: &[__KiArgumentReprInterface],
        value_at_generic_pedestal: &Self::ValueAtGenericPedestal,
    ) -> OneVsAllResult {
        let __KiArgumentReprInterface::Variadic(ref features) = ki_argument_reprs[0] else {
            unreachable!()
        };
        debug_assert_eq!(features.len(), value_at_generic_pedestal.flag_ranges.len());
        for (&feature, flag_range) in
            std::iter::zip(features, &value_at_generic_pedestal.flag_ranges)
        {
            let Some(flag_range) = flag_range else {
                continue;
            };
            let v: f32 = match __eval_ki_repr_interface(feature, None) {
                __KiControlFlow::Continue(v) => v,
                __KiControlFlow::LoopContinue => todo!(),
                __KiControlFlow::LoopExit(_) => todo!(),
                __KiControlFlow::Return(_) => todo!(),
                __KiControlFlow::Undefined => todo!(),
                __KiControlFlow::Throw(_) => todo!(),
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
