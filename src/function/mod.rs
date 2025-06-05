pub mod tag;
pub use tag::*;

use std::{fmt::Display, ops::Deref};

use crate::prelude::{Command, Variable};

#[derive(Debug)]
pub struct Function {
    name: String,
    body: Vec<String>,
    namespace: String,
    module: String,
    path: String,
}

impl Function {
    pub fn new(name: &str, path: &str) -> Self {
        let split_path = path.split("::").collect::<Vec<&str>>();
        let namespace = split_path.first().unwrap().to_string();
        let module = split_path
            .join("/")
            .trim_start_matches(namespace.as_str())
            .trim_start_matches("/")
            .to_owned();
        Self {
            name: name.to_owned(),
            body: vec![],
            namespace,
            module,
            path: path.to_owned(),
        }
    }
    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }
    pub fn get_path(&self) -> String {
        self.path.to_owned()
    }
    pub fn add_command<T: Command + ?Sized>(&mut self, command: &T) -> &Self {
        self.body.push(command.to_string());
        self
    }
    pub fn add_variable<T: Variable>(&mut self, variable: &T) -> &Self {
        if !self.body.contains(&variable.get_declaration().to_string()) {
            self.add_command(variable.get_declaration());
        }
        variable.get_init().iter().for_each(|command| {
            self.add_command(command.deref());
        });
        self
    }
}

impl Command for Function {}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function")
    }
}
