use super::*;
use std::ops::Range;

#[derive(Debug)]
pub(super) struct GenericF32Scale {
    partitions_len: usize,
    value_min: f32,
    value_max: f32,
    value_padding: f32,
    r: f32,
}

impl GenericF32Scale {
    pub fn new(partitioned_samples: &[(PartitionDefnData, Vec<(SampleId, f32)>)]) -> Self {
        let mut value_min = f32::MAX;
        let mut value_max = f32::MIN;
        for (_, samples) in partitioned_samples {
            for (sample, value) in samples {
                if *value < value_min {
                    value_min = *value
                } else if *value > value_max {
                    value_max = *value
                }
            }
        }
        let value_padding = if value_min == value_max {
            1.0
        } else {
            (value_max - value_min) * 0.05
        };
        Self {
            partitions_len: partitioned_samples.len(),
            value_min,
            value_max,
            value_padding,
            r: 5.0,
        }
    }

    pub fn normalized_class_index(&self, class_index: usize) -> f32 {
        let middle = ((self.partitions_len - 1) as f32) / 2.0;
        ((class_index as f32) - middle) / (self.partitions_len as f32)
    }

    pub fn normalized_value(&self, value: f32) -> f32 {
        let middle = (self.value_max + self.value_min) / 2.0;
        (value - middle) / (self.value_max - self.value_min + self.value_padding)
    }

    pub fn circle(&self, class_index: usize, value: f32) -> CircleProps {
        let cx = 150. + 300. * self.normalized_class_index(class_index);
        let cy = 500. + 1000. * self.normalized_value(value);
        CircleProps {
            class_index,
            cx,
            cy,
            r: self.r,
        }
    }

    pub fn svg_view_box(&self) -> &'static str {
        "0 0 300 1000"
    }
}
pub struct CircleProps {
    pub(super) class_index: usize,
    pub(super) cx: f32,
    pub(super) cy: f32,
    pub(super) r: f32,
}
