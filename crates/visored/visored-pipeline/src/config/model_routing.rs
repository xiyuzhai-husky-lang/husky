use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VdPipelineModelRouting {
    pub solver: VdPipelineModelSolverRouting,
    pub verifier: VdPipelineModelVerifierRouting,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VdPipelineModelSolverRouting {
    pub mathematical_reasoning: String,
    pub mathematical_understanding: String,
    pub latex_rewriter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VdPipelineModelVerifierRouting {
    pub snl_dispatcher: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VdPipelineModelRoutingResolved {
    pub solver: VdPipelineModelSolverRoutingResolved,
    pub verifier: VdPipelineModelVerifierRoutingResolved,
}

impl VdPipelineModelRoutingResolved {
    pub fn new(routing: &VdPipelineModelRouting, presets: &VdPipelineModelPresets) -> Self {
        // Create a map of preset names to presets for efficient lookup, checking for duplicates
        let mut preset_map = std::collections::HashMap::new();
        for preset in presets {
            if preset_map.insert(preset.name.as_str(), preset).is_some() {
                panic!("Duplicate preset name found in presets: {}", preset.name);
            }
        }

        // Helper closure to find preset by name
        let t = |name: &str| -> VdPipelineModelPreset {
            (*preset_map
                .get(name)
                .expect(&format!("Preset not found: {}", name)))
            .clone()
        };

        Self {
            solver: VdPipelineModelSolverRoutingResolved {
                mathematical_reasoning: t(&routing.solver.mathematical_reasoning),
                mathematical_understanding: t(&routing.solver.mathematical_understanding),
                latex_rewriter: t(&routing.solver.latex_rewriter),
            },
            verifier: VdPipelineModelVerifierRoutingResolved {
                snl_dispatcher: t(&routing.verifier.snl_dispatcher),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VdPipelineModelSolverRoutingResolved {
    pub mathematical_reasoning: VdPipelineModelPreset,
    pub mathematical_understanding: VdPipelineModelPreset,
    pub latex_rewriter: VdPipelineModelPreset,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VdPipelineModelVerifierRoutingResolved {
    pub snl_dispatcher: VdPipelineModelPreset,
}
