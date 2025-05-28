use std::{
    fmt::Display,
    ops::{Range, RangeFrom, RangeTo},
};

#[derive(Debug)]
pub enum MCRange {
    Bound(Range<f64>),
    From(RangeFrom<f64>),
    To(RangeTo<f64>),
}

impl Display for MCRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_string = match self {
            MCRange::Bound(range) => format!("{}..{}", range.start, range.end),
            MCRange::From(range) => format!("{}..", range.start),
            MCRange::To(range) => format!("..{}", range.end),
        };
        write!(f, "{}", as_string)
    }
}
