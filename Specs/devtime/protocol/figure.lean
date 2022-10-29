structure FigureCanvasElementArena

structure FigureCanvasElementIdx

inductive FigureCanvasElement

structure MutationCanvasData where
  before: Option FigureCanvasElementIdx
  after: FigureCanvasElementIdx

inductive SpecificFigureCanvasData
  | Single
  | Mutations (mutations : List MutationCanvasData)

inductive GenericFigureCanvasData