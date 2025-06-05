pub mod tag;
pub use tag::*;

use std::{fmt::Display, ops::Deref};

use crate::prelude::{Command, PlayerSelector, Variable, resource, scoreboard};

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
    pub fn add_variable<T: Variable>(&mut self, variable: T) -> &Self {
        if !self.body.contains(&variable.get_declaration().to_string()) {
            self.add_command(variable.get_declaration());
        }
        variable.get_stack().iter().for_each(|command| {
            self.add_command(command.deref());
        });
        self
    }
    pub fn add_scoreboard(&mut self, name: &str, value_opt: Option<i32>) -> &Self {
        let name = format!(".{name}");
        let path = self.path.replace("::", ".");
        let path = format!("{}.{}", path, self.name);
        let declaration = scoreboard()
            .objectives()
            .add(path.as_str(), resource::Criteria::Dummy);
        if !self.body.contains(&declaration.to_string()) {
            self.add_command(&declaration);
        }
        if let Some(value) = value_opt {
            self.add_command(&scoreboard().players().set(
                PlayerSelector::new(name.as_str()),
                path.as_str(),
                value,
            ));
        }
        self
    }
    pub fn add_storage(&mut self) -> &Self {
        todo!()
    }
}

impl Command for Function {}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function")
    }
}
