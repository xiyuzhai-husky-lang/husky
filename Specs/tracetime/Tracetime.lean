import Specs.tracetime.Tracetime.TraceNode
import Specs.tracetime.Trace
import Specs.abstraction.vec_like
import Specs.abstraction.HashMap
import Specs.tracetime.protocol

structure TracetimeState where
    restriction: Restriction
    pins: VecSet TraceId

structure TraceFactory where

structure TracetimeDb where
  trace_nodes: List (Option TraceNode)
  opt_active_trace_id: Option TraceId
  figure_canvases: VecSet FigureCanvasKey
  figure_controls: HashMap FigureControlKey FigureControlData
  trace_stalks: HashMap TraceStalkKey TraceStalk
  trace_statss: HashMap TraceStatsKey (Option TraceStats)
  root_traces: List TraceId
  subtrace_ids_map: HashMap SubtracesKey (List TraceId)

structure Tracetime where
  state : TracetimeState
  db : TracetimeDb