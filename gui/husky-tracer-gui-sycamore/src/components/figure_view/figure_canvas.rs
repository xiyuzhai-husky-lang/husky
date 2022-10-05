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
    value: &'a ReadSignal<FigureCanvasValue>,
    presentation_kind: &'a ReadSignal<PresentationKind>,
}

#[component]
pub fn FigureCanvas<'a, G: Html>(scope: Scope<'a>, props: FigureCanvasProps<'a>) -> View<G> {
    view! {
        scope,
        (match *props.value.get() {
            FigureCanvasValue::Void => {
                view! {
                    scope,
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
                ref partitioned_samples,
                ref particular,
            } => {
                match props.presentation_kind.cget() {
                    PresentationKind::Generic => {
                        todo!()
                    }
                    PresentationKind::Specific => {
                        todo!()
                    }
                    PresentationKind::Panic => {
                        todo!()
                    }
                }
                // view! {
                //     scope,
                //     Graphics2dCanvas {
                //         dimension: props.dimension,
                //         image_layers: graphics2d_data.image_layers(),
                //         shapes: graphics2d_data.shapes(),
                //         xrange: graphics2d_data.xrange,
                //         yrange: graphics2d_data.yrange,
                //     }
                // }
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
            FigureCanvasValue::Graphics2d {
                ref partitioned_samples,
                ..
            } => {
                view! {
                    scope,
                    GenericGraphics2d {
                        dimension: props.dimension,
                        partitioned_samples: partitioned_samples.clone(),
                    }
                }
            }
            FigureCanvasValue::Integer {
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
            FigureCanvasValue::Float {
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
            }
            // FigureCanvasValue::EvalError { ref message } => {
            //     view! {
            //         scope,
            //         div (class="EvalErrorCanvas") {
            //             (message.clone())
            //         }
            //     }
            // }
        })
    }
}
