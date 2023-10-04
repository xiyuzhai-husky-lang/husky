// use serde::Serialize;
// #[derive(Debug, Serialize, Clone)]
// #[serde(tag = "kind")]
// pub enum JsonResult<T> {
//     Ok { value: T },
//     Err { message: String },
// }

pub type JsonResult<T> = Result<T, String>;
