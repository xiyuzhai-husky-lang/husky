use std::iter::zip;

use crate::*;
use husky_signal::Signalable;
use husky_vm_primitive_value::PrimitiveValueData;

#[derive(Debug, Clone, PartialEq)]
pub enum FigureCanvasValue {
    Primitive {
        value: PrimitiveValueData,
    },
    GenericF32 {
        partitioned_samples: &'static [(Partition, Vec<(SampleId, f32)>)],
    },
    GenericI32 {
        partitioned_samples: &'static [(Partition, Vec<(SampleId, i32)>)],
    },
    GenericGraphics2d {
        partitioned_samples: Vec<(&'static Partition, Vec<(SampleId, Graphics2dCanvasValue)>)>,
    },
}

impl Signalable for FigureCanvasValue {}

impl FigureCanvasValue {
    fn void() -> Self {
        FigureCanvasValue::Primitive { value: ().into() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Graphics2dCanvasValue {
    pub(crate) image_layers: Vec<&'static ImageLayerData>,
    pub(crate) shapes: Vec<&'static Shape2dData>,
    pub xrange: (f32, f32),
    pub yrange: (f32, f32),
}

impl Graphics2dCanvasValue {
    fn add(&mut self, other: &Self) {
        if self.xrange != other.xrange {
            todo!()
        }
        if self.yrange != other.yrange {
            todo!()
        }
        self.image_layers
            .extend(other.image_layers.iter().map(|r| *r));
        self.shapes.extend(other.shapes.iter().map(|r| *r))
    }
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
        presentation_kind: PresentationKind,
        opt_active_figure_not_pinned: Option<FigureCanvasDataItd>,
        pinned_figures: Vec<FigureCanvasDataItd>,
    ) -> Self {
        let mut all_figures = pinned_figures;
        if let Some(active_figure) = opt_active_figure_not_pinned {
            all_figures.insert(0, active_figure)
        }
        if all_figures.len() == 0 {
            return Self::void();
        }
        let mut value = Self::new_piece(presentation_kind, &all_figures[0]);
        for other in all_figures[1..].iter() {
            value.merge(Self::new_piece(presentation_kind, other))
        }
        value
    }

    fn new_piece(presentation_kind: PresentationKind, data_itd: &FigureCanvasDataItd) -> Self {
        match presentation_kind {
            PresentationKind::Generic => Self::new_generic_piece(data_itd),
            PresentationKind::Specific => todo!(),
            PresentationKind::Panic => todo!(),
        }
    }

    fn new_generic_piece(data_itd: &FigureCanvasDataItd) -> Self {
        match data_itd.generic {
            FigureCanvasData::Primitive { value } => FigureCanvasValue::Primitive { value: *value },
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
            } => FigureCanvasValue::GenericGraphics2d {
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

    fn merge(&mut self, other: FigureCanvasValue) {
        match self {
            FigureCanvasValue::Primitive { .. } => *self = other,
            FigureCanvasValue::GenericF32 {
                partitioned_samples,
            } => todo!(),
            FigureCanvasValue::GenericI32 {
                partitioned_samples,
            } => todo!(),
            FigureCanvasValue::GenericGraphics2d {
                partitioned_samples: partitioned_samples0,
            } => match other {
                FigureCanvasValue::Primitive { .. } => (),
                FigureCanvasValue::GenericF32 { .. } => (),
                FigureCanvasValue::GenericI32 { .. } => (),
                FigureCanvasValue::GenericGraphics2d {
                    partitioned_samples: partitioned_samples1,
                } => {
                    assert_eq!(partitioned_samples0.len(), partitioned_samples1.len());
                    for ((partition0, samples0), (partition1, samples1)) in
                        zip(partitioned_samples0.iter_mut(), partitioned_samples1.iter())
                    {
                        assert_eq!(partition0, partition1);
                        assert_eq!(samples0.len(), samples1.len());
                        for (
                            (sample_id0, sample_figure_canvas_value0),
                            (sample_id1, sample_figure_canvas_value1),
                        ) in zip(samples0.iter_mut(), samples1.iter())
                        {
                            assert_eq!(sample_id0, sample_id1);
                            sample_figure_canvas_value0.add(sample_figure_canvas_value1)
                        }
                    }
                    todo!()
                }
            },
        }
    }
}
