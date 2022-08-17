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
            dev_unreturneds,
            dev_nones,
            dev_trues,
            dev_falses,
        } => view! {
            scope,
            div (
                class="TraceStatsView",
                style=format!("padding-left: {}ch", 3 + props.indent),
            ) {
                div (class = "SampleStats") {
                    "S "
                    (*dev_samples)
                }
                div (class = "ArrivalStats") {
                    "A "
                    (*dev_arrivals)
                }
                div (class = "UnreturnedStats") {
                    "U "
                    (*dev_unreturneds)
                }
                div (class = "NoneStats") {
                    "N "
                    (*dev_nones)
                }
                div (class = "TrueStats") {
                    "T "
                    (*dev_trues)
                }
                div (class = "FalseStats") {
                    "F "
                    (*dev_falses)
                }
            }
        },
    }
}
