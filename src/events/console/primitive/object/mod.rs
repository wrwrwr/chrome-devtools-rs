mod object;
mod subtype;

use std::fmt;

pub use object::Object;
pub use subtype::Subtype;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObjectData {
    Subtype(Subtype),
    Object(Object),
}

impl fmt::Display for ObjectData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ObjectData::Subtype(subtype) => write!(f, "{}", subtype),
            ObjectData::Object(object) => write!(f, "{}", object),
        }
    }
}
