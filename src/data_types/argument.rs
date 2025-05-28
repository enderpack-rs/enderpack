use std::fmt::Display;

#[derive(Debug)]
pub struct Argument<T>
where
    T: Display,
{
    name: String,
    value: T,
}

impl<T> Argument<T>
where
    T: Display,
{
    pub fn new(name: &str, value: T) -> Self {
        Argument {
            name: name.to_owned(),
            value,
        }
    }
}

impl<T> Display for Argument<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}
