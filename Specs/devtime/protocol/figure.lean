structure FigureCanvasElementArena

structure FigureCanvasElementIdx

inductive FigureCanvasAtom

structure MutationCanvasData where
  before: Option FigureCanvasElementIdx
  after: FigureCanvasElementIdx

inductive SpecificFigureCanvasData
  | Single
  | Mutations (mutations : List MutationCanvasData)

inductive GenericFigureCanvasData