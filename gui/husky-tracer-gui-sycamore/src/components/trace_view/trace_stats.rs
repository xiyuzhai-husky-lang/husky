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
            samples,
            arrivals,
            nulls,
            trues,
            falses,
        } => view! {
            scope,
            div (
                class="TraceStatsView",
                style=format!("padding-left: {}ch", 3 + props.indent),
            ) {
                "samples = "
                (*samples)
                ", arrivals = "
                (*arrivals)
                ", nulls = "
                (*nulls)
                ", trues = "
                (*trues)
                ", falses = "
                (*falses)
            }
        },
    }
}
