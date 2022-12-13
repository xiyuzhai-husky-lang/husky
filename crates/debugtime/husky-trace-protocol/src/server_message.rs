use super::*;

pub type JsonResult<T> = Result<T, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HuskyTracerServerMessage {
    pub opt_request_id: Option<usize>,
    pub variant: HuskyTracerServerMessageVariant,
    pub change: ServerTraceStateChange,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum HuskyTracerServerMessageVariant {
    HotReload,
    Activate,
    ActivateWithError { sample_id: SampleId, error: String },
    TogglePin,
    TogglePinWithError { sample_id: SampleId, error: String },
    ToggleExpansion,
    ToggleShow,
    SetPresentation,
    SetPresentationWithError { sample_id: SampleId, error: String },
}
