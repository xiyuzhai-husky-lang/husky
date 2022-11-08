use super::*;
use ref_arena::RefArena;

pub struct ClientFigureCanvasElementStorage(RefArena<FigureCanvasAtom, 1000>);
