use crate::domain::model::file_id::FileId;
use serde::{Serialize, Serializer};

impl Serialize for FileId {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let value = format!("{}", self);
        serializer.serialize_str(&value)
    }
}