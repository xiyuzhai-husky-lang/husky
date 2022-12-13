inductive PresentationKind
  | Generic
  | Specific
  | Panic

structure SampleId

structure Partitions

structure Arrivals

structure Presentation where
  kind : PresentationKind
  sample_id: SampleId
  partitions: Partitions
  arrivals: Arrivals