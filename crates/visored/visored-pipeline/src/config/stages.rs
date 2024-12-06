use super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VdPipelineStagesConfig {
    /// Given a problem, generate a natural solution.
    #[serde(default)]
    question_to_natural_solution_stage: (),
    /// Given a natural solution, generate a detailed natural solution.
    #[serde(default)]
    natural_solution_to_details_stage: (),
    /// Given a detailed natural solution, generate a natural language explanation.
    #[serde(default)]
    details_to_lean_sketch_stage: (),
    /// Given a lean sketch, generate a lean proof.
    #[serde(default)]
    lean_sketch_completion_stage: (),
}
