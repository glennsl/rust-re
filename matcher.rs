
use super::compile;

#[deriving(Clone)]
pub struct Match {
    start: uint,
    end: uint
}

pub trait Matcher {
    fn do_match(code: &[compile::Instruction], input: &str) -> Option<~Match>;
}