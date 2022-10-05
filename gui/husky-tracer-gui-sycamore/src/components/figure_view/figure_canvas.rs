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
    value: FigureCanvasValue,
}

#[component]
pub fn FigureCanvas<'a, G: Html>(scope: Scope<'a>, props: FigureCanvasProps<'a>) -> View<G> {
    match props.value {
        FigureCanvasValue::Primitive { value } => {
            view! {
                scope,
                PrimitiveValueCanvas {
                    value
                }
            }
        }
        // FigureCanvasValue::Plot2d {
        //     plot_kind,
        //     ref point_groups,
        //     xrange,
        //     yrange,
        // } => {
        //     view! {
        //         scope,
        //         Plot2dCanvas {
        //             dimension: props.dimension,
        //             plot_kind: *plot_kind,
        //             point_groups: point_groups.clone(),
        //             xrange: *xrange,
        //             yrange: *yrange,
        //         }
        //     }
        // }
        FigureCanvasValue::Graphics2d {
            ref graphics2d_data,
        } => {
            view! {
                scope,
                Graphics2dCanvas {
                    dimension: props.dimension,
                    image_layers: graphics2d_data.image_layers(),
                    shapes: graphics2d_data.shapes(),
                    xrange: graphics2d_data.xrange,
                    yrange: graphics2d_data.yrange,
                }
            }
        }
        // FigureCanvasValue::Mutations { ref mutations } => {
        //     if let Some(mutation_selection) = props.control_data.get().opt_mutation_selection {
        //         view! {
        //             scope,
        //             MutationCanvas {
        //                 dimension: props.dimension,
        //                 control_data: props.control_data,
        //                 mutation: &mutations[mutation_selection as usize]
        //             }
        //         }
        //     } else {
        //         view! {scope, }
        //     }
        // }
        FigureCanvasValue::GenericGraphics2d {
            partitioned_samples,
            ..
        } => {
            view! {
                scope,
                GenericGraphics2d {
                    dimension: props.dimension,
                    partitioned_samples,
                }
            }
        }
        FigureCanvasValue::GenericI32 {
            partitioned_samples,
            ..
        } => {
            view! {
                scope,
                GenericI32 {
                    dimension: props.dimension,
                    partitioned_samples,
                }
            }
        }
        FigureCanvasValue::GenericF32 {
            ref partitioned_samples,
        } => {
            view! {
                scope,
                GenericF32 {
                    dimension: props.dimension,
                    partitioned_samples,
                    image_layers: todo!(),
                    shapes: todo!(),
                }
            }
        } // FigureCanvasValue::EvalError { ref message } => {
          //     view! {
          //         scope,
          //         div (class="EvalErrorCanvas") {
          //             (message.clone())
          //         }
          //     }
          // }
    }
}
