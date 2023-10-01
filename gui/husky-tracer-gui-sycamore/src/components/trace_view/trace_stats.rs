use super::*;
use husky_trace_protocol_old::TraceStats;
use web_sys::Event;

#[derive(Prop)]
pub struct TraceStatsProps<'a> {
    trace_id: TraceId,
    stats: &'a TraceStats,
    indent: Indent,
}

pub fn TraceStatsView<'a, G: Html>(visibility: Scope<'a>, props: TraceStatsProps<'a>) -> View<G> {
    let ctx = use_dev_context(visibility);
    let presentation_signal = ctx.presentation_signal();
    let trace_id = props.trace_id;
    let restrict_to_arrival = memo!(visibility, move || {
        let presentation = presentation_signal.get();
        if presentation.opt_active_trace_id() != Some(trace_id) {
            return false;
        }
        match presentation.restriction() {
            Some(restriction) => match restriction.kind {
                RestrictionKind::Arrival => true,
                RestrictionKind::Return => false,
            },
            None => true,
        }
    });
    let restrict_to_return = memo!(visibility, move || {
        let presentation = presentation_signal.get();
        if presentation.opt_active_trace_id() != Some(trace_id) {
            return false;
        }
        match presentation.restriction() {
            Some(restriction) => match restriction.kind {
                RestrictionKind::Arrival => false,
                RestrictionKind::Return => true,
            },
            None => false,
        }
    });
    match props.stats {
        TraceStats::Classification {
            dev_samples,
            dev_arrivals,
            dev_unreturneds,
            dev_nones,
            dev_trues,
            dev_falses,
            dev_partition_noness,
        } => view! {
            visibility,
            div (
                class="TraceStatsView",
                style=format!("padding-left: {}ch", 3 + props.indent),
            ) {
                div (class="TraceStatsViewLeft") {
                    div (
                        class = format!(
                            "ArrivalStats{}",
                            if restrict_to_arrival.cget() {
                                " active"
                            } else {
                                " inactive"
                            }
                        ),
                        on:click = ctx.restrict_presentation_to_arrival_handler(trace_id)
                    ) {
                        "A "
                        (*dev_arrivals)
                    }
                    div (
                        class = format!("ReturnStats{}",
                            if restrict_to_return.cget() {
                                " active"
                            } else {
                                " inactive"
                            }
                        ),
                        on:click = ctx.restrict_presentation_to_return_handler(trace_id)
                    ) {
                        "R "
                        (*dev_arrivals - *dev_unreturneds)
                    }
                }
                div (class="TraceStatsViewRight") {
                    div (class = "TruePositiveStats") {
                        "TP "
                        (*dev_trues)
                    }
                    div (class = "FalsePositiveStats") {
                        "FP "
                        (*dev_falses)
                    }
                    (View::new_fragment(dev_partition_noness.iter().enumerate().map(
                        |(idx, (partition, dev_partition_nones))| {
                            let partition_name = partition.name();
                            view!{
                                visibility,
                                div (class = "DevPartitionNoneStats") {
                                    "N["
                                    (partition_name)
                                    "] "
                                    (*dev_partition_nones)
                                }
                            }
                        }
                    ).collect()))
                }
            }
        },
    }
}
