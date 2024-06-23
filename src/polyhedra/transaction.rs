use super::Edge;
use crate::ConwayMessage;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Transaction {
    Contraction(HashSet<Edge>),
    Conway(ConwayMessage),
    Name(char),
    #[allow(dead_code)]
    None,
}
