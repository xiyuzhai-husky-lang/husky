mod generic_f32;
mod generic_graphics2d;
mod generic_i32;
mod graphics2d;
mod mutation;
mod plot2d;
mod primitive_value;

use super::*;
use generic_f32::*;
use generic_graphics2d::*;
use generic_i32::*;
use graphics2d::*;
use mutation::*;
use plot2d::*;
use primitive_value::*;

#[derive(Prop)]
pub struct FigureCanvasProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
}

#[component]
pub fn FigureCanvas<'a, G: Html>(scope: Scope<'a>, props: FigureCanvasProps<'a>) -> View<G> {
    let tracer_context = use_context::<DebuggerContext>(scope);
    let opt_active_trace_id = &tracer_context.trace_context.opt_active_trace_id;
    let restriction = &tracer_context.restriction_context.restriction;
    let opt_canvas_and_control_data = memo!(scope, move || opt_active_trace_id.cget().map(
        |active_trace_id| {
            let active_trace = tracer_context.trace_context.trace_data(active_trace_id);
            let canvas_data = tracer_context
                .figure_context
                .figure_canvas_data(&active_trace, &restriction.get());
            let control_data = tracer_context
                .figure_context
                .figure_control_data(&active_trace, &restriction.get());
            (canvas_data, control_data)
        }
    ));
    view! {
        scope,
        (if let Some((canvas_data, control_data)) = opt_canvas_and_control_data.cget() {
            view! {
                scope,
                FigureCanvasSwitch {
                    canvas_data,
                    control_data,
                    dimension: props.dimension
                }
            }
        } else {
            view!{
                scope,
                "no active trace"
            }
        })
    }
}

#[derive(Prop)]
struct FigureCanvasSwitchProps<'a> {
    canvas_data: &'a FigureCanvasData,
    control_data: &'a Signal<FigureControlData>,
    dimension: &'a ReadSignal<PixelDimension>,
}

#[component]
fn FigureCanvasSwitch<'a, G: Html>(
    scope: Scope<'a>,
    props: FigureCanvasSwitchProps<'a>,
) -> View<G> {
    match props.canvas_data {
        FigureCanvasData::Primitive { value } => {
            view! {
                scope,
                PrimitiveValueCanvas {
                    value: *value
                }
            }
        }
        FigureCanvasData::Plot2d {
            plot_kind,
            ref point_groups,
            xrange,
            yrange,
        } => {
            view! {
                scope,
                Plot2dCanvas {
                    dimension: props.dimension,
                    plot_kind: *plot_kind,
                    point_groups: point_groups.clone(),
                    xrange: *xrange,
                    yrange: *yrange,
                }
            }
        }
        FigureCanvasData::Graphics2d {
            ref graphics2d_data,
        } => {
            view! {
                scope,
                Graphics2dCanvas {
                    dimension: props.dimension,
                    image_layers: &graphics2d_data.image_layers,
                    shapes: &graphics2d_data.shapes,
                    xrange: graphics2d_data.xrange,
                    yrange: graphics2d_data.yrange,
                }
            }
        }
        FigureCanvasData::Mutations { ref mutations } => {
            if let Some(mutation_selection) = props.control_data.get().opt_mutation_selection {
                view! {
                    scope,
                    MutationCanvas {
                        dimension: props.dimension,
                        control_data: props.control_data,
                        mutation: &mutations[mutation_selection as usize]
                    }
                }
            } else {
                view! {scope, }
            }
        }
        FigureCanvasData::GenericGraphics2d {
            ref partitioned_samples,
        } => {
            view! {
                scope,
                GenericGraphics2d {
                    dimension: props.dimension,
                    partitioned_samples: partitioned_samples ,
                }
            }
        }
        FigureCanvasData::GenericI32 {
            ref partitioned_samples,
        } => {
            view! {
                scope,
                GenericI32 {
                    dimension: props.dimension,
                    partitioned_samples: partitioned_samples,
                }
            }
        }
        FigureCanvasData::GenericF32 {
            ref partitioned_samples,
        } => {
            view! {
                scope,
                GenericF32 {
                    dimension: props.dimension,
                    partitioned_samples: partitioned_samples,
                }
            }
        }
        FigureCanvasData::EvalError { ref message } => {
            view! {
                scope,
                div (class="EvalErrorCanvas") {
                    (message.clone())
                }
            }
        }
    }
}
