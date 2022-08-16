use super::*;
use husky_trace_protocol::TraceStats;
use web_sys::Event;

#[derive(Prop)]
pub struct TraceStatsProps<'a> {
    stats: &'a TraceStats,
    indent: Indent,
}

pub fn TraceStatsView<'a, G: Html>(scope: Scope<'a>, props: TraceStatsProps<'a>) -> View<G> {
    match props.stats {
        TraceStats::Classification {
            dev_samples,
            dev_arrivals,
            dev_undefineds,
            dev_unreturneds,
            dev_trues,
            dev_falses,
        } => view! {
            scope,
            div (
                class="TraceStatsView",
                style=format!("padding-left: {}ch", 3 + props.indent),
            ) {
                span (class = "Division") {
                    "dev:"
                }
                " "
                span (class = "SampleStats") {
                    "samples = "
                    (*dev_samples)
                }
                ", "
                span (class = "ArrivalStats") {
                    "arrivals = "
                    (*dev_arrivals)
                }
                ", "
                span (class = "UndefinedStats") {
                    "undefineds = "
                    (*dev_undefineds)
                }
                ", "
                span (class = "UnreturnedStats") {
                    "unreturneds = "
                    (*dev_unreturneds)
                }
                ", "
                span (class = "TrueStats") {
                    "trues = "
                    (*dev_trues)
                }
                ", "
                span (class = "FalseStats") {
                    "falses = "
                    (*dev_falses)
                }
                ", val: todo!()"
            }
        },
    }
}
