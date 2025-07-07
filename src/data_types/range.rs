use std::{
    fmt::Display,
    ops::{Range, RangeFrom, RangeTo},
};

#[derive(Debug, Clone)]
pub enum MCRange {
    Bound(Range<f64>),
    From(RangeFrom<f64>),
    To(RangeTo<f64>),
    Exact(f64),
}

impl Display for MCRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_string = match self {
            MCRange::Bound(range) => format!("{}..{}", range.start, range.end),
            MCRange::From(range) => format!("{}..", range.start),
            MCRange::To(range) => format!("..{}", range.end),
            MCRange::Exact(exact) => exact.to_string(),
        };
        write!(f, "{}", as_string)
    }
}

impl From<Range<f64>> for MCRange {
    fn from(value: Range<f64>) -> Self {
        MCRange::Bound(value)
    }
}

impl From<RangeFrom<f64>> for MCRange {
    fn from(value: RangeFrom<f64>) -> Self {
        MCRange::From(value)
    }
}

impl From<RangeTo<f64>> for MCRange {
    fn from(value: RangeTo<f64>) -> Self {
        MCRange::To(value)
    }
}

impl From<f64> for MCRange {
    fn from(value: f64) -> Self {
        MCRange::Exact(value)
    }
}
