use serde_derive::*;
use serde_json;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Data {
    pub stamp: f64,
    pub value: String,
}

impl From<Data> for String {
    fn from(data: Data) -> Self {
        match serde_json::to_string(&data) {
            Ok(data) => data,
            Err(_) => "".to_string(),
        }
    }
}
