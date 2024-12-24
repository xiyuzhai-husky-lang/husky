use crate::{error::AllLlmsResult, model::AllLlmModel, AllLlmsClient};
use llm_prelude::transformation::{LlmStringTransformation, LlmStringTransformationRecord};

pub type AllLlmsStringTransformation = LlmStringTransformation<AllLlmModel>;
pub type AllLlmsStringTransformationRecord = LlmStringTransformationRecord<AllLlmModel>;

impl<'db> AllLlmsClient<'db> {
    pub fn apply_transformations_sequentially(
        &self,
        transformations: &[AllLlmsStringTransformation],
        mut input: String,
    ) -> AllLlmsResult<(Vec<AllLlmsStringTransformationRecord>, String)> {
        let mut records = vec![];
        for transformation in transformations {
            let transformation = transformation.clone();
            let (record, next_input) = self.apply_transformation(transformation, input)?;
            input = next_input;
            records.push(record);
        }
        Ok((records, input))
    }

    pub fn apply_transformation(
        &self,
        transformation: AllLlmsStringTransformation,
        input: String,
    ) -> AllLlmsResult<(AllLlmsStringTransformationRecord, String)> {
        let prompt = transformation.prompt(&input);
        let output = self.generate_text(transformation.model, prompt.clone())?;
        let record = AllLlmsStringTransformationRecord {
            transformation,
            input,
            prompt,
            output: output.clone(),
        };
        Ok((record, output))
    }
}
