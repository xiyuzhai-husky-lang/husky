use std::iter::zip;

use crate::*;
use husky_vm_primitive_value::PrimitiveValueData;

#[derive(Debug, Clone, PartialEq)]
pub enum FigureCanvasValue<'a> {
    Unit,
    NonUnitPrimitive {
        data: PrimitiveValueData,
    },
    Graphics2d {
        value: Graphics2dCanvasValue<'a>,
    },
    GenericF32 {
        partitioned_samples: &'a [(Partition, Vec<(SampleId, f32)>)],
        image_layers: Vec<&'a ImageLayerData>,
        shapes: Vec<&'a Shape2dData>,
    },
    GenericI32 {
        partitioned_samples: &'a [(Partition, Vec<(SampleId, i32)>)],
        image_layers: Vec<&'a ImageLayerData>,
        shapes: Vec<&'a Shape2dData>,
    },
    GenericGraphics2d {
        partitioned_samples: Vec<(&'a Partition, Vec<(SampleId, Graphics2dCanvasValue<'a>)>)>,
        specific: Graphics2dCanvasValue<'a>,
    },
}

impl<'a> Default for FigureCanvasValue<'a> {
    fn default() -> Self {
        FigureCanvasValue::Unit
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Graphics2dCanvasValue<'a> {
    pub(crate) image_layers: Vec<&'a ImageLayerData>,
    pub(crate) shapes: Vec<&'a Shape2dData>,
    pub xrange: (f32, f32),
    pub yrange: (f32, f32),
}

impl<'a> Graphics2dCanvasValue<'a> {
    fn add(&mut self, other: Self) {
        if self.xrange != other.xrange {
            todo!()
        }
        if self.yrange != other.yrange {
            todo!()
        }
        self.image_layers.extend(other.image_layers.into_iter());
        self.shapes.extend(other.shapes.into_iter())
    }
}

impl<'a> ContainsShapes<'a> for Graphics2dCanvasValue<'a> {
    fn shapes(&self) -> Vec<&'a Shape2dData> {
        self.shapes.clone()
    }
}

impl<'a> ContainsImageLayers<'a> for Graphics2dCanvasValue<'a> {
    fn image_layers(&self) -> Vec<&'a ImageLayerData> {
        self.image_layers.clone()
    }
}

impl<'a> Graphics2dCanvasValue<'a> {
    pub fn new(data: &'a Graphics2dCanvasData) -> Self {
        Graphics2dCanvasValue {
            image_layers: data.image_layers.iter().collect(),
            shapes: data.shapes.iter().collect(),
            xrange: data.xrange,
            yrange: data.yrange,
        }
    }
}

impl<'a> FigureCanvasValue<'a> {
    pub fn new<C: AsRef<FigureControlData>>(
        presentation_kind: PresentationKind,
        opt_active_figure_not_pinned: Option<FigureCanvasData<'a>>,
        pinned_figures: Vec<FigureCanvasData<'a>>,
        opt_control: Option<&C>,
    ) -> Self {
        let mut all_figures = pinned_figures;
        if let Some(active_figure) = opt_active_figure_not_pinned {
            all_figures.insert(0, active_figure)
        }
        if all_figures.len() == 0 {
            return Default::default();
        }
        let mut value = Self::new_piece(presentation_kind, all_figures[0], opt_control);
        for other in all_figures[1..].iter().copied() {
            let new_piece = Self::new_piece(presentation_kind, other, opt_control);
            value.merge(new_piece)
        }
        value
    }

    fn new_piece<C: AsRef<FigureControlData>>(
        presentation_kind: PresentationKind,
        data_itd: FigureCanvasData<'a>,
        opt_control: Option<&C>,
    ) -> Self {
        match presentation_kind {
            PresentationKind::Generic => Self::new_generic_piece(data_itd),
            PresentationKind::Specific => Self::new_specific_piece(data_itd, opt_control),
            PresentationKind::Panic => todo!(),
        }
    }

    fn new_generic_piece(data_itd: FigureCanvasData<'a>) -> Self {
        match data_itd.generic {
            GenericFigureCanvasData::Unit => FigureCanvasValue::Unit,
            GenericFigureCanvasData::Plot2d {
                plot_kind: _,
                point_groups: _,
                xrange: _,
                yrange: _,
            } => todo!(),
            GenericFigureCanvasData::Graphics2d { graphics2d_data: _ } => todo!(),
            GenericFigureCanvasData::GenericGraphics2d {
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
                specific: match data_itd.specific {
                    SpecificFigureCanvasData::Atom(atom) => match atom {
                        FigureCanvasAtom::Primitive(data) => match data {
                            PrimitiveValueData::I32(_) => todo!(),
                            PrimitiveValueData::I64(_) => todo!(),
                            PrimitiveValueData::F32(_) => todo!(),
                            PrimitiveValueData::B32(_) => todo!(),
                            PrimitiveValueData::B64(_) => todo!(),
                            PrimitiveValueData::Bool(_) => todo!(),
                            PrimitiveValueData::Unit => Graphics2dCanvasValue {
                                image_layers: vec![],
                                shapes: vec![],
                                xrange: (0., 28.), // ad hoc
                                yrange: (0., 28.), // ad hoc
                            },
                        },
                        FigureCanvasAtom::Graphics2d(data) => Graphics2dCanvasValue::new(data),
                    },
                    _ => unreachable!(),
                },
            },
            GenericFigureCanvasData::GenericF32 {
                partitioned_samples,
            } => FigureCanvasValue::GenericF32 {
                partitioned_samples,
                image_layers: vec![],
                shapes: vec![],
            },
            GenericFigureCanvasData::GenericI32 {
                partitioned_samples,
            } => FigureCanvasValue::GenericI32 {
                partitioned_samples,
                image_layers: vec![],
                shapes: vec![],
            },
            GenericFigureCanvasData::EvalError { message: _ } => todo!(),
        }
    }

    fn new_specific_piece<C: AsRef<FigureControlData>>(
        data: FigureCanvasData<'a>,
        opt_control: Option<&C>,
    ) -> Self {
        match data.specific {
            SpecificFigureCanvasData::Unit => FigureCanvasValue::Unit,
            SpecificFigureCanvasData::Atom(atom) => Self::new_specific_opt_atom_piece(Some(atom)),
            SpecificFigureCanvasData::Mutations { mutations } => {
                match opt_control.unwrap().as_ref() {
                    FigureControlData::Unit => unreachable!(),
                    FigureControlData::Mutations {
                        opt_mutation_selection,
                    } => {
                        if let Some(mutation_selection) = opt_mutation_selection {
                            Self::new_specific_opt_atom_piece(
                                mutations[*mutation_selection as usize].after.as_ref(),
                            )
                        } else {
                            Default::default()
                        }
                    }
                }
            }
            SpecificFigureCanvasData::EvalError { message: _ } => todo!(),
        }
    }

    fn new_specific_opt_atom_piece(opt_atom: Option<&'a FigureCanvasAtom>) -> Self {
        if let Some(atom) = opt_atom {
            match atom {
                FigureCanvasAtom::Primitive(data) => match data {
                    PrimitiveValueData::Unit => Default::default(),
                    _ => FigureCanvasValue::NonUnitPrimitive { data: *data },
                },
                FigureCanvasAtom::Graphics2d(graphics2d_data) => FigureCanvasValue::Graphics2d {
                    value: Graphics2dCanvasValue::new(graphics2d_data),
                },
            }
        } else {
            Default::default()
        }
    }

    fn merge(&mut self, other: FigureCanvasValue<'a>) {
        match self {
            FigureCanvasValue::Unit => *self = other,
            FigureCanvasValue::NonUnitPrimitive { data: _ } => (), // ad hoc
            FigureCanvasValue::GenericF32 {
                partitioned_samples: _,
                image_layers,
                shapes,
            } => match other {
                FigureCanvasValue::Unit => (),
                FigureCanvasValue::NonUnitPrimitive { data: _ } => todo!(),
                FigureCanvasValue::GenericF32 {
                    partitioned_samples: _,
                    image_layers: _,
                    shapes: _,
                } => todo!(),
                FigureCanvasValue::GenericI32 {
                    partitioned_samples: _,
                    image_layers: _,
                    shapes: _,
                } => todo!(),
                FigureCanvasValue::GenericGraphics2d { specific, .. } => {
                    image_layers.extend(specific.image_layers().into_iter());
                    shapes.extend(specific.shapes().into_iter())
                }
                FigureCanvasValue::Graphics2d { value: _ } => todo!(),
            },
            FigureCanvasValue::GenericI32 {
                partitioned_samples: _,
                image_layers: _,
                shapes: _,
            } => todo!(),
            FigureCanvasValue::GenericGraphics2d {
                partitioned_samples: partitioned_samples0,
                specific: particular0,
            } => match other {
                FigureCanvasValue::Unit => (),
                FigureCanvasValue::NonUnitPrimitive { data: _ } => todo!(),
                FigureCanvasValue::GenericF32 { .. } => (),
                FigureCanvasValue::GenericI32 { .. } => (),
                FigureCanvasValue::GenericGraphics2d {
                    partitioned_samples: partitioned_samples1,
                    specific: particular1,
                } => {
                    assert_eq!(partitioned_samples0.len(), partitioned_samples1.len());
                    for ((partition0, samples0), (partition1, samples1)) in zip(
                        partitioned_samples0.iter_mut(),
                        partitioned_samples1.into_iter(),
                    ) {
                        assert_eq!(partition0, &partition1);
                        assert_eq!(samples0.len(), samples1.len());
                        for (
                            (sample_id0, sample_canvas_value0),
                            (sample_id1, sample_canvas_value1),
                        ) in zip(samples0.iter_mut(), samples1.into_iter())
                        {
                            assert_eq!(*sample_id0, sample_id1);
                            sample_canvas_value0.add(sample_canvas_value1)
                        }
                    }
                    particular0.add(particular1)
                }
                FigureCanvasValue::Graphics2d { value: _ } => todo!(),
            },
            FigureCanvasValue::Graphics2d { value: value0 } => match other {
                FigureCanvasValue::Unit => (),
                FigureCanvasValue::NonUnitPrimitive { data: _ } => todo!(),
                FigureCanvasValue::Graphics2d { value: value1 } => value0.add(value1),
                FigureCanvasValue::GenericF32 {
                    partitioned_samples: _,
                    image_layers: _,
                    shapes: _,
                } => todo!(),
                FigureCanvasValue::GenericI32 {
                    partitioned_samples: _,
                    image_layers: _,
                    shapes: _,
                } => todo!(),
                FigureCanvasValue::GenericGraphics2d {
                    partitioned_samples: _,
                    specific: _,
                } => todo!(),
            },
        }
    }
}
