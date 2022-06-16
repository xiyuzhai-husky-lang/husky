mod graphics2d;
mod plot2d;
mod primitive_value;

use super::*;
use graphics2d::*;
use plot2d::*;
use primitive_value::*;

#[derive(Prop)]
pub struct FigureCanvasProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
}

#[component]
pub fn FigureCanvas<'a, G: Html>(scope: Scope<'a>, props: FigureCanvasProps<'a>) -> View<G> {
    let tracer_context = use_context::<TracerContext>(scope);
    let opt_active_trace_id = &tracer_context.tree_context.opt_active_trace_id;
    let focus = &tracer_context.focus_context.focus;
    view! {
        scope,
        (if let Some(active_trace_id) = opt_active_trace_id.get_cloned() {
            let active_trace = tracer_context.tree_context.trace(active_trace_id);
            let data = memo!(
                scope,
                tracer_context
                    .figure_context
                    .figure_canvas_data(&active_trace, &focus.get())
            );
            match *data.get_cloned() {
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
                            image_layers: Rc::new(graphics2d_data.image_layers.clone()),
                            shapes: Rc::new(graphics2d_data.shapes.clone()),
                            xrange: graphics2d_data.xrange,
                            yrange: graphics2d_data.yrange,
                        }
                    }
                },
                FigureCanvasData::Mutations { .. } => todo!(),
                FigureCanvasData::GenericGraphics2d { ref partitioned_samples, ref others} => todo!(),
            }
        } else {
            view!{
                scope,
                "no active trace"
            }
        })
    }
}
