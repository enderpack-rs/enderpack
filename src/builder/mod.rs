pub mod namespace;
pub mod version;
pub use namespace::Namespace;
pub use version::Version;

use crate::prelude::{Function, Tag};

#[derive(Debug, Clone)]
pub struct Datapack {
    namespace: Namespace,
    description: String,
    version: Version,
    functions: Vec<(Tag, Function)>,
}

#[macro_export]
macro_rules! get_namespace {
    () => {
        Namespace::new(env!("CARGO_CRATE_NAME"))
    };
}

impl Datapack {
    pub fn new(namespace: Namespace, description: &str, version: Version) -> Self {
        Self {
            namespace,
            description: description.to_owned(),
            version,
            functions: vec![],
        }
    }
    pub fn add_function<T: Fn() -> Function>(mut self, tag: Tag, function_factory: T) -> Self {
        let function = function_factory();
        self.functions.push((tag, function.clone()));
        function
            .implicit_registrations
            .into_iter()
            .for_each(|implicit_function| self.functions.push((Tag::None, implicit_function)));
        self
    }
}
