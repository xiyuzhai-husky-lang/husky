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
pub struct FigureContentProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
}

#[component]
pub fn FigureContent<'a, G: Html>(scope: Scope<'a>, props: FigureContentProps<'a>) -> View<G> {
    let ctx = use_debugger_context(scope);
    let opt_active_trace_id = &ctx.trace_context.opt_active_trace_id;
    let restriction = &ctx.restriction_context.restriction;
    let opt_canvas_and_control_data = memo!(scope, move || opt_active_trace_id.cget().map(
        |active_trace_id| {
            let active_trace = ctx.trace_context.trace_data(active_trace_id);
            let canvas_value = ctx.figure_canvas_data(&active_trace, &restriction.get());
            let control_data = ctx.figure_control_data(&active_trace, &restriction.get());
            (canvas_value, control_data)
        }
    ));
    let pinned_canvas_values =
        create_static_memo(scope, move || ctx.collect_pinned_canvas_values());
    view! {
        scope,
        (if let Some((canvas_value, control_data)) = opt_canvas_and_control_data.cget() {
            view! {
                scope,
                FigureContentSwitch {
                    canvas_value,
                    pinned_canvas_values,
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
struct FigureContentSwitchProps<'a> {
    canvas_value: &'static FigureCanvasData,
    pinned_canvas_values: &'a ReadSignal<Vec<&'static FigureCanvasData>>,
    control_data: &'a Signal<FigureControlData>,
    dimension: &'a ReadSignal<PixelDimension>,
}

#[component]
fn FigureContentSwitch<'a, G: Html>(
    scope: Scope<'a>,
    props: FigureContentSwitchProps<'a>,
) -> View<G> {
    match props.canvas_value {
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
                    image_layers: graphics2d_data.total_image_layers(&props.pinned_canvas_values.get()),
                    shapes: graphics2d_data.total_shapes(&props.pinned_canvas_values.get()),
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
                        pinned_canvas_values: props.pinned_canvas_values,
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
                    partitioned_samples,
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
                    partitioned_samples,
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
                    pinned_canvas_values: props.pinned_canvas_values,
                    partitioned_samples,
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
