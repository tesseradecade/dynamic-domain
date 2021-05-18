use crate::{Domain, Value};

pub fn convert_to_secluded(value: Value<i32>) -> Option<i32> {
    Some(
        match value {
            Value::Included(i) => i + 1,
            Value::Secluded(i) => i,
            Value::Infinite => {
                return None;
            },
        }
    )
}