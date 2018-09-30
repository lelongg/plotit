use failure::Error;
use serde_derive::*;
use serde_json;
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Data {
    pub stamp: f64,
    pub value: String,
}

impl TryFrom<Data> for String {
    type Error = Error;

    fn try_from(data: Data) -> Result<Self, Error> {
        serde_json::to_string(&data).map_err(|err| err.into())
    }
}
