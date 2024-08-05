use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CredentialSchema {
    pub id: String,
    pub type_name: String,
    pub properties: HashMap<String, PropertyType>,
    pub required: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PropertyType {
    String,
    Number,
    Boolean,
    Object,
    Array,
}
