pub mod parser;
pub mod runtime;

#[derive(Debug, Clone)]
pub enum Token {
    Inc(usize),
    Dec(usize),
    LMov(usize),
    RMov(usize),
    OutStd,
    InStd,
    OpenBrk(usize),
    ClosedBrk(usize),
}