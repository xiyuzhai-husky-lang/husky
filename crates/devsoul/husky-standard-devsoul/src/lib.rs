mod runtime_storage;

use self::runtime_storage::*;
use husky_devsoul::devsoul::{DevEvalContextLocalKey, IsDevsoul};
use husky_item_path_interface::ItemPathIdInterface;
use husky_ki_repr_interface::{KiDomainReprInterface, KiReprInterface};
use husky_linket_impl::dev_eval_context::IsDevRuntimeInterfaceDyn;
use husky_mono_linktime::MonoLinktime;
use husky_standard_linket_impl::pedestal::StandardPedestal;
use husky_standard_trace_protocol::{caryatid::StandardCaryatid, StandardTraceProtocol};
use husky_trace_protocol::{
    caryatid::IsCaryatid, figure::IsFigure, protocol::IsTraceProtocol, server::TraceVisualCache,
    trace_id::TraceId,
};
use husky_visual_protocol::synchrotron::VisualSynchrotron;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub struct StandardDevsoul;

type LinketImpl = husky_standard_linket_impl::StandardLinketImpl;

impl IsDevsoul for StandardDevsoul {
    type Pedestal = StandardPedestal;

    type LinketImpl = LinketImpl;

    type Linktime = MonoLinktime<LinketImpl>;

    type RuntimeStorage = StandardDevRuntimeStorage;

    type RuntimeSpecificConfig = ();

    type TraceProtocol = StandardTraceProtocol;
}
