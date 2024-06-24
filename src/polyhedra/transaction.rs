use super::Edge;
use crate::ConwayMessage;
use std::{collections::HashSet, time::Instant};

#[derive(Debug, Clone)]
pub enum Transaction {
    Contraction(HashSet<Edge>),
    Release(HashSet<Edge>),
    Conway(ConwayMessage),
    ShortenName(usize),
    Name(char),
    Wait(Instant),
    #[allow(dead_code)]
    None,
}
