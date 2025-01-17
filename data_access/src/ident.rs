use std::fmt;
use strum::{EnumCount, EnumIter};

pub struct PrimaryKey(pub String);

impl PrimaryKey {
    pub fn from(id: &str) -> Self {
        PrimaryKey(id.to_string())
    }
}

pub struct ClientId(pub String);

#[derive(EnumIter, EnumCount, Debug, PartialEq)]
pub enum Model {
    Org,
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_prefix = match self {
            Model::Org => "org",
        };
        write!(f, "{}", str_prefix)
    }
}

pub fn generate(prefix: Model) -> String {
    let str_prefix = prefix.to_string();
    if str_prefix.len() < 3 {
        panic!("An id prefix may not be fewer than 3 characters in length");
    }
    if str_prefix.len() > 6 {
        panic!("An id prefix may not be greater than 6 characters in length");
    }
    let id = uuid::Uuid::new_v4().to_string();
    (&format!("{}_{}", str_prefix, str::replace(&id, "-", ""))[..36]).to_string()
}
