use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "kind")]
pub enum GenericFigureCanvasData {
    Unit,
    Plot2d {
        plot_kind: Plot2dKind,
        point_groups: Vec<Point2dGroup>,
        xrange: (f32, f32),
        yrange: (f32, f32),
    },
    Graphics2d {
        graphics2d_data: Graphics2dCanvasData,
    },
    GenericGraphics2d {
        partitioned_samples: Vec<(Partition, Vec<(SampleId, Graphics2dCanvasData)>)>,
    },
    GenericF32 {
        partitioned_samples: Vec<(Partition, Vec<(SampleId, f32)>)>,
    },
    GenericI32 {
        partitioned_samples: Vec<(Partition, Vec<(SampleId, i32)>)>,
    },
    EvalError {
        message: String,
    },
}

impl Default for GenericFigureCanvasData {
    fn default() -> Self {
        GenericFigureCanvasData::Unit
    }
}

impl Signalable for GenericFigureCanvasData {}
