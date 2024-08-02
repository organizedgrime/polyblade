use super::Edge;
use crate::ConwayMenu;
use std::{collections::HashSet, time::Instant};

#[derive(Debug, Clone)]
pub enum Transaction {
    Contraction(HashSet<Edge>),
    Release(HashSet<Edge>),
    Conway(ConwayMenu),
    #[allow(dead_code)]
    ShortenName(usize),
    Name(char),
    Wait(Instant),
    #[allow(dead_code)]
    None,
}
