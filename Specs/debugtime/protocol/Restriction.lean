inductive RestrictionKind
  | Generic
  | Specific
  | Panic

structure SampleId

structure Partitions

structure Arrivals

structure Restriction where
  kind : RestrictionKind
  sample_id: SampleId
  partitions: Partitions
  arrivals: Arrivals