use crate::{bones::Edge, render::message::ConwayMessage};
use rustc_hash::FxHashSet as HashSet;
use std::time::Instant;

#[derive(Debug, Clone)]
pub enum Transaction {
    Contraction(HashSet<Edge>),
    Release(HashSet<Edge>),
    Conway(ConwayMessage),
    #[allow(dead_code)]
    ShortenName(usize),
    Name(char),
    Wait(Instant),
    #[allow(dead_code)]
    None,
}
