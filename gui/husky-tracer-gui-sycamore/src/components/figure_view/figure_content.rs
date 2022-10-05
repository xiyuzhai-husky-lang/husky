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
    let ctx = use_dev_context(scope);
    let presentation_signal = ctx.presentation_signal();
    let opt_canvas_and_control_data = memo!(scope, move || {
        let presentation = &presentation_signal.get();
        presentation.opt_active_trace_id().map(|active_trace_id| {
            let active_trace = ctx.trace_data(active_trace_id);
            let canvas_value = ctx.figure_canvas_data(active_trace, presentation);
            let control_data = ctx.figure_control_data(active_trace, presentation);
            (canvas_value, control_data)
        })
    });
    view! {
        scope,
        (if let Some((canvas_value, control_data)) = opt_canvas_and_control_data.cget() {
            view! {
                scope,
                FigureContentSwitch {
                    canvas_value,
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
    control_data: &'a Signal<FigureControlData>,
    dimension: &'a ReadSignal<PixelDimension>,
}

#[component]
fn FigureContentSwitch<'a, G: Html>(
    scope: Scope<'a>,
    props: FigureContentSwitchProps<'a>,
) -> View<G> {
    let ctx = use_dev_context(scope);
    let pinned_canvas_values = memo!(scope, move || ctx.collect_pinned_canvas_values());
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
            let image_layers = memo!(scope, move || (graphics2d_data, pinned_canvas_values.get())
                .image_layers());
            let shapes = memo!(scope, move || (graphics2d_data, pinned_canvas_values.get())
                .shapes());
            view! {
                scope,
                Graphics2dCanvas {
                    dimension: props.dimension,
                    image_layers,
                    shapes,
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
