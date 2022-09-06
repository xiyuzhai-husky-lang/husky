use crate::*;

impl HuskyTracetime {
    pub(crate) fn update_trace_statss(
        &mut self,
    ) -> __VMResult<Vec<(TraceStatsKey, Option<TraceStats>)>> {
        let mut trace_statss = Vec::new();
        for root_trace_id in self.root_trace_ids.clone() {
            self.update_trace_statss_within_trace(root_trace_id, &mut trace_statss)?;
        }
        Ok(trace_statss)
    }

    fn update_trace_statss_within_trace(
        &mut self,
        trace_id: TraceId,
        new_trace_statss: &mut Vec<(TraceStatsKey, Option<TraceStats>)>,
    ) -> __VMResult<()> {
        let trace_node_data = self.trace_node_data(trace_id);
        let expanded = trace_node_data.expanded;
        let trace_raw_data = &trace_node_data.trace_data;
        let trace_stats_key = TraceStatsKey {
            trace_id,
            partitions: self.restriction.partitions().clone(),
        };
        let associated_trace_ids = trace_raw_data.associated_trace_ids();
        if !self.trace_statss.contains_key(&trace_stats_key) {
            self.gen_trace_stats(trace_id, trace_stats_key, new_trace_statss)?
        }
        for associated_trace_id in associated_trace_ids {
            self.update_trace_statss_within_trace(associated_trace_id, new_trace_statss)?
        }
        if expanded {
            for subtrace_id in self.subtrace_ids(trace_id) {
                self.update_trace_statss_within_trace(subtrace_id, new_trace_statss)?
            }
        }
        Ok(())
    }

    fn gen_trace_stats(
        &mut self,
        trace_id: TraceId,
        key: TraceStatsKey,
        new_trace_statss: &mut Vec<(TraceStatsKey, Option<TraceStats>)>,
    ) -> __VMResult<()> {
        match self
            .trace(trace_id)
            .variant
            .opt_stats(self.runtime(), self.restriction.partitions())
        {
            Ok(opt_trace_stats) => {
                self.trace_statss
                    .insert(key.clone(), opt_trace_stats.clone());
                new_trace_statss.push((key, opt_trace_stats));
                Ok(())
            }
            Err(e) => {
                self.trace_statss.insert(key.clone(), None);
                new_trace_statss.push((key, None));
                self.throw_otherworldly(e)
            }
        }
    }

    fn throw_otherworldly(&self, e: __VMError) -> __VMResult<()> {
        match e.variant() {
            __VMErrorVariant::Normal => todo!(),
            __VMErrorVariant::FromBatch { sample_id } => {
                if self.restriction.is_generic()
                    || self.restriction.sample_id() != SampleId(*sample_id)
                {
                    Err(e)
                } else {
                    Ok(())
                }
            }
        }
    }
}
