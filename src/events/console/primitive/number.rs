use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct NumberData {
    description: String,
}

impl fmt::Display for NumberData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.description)
    }
}
