use serde::{Deserialize, Serialize};

pub type Polydex = Vec<Entry>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    conway: String,
    name: String,
    bowers: String,
    wiki: String,
}
