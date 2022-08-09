use crate::*;
use husky_compile_time::*;
use husky_trace_protocol::TraceStats;
use vm::{__RegisterDataKind, __VMResult, __VirtualEnum, __VIRTUAL_ENUM_VTABLE};

impl<'eval> TraceVariant<'eval> {
    pub fn opt_stats(&self, runtime: &dyn EvalFeature) -> __VMResult<Option<TraceStats>> {
        match self {
            TraceVariant::Main(repr) => feature_repr_opt_stats(runtime, repr, None),
            TraceVariant::Module { route, file, range } => Ok(None),
            TraceVariant::EntityFeature { repr, .. } => feature_repr_opt_stats(runtime, repr, None),
            TraceVariant::FeatureStmt(_) => todo!(),
            TraceVariant::FeatureBranch(_) => todo!(),
            TraceVariant::FeatureExpr(_) => todo!(),
            TraceVariant::FeatureCallArgument { name, argument } => todo!(),
            TraceVariant::FuncStmt { stmt, history } => todo!(),
            TraceVariant::ProcStmt { stmt, history } => todo!(),
            TraceVariant::ProcBranch {
                stmt,
                branch,
                opt_vm_branch,
                branch_idx,
                history,
            } => todo!(),
            TraceVariant::FuncBranch {
                stmt,
                branch,
                opt_vm_branch,
                branch_idx,
                history,
            } => todo!(),
            TraceVariant::LoopFrame {
                loop_stmt,
                body_instruction_sheet,
                body_stmts,
                loop_frame_data,
            } => todo!(),
            TraceVariant::EagerExpr { expr, history } => todo!(),
            TraceVariant::EagerCallArgument {
                name,
                argument,
                history,
            } => todo!(),
            TraceVariant::CallHead { entity, tokens } => todo!(),
        }
    }
}

const MAX_SAMPING_SIZE: usize = 1000;

fn feature_repr_opt_stats(
    db: &dyn EvalFeature,
    repr: &FeatureRepr,
    opt_arrival_indicator: Option<&Arc<FeatureArrivalIndicator>>,
) -> __VMResult<Option<TraceStats>> {
    let comptime = db.comptime();
    let target_output_ty = comptime.target_output_ty().unwrap();
    // todo check this could cause some problem
    if !comptime.is_implicitly_castable(repr.ty(), target_output_ty) {
        return Ok(None);
    }
    let mut samples = 0;
    let mut arrivals = 0;
    let mut nulls = 0;
    let mut trues = 0;
    let mut falses = 0;
    for labeled_data in db.session().dev().each_labeled_data() {
        samples += 1;
        let sample_id = labeled_data.sample_id;
        if !db
            .eval_opt_arrival_indicator(opt_arrival_indicator, sample_id)
            .map_err(|_| todo!())?
        {
            continue;
        }
        arrivals += 1;
        let value = db
            .eval_feature_repr_cached(repr, sample_id)
            .map_err(|_| todo!())?;
        match value.data_kind() {
            __RegisterDataKind::PrimitiveValue => todo!(),
            __RegisterDataKind::Box => todo!(),
            __RegisterDataKind::EvalRef => {
                let prediction = Label(
                    value
                        .downcast_temp_ref::<__VirtualEnum>(&__VIRTUAL_ENUM_VTABLE)
                        .kind_idx,
                );
                match prediction == labeled_data.label {
                    true => trues += 1,
                    false => falses += 1,
                }
            }
            __RegisterDataKind::TempRef => todo!(),
            __RegisterDataKind::TempMut => todo!(),
            __RegisterDataKind::Moved => todo!(),
            __RegisterDataKind::Undefined => nulls += 1,
            __RegisterDataKind::Unreturned => todo!(),
        }
    }
    Ok(Some(TraceStats::Classification {
        samples,
        arrivals,
        nulls,
        trues,
        falses,
    }))
}
