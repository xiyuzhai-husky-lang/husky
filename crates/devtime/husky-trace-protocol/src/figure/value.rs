use crate::*;
use husky_signal::Signalable;
use husky_vm_primitive_value::PrimitiveValueData;

#[derive(Debug, Clone, PartialEq)]
pub enum FigureCanvasValue {
    Void,
    Float {
        partitioned_samples: &'static [(Partition, Vec<(SampleId, f32)>)],
    },
    Integer {
        partitioned_samples: &'static [(Partition, Vec<(SampleId, i32)>)],
    },
    Graphics2d {
        partitioned_samples: Vec<(&'static Partition, Vec<(SampleId, Graphics2dCanvasValue)>)>,
        particular: Graphics2dCanvasValue,
    },
}

impl Signalable for FigureCanvasValue {}

impl Default for FigureCanvasValue {
    fn default() -> Self {
        FigureCanvasValue::Void
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Graphics2dCanvasValue {
    pub(crate) image_layers: Vec<&'static ImageLayerData>,
    pub(crate) shapes: Vec<&'static Shape2dData>,
    pub xrange: (f32, f32),
    pub yrange: (f32, f32),
}

impl ContainsShapes<'static> for Graphics2dCanvasValue {
    fn shapes(&self) -> Vec<&'static Shape2dData> {
        self.shapes.clone()
    }
}

impl ContainsImageLayers<'static> for Graphics2dCanvasValue {
    fn image_layers(&self) -> Vec<&'static ImageLayerData> {
        self.image_layers.clone()
    }
}

impl Graphics2dCanvasValue {
    pub fn new(data: &'static Graphics2dCanvasData) -> Self {
        Graphics2dCanvasValue {
            image_layers: data.image_layers.iter().collect(),
            shapes: data.shapes.iter().collect(),
            xrange: data.xrange,
            yrange: data.yrange,
        }
    }
}

impl FigureCanvasValue {
    pub fn new(
        opt_active_figure: Option<FigureCanvasDataItd>,
        pinned_figures: Vec<FigureCanvasDataItd>,
    ) -> Self {
        let mut all_figures = pinned_figures;
        if let Some(active_figure) = opt_active_figure {
            all_figures.insert(0, active_figure)
        }
        if all_figures.len() == 0 {
            return Default::default();
        }
        let mut value = Self::new_piece(&all_figures[0]);
        for other in all_figures[1..].iter() {
            value.add(Self::new_piece(other))
        }
        value
    }

    fn new_piece(data_itd: &FigureCanvasDataItd) -> Self {
        match data_itd.generic {
            FigureCanvasData::Primitive { value } => todo!(),
            FigureCanvasData::Plot2d {
                plot_kind,
                point_groups,
                xrange,
                yrange,
            } => todo!(),
            FigureCanvasData::Graphics2d { graphics2d_data } => todo!(),
            FigureCanvasData::Mutations { mutations } => todo!(),
            FigureCanvasData::GenericGraphics2d {
                partitioned_samples,
            } => FigureCanvasValue::Graphics2d {
                partitioned_samples: partitioned_samples
                    .iter()
                    .map(|(partition, samples)| {
                        (
                            partition,
                            samples
                                .iter()
                                .map(|(sample_id, data)| {
                                    (*sample_id, Graphics2dCanvasValue::new(data))
                                })
                                .collect(),
                        )
                    })
                    .collect(),
                particular: match data_itd.specific {
                    FigureCanvasData::Graphics2d { graphics2d_data } => {
                        Graphics2dCanvasValue::new(graphics2d_data)
                    }
                    _ => unreachable!(),
                },
            },
            FigureCanvasData::GenericF32 {
                partitioned_samples,
            } => todo!(),
            FigureCanvasData::GenericI32 {
                partitioned_samples,
            } => todo!(),
            FigureCanvasData::EvalError { message } => todo!(),
        }
    }

    fn add(&mut self, other: FigureCanvasValue) {
        match self {
            FigureCanvasValue::Void => todo!(),
            FigureCanvasValue::Float {
                partitioned_samples,
            } => todo!(),
            FigureCanvasValue::Integer {
                partitioned_samples,
            } => todo!(),
            FigureCanvasValue::Graphics2d {
                partitioned_samples,
                particular,
            } => todo!(),
        }
    }
}
