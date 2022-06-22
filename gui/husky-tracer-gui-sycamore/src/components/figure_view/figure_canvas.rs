mod generic_f32;
mod generic_graphics2d;
mod generic_i32;
mod graphics2d;
mod plot2d;
mod primitive_value;

use super::*;
use generic_f32::*;
use generic_graphics2d::*;
use generic_i32::*;
use graphics2d::*;
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
    let attention = &tracer_context.attention_context.attention;
    let opt_data = memo!(scope, move || opt_active_trace_id.cget().map(
        |active_trace_id| {
            let active_trace = tracer_context.trace_context.trace(active_trace_id);
            tracer_context
                .figure_context
                .figure_canvas_data(&active_trace, &attention.get())
        }
    ));
    view! {
        scope,
        (if let Some(data) = opt_data.cget() {
            match *data {
                FigureCanvasData::Primitive { value } => {
                    view!{
                        scope,
                        PrimitiveValueCanvas {
                            value
                        }
                    }
                },
                FigureCanvasData::Plot2d {
                    plot_kind,
                    ref point_groups,
                    xrange,
                    yrange
                } => {
                    view!{
                        scope,
                        Plot2dCanvas {
                            dimension: props.dimension,
                            plot_kind,
                            point_groups: point_groups.clone(),
                            xrange,
                            yrange,
                        }
                    }
                },
                FigureCanvasData::Graphics2d {
                    ref graphics2d_data
                } => {
                    view!{
                        scope,
                        Graphics2dCanvas {
                            dimension: props.dimension,
                            image_layers: &graphics2d_data.image_layers,
                            shapes:&graphics2d_data.shapes,
                            xrange: graphics2d_data.xrange,
                            yrange: graphics2d_data.yrange,
                        }
                    }
                },
                FigureCanvasData::Mutations { .. } => todo!(),
                FigureCanvasData::GenericGraphics2d {
                    ref partitioned_samples,
                } =>
                view!{
                    scope,
                    GenericGraphics2d {
                        dimension: props.dimension,
                        partitioned_samples: partitioned_samples ,
                }},
                FigureCanvasData::GenericI32 {
                    ref partitioned_samples,
                } =>
                view!{
                    scope,
                    GenericI32 {
                        dimension: props.dimension,
                        partitioned_samples: partitioned_samples,
                }},
                _=> todo!(),
            }
        } else {
            view!{
                scope,
                "no active trace"
            }
        })
    }
}
