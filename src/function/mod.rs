pub mod tag;
pub use tag::*;

use std::fmt::Display;

use crate::prelude::{Command, PlayerSelector, resource, scoreboard};

#[derive(Debug)]
pub struct Function {
    name: String,
    body: Vec<String>,
    namespace: Option<String>,
    module: Option<String>,
    path: Option<String>,
}

impl Function {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            body: vec![],
            namespace: None,
            module: None,
            path: None,
        }
    }
    pub fn set_path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
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
    pub fn add_scoreboard(self, name: &str, value_opt: Option<i32>) -> Self {
        let path = self.path.as_ref().unwrap().replace("::", ".");
        let path = format!("{}.{}", path, self.name);
        let self_binding = self.add_command(
            scoreboard()
                .objectives()
                .add(path.as_str(), resource::Dummy),
        );
        match value_opt {
            Some(value) => self_binding.add_command(scoreboard().players().set(
                PlayerSelector::new(name),
                path.as_str(),
                value,
            )),
            None => self_binding,
        }
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
