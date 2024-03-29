use chrono::NaiveDateTime;
use serde::{Serialize, Serializer};

// The Serialize trait is not impl'd for NaiveDateTime
// This is a custom wrapper type to get around that
#[derive(Debug, PartialEq)]
pub struct CustomDateTime(pub NaiveDateTime);

impl Serialize for CustomDateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let s = self.0.format("%Y-%m-%dT%H:%M:%S.%3fZ");
        serializer.serialize_str(&s.to_string())
    }
}

use async_graphql::*;
#[Scalar]
impl ScalarType for CustomDateTime {
  fn parse(value: Value) -> InputValueResult<Self> { todo!() } 

  fn to_value(&self) -> Value {
    Value::String(self.0.format("%Y-%m-%dT%H:%M:%S.%3fZ").to_string())
  }
}