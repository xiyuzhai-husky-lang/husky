use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "kind")]
pub enum SpecificFigureCanvasData {
    Atom(FigureCanvasAtom),
    Mutations { mutations: Vec<MutationFigureData> },
    EvalError { message: String },
}

impl SpecificFigureCanvasData {
    pub fn new_atom(visual_data: VisualData) -> Self {
        SpecificFigureCanvasData::Atom(FigureCanvasAtom::new(visual_data))
    }
}

impl Default for SpecificFigureCanvasData {
    fn default() -> Self {
        SpecificFigureCanvasData::Atom(Default::default())
    }
}
