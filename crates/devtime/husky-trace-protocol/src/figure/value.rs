use crate::*;
use husky_vm_primitive_value::PrimitiveValueData;

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
    Graphics2d {
        graphics2d_data: Graphics2dCanvasValue,
    },
    GenericGraphics2d {
        partitioned_samples: Vec<(&'static Partition, Vec<(SampleId, Graphics2dCanvasValue)>)>,
        particular: Graphics2dCanvasValue,
    },
}

impl Default for FigureCanvasValue {
    fn default() -> Self {
        FigureCanvasValue::Primitive {
            value: PrimitiveValueData::Void(()),
        }
    }
}

#[derive(Clone)]
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
        todo!()
    }

    fn add(&mut self, other: FigureCanvasValue) {
        match self {
            FigureCanvasValue::Primitive { value } => todo!(),
            FigureCanvasValue::GenericF32 {
                partitioned_samples,
            } => todo!(),
            FigureCanvasValue::GenericI32 {
                partitioned_samples,
            } => todo!(),
            FigureCanvasValue::Graphics2d {
                graphics2d_data: particular,
            } => todo!(),
            FigureCanvasValue::GenericGraphics2d {
                partitioned_samples,
                particular,
            } => todo!(),
        }
    }
}
