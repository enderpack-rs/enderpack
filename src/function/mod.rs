pub mod tag;
pub use tag::*;

use std::fmt::Display;

use crate::prelude::Command;

#[derive(Debug)]
pub struct Function {
    name: String,
    body: Vec<String>,
    namespace: Option<String>,
    module: Option<String>,
}

impl Function {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            body: vec![],
            namespace: None,
            module: None,
        }
    }
    pub fn set_path(mut self, path: &str) -> Self {
        let path = path.split("::").collect::<Vec<&str>>();
        let namespace = path.first().unwrap().to_string();
        let module = path
            .join("/")
            .trim_start_matches(namespace.as_str())
            .trim_start_matches("/")
            .to_owned();
        self.namespace = Some(namespace);
        self.module = Some(module);
        self
    }
    pub fn add_command<T: Command>(mut self, command: T) -> Self {
        self.body.push(command.to_string());
        self
    }
}

impl Command for Function {}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function")
    }
}
