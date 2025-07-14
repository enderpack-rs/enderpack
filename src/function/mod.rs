pub mod tag;
pub use tag::*;

use std::{fmt::Display, ops::Deref};

use crate::prelude::{Command, Variable};

pub trait CommandRegister<T> {
    fn add_command(&mut self, command: T) -> &Self;
}

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    body: Vec<String>,
    namespace: String,
    module: String,
    path: String,
    pub implicit_registrations: Vec<Function>,
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
            implicit_registrations: vec![],
        }
    }
    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }
    pub fn get_path(&self) -> String {
        self.path.to_owned()
    }
    pub fn add_variable(&mut self, variable: &impl Variable) -> &Self {
        if !self.body.contains(&variable.get_declaration().to_string()) {
            self.add_command(variable.get_declaration());
        }
        variable.get_init().iter().for_each(|command| {
            self.add_command(command.deref());
        });
        self
    }
}

impl<T: Command + ?Sized> CommandRegister<&T> for Function {
    fn add_command(&mut self, command: &T) -> &Self {
        self.body.push(command.to_string());
        self
    }
}

impl CommandRegister<&Function> for Function {
    fn add_command(&mut self, command: &Function) -> &Self {
        self.implicit_registrations.push(command.clone());
        self.body.push(command.to_string());
        self
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function {}:", self.namespace)?;
        if !self.module.is_empty() {
            write!(f, "{}/", self.module)?;
        }
        write!(f, "{}", self.name)?;
        Ok(())
    }
}
