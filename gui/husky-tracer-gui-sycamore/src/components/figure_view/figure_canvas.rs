mod graphics2d;
mod plot2d;

use super::*;
use graphics2d::*;
use plot2d::*;

#[derive(Prop)]
pub struct FigureCanvasProps {
    data: FigureCanvasData,
}

#[component]
pub fn FigureCanvas<'a, G: Html>(scope: Scope<'a>, props: FigureCanvasProps) -> View<G> {
    match props.data {
        FigureCanvasData::Primitive { value } => todo!(),
        FigureCanvasData::Plot2d {
            plot_kind,
            point_groups,
            xrange,
            yrange,
        } => todo!(),
        FigureCanvasData::Graphics2d {
            image_layers,
            shapes,
            xrange,
            yrange,
        } => todo!(),
        FigureCanvasData::Mutations { mutations } => todo!(),
    }
}
