use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "kind")]
pub enum SpecificFigureCanvasData {
    Unit,
    Atom(FigureCanvasAtom),
    Mutations { mutations: Vec<MutationFigureData> },
    EvalError { message: String },
}

impl SpecificFigureCanvasData {
    pub fn from_visual_data(visual_data: VisualData) -> Self {
        if let Some(figure_canvas_atom) = FigureCanvasAtom::new(visual_data) {
            SpecificFigureCanvasData::Atom(figure_canvas_atom)
        } else {
            Default::default()
        }
    }
}

impl Default for SpecificFigureCanvasData {
    fn default() -> Self {
        SpecificFigureCanvasData::Unit
    }
}

impl Signalable for SpecificFigureCanvasData {}
