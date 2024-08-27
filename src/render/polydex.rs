use serde::{Deserialize, Serialize};

pub type Polydex = Vec<Entry>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Entry {
    pub conway: String,
    pub name: String,
    pub bowers: String,
    pub wiki: String,
}
