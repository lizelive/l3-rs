//pub use smartstring::alias::String as String;
pub use ::std::collections::BTreeMap as Map;

pub use serde_json::{Value as JsonValue};

/// A value that can be hashed
#[derive(Clone, PartialEq, Eq, Hash, Debug, ::serde::Serialize, ::serde::Deserialize)]
#[serde(untagged)]
pub enum HashableValue {
    Bool(bool),
    Number(i64),
    String(String),
    List(Vec<HashableValue>),
    Map(Map<String, HashableValue>),
    Null,
}

pub enum HashableNumber {
    
}

impl Default for HashableValue {
    fn default() -> Self {
        HashableValue::Null
    }
}

#[derive(Debug, crate::ThisError)]
pub enum HashableValueFromJsonValueError {
    #[error("number is not real")]
    BadNumber
}

impl TryFrom<JsonValue> for HashableValue {
    type Error = HashableValueFromJsonValueError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        use HashableValue::*;
        Ok(match value {
            JsonValue::Null => Null,
            JsonValue::Bool(v) => Bool(v),
            JsonValue::Number(n) => match n.as_i64() {
                Some(v) => Number(v),
                None => todo!(),
            },
            JsonValue::String(_) => todo!(),
            JsonValue::Array(_) => todo!(),
            JsonValue::Object(_) => todo!(),
        })
    }
}