use crate::{core::WaitFor, Image};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GenericImage {
    descriptor: String,
    arguments: Vec<String>,
    volumes: HashMap<String, String>,
    env_vars: HashMap<String, String>,
    wait_for: Vec<WaitFor>,
    entrypoint: Option<String>,
}

impl Default for GenericImage {
    fn default() -> Self {
        Self {
            descriptor: "".to_owned(),
            arguments: vec![],
            volumes: HashMap::new(),
            env_vars: HashMap::new(),
            wait_for: Vec::new(),
            entrypoint: None,
        }
    }
}

impl GenericImage {
    pub fn new<S: Into<String>>(descriptor: S) -> GenericImage {
        Self {
            descriptor: descriptor.into(),
            ..Default::default()
        }
    }

    pub fn with_volume<F: Into<String>, D: Into<String>>(mut self, from: F, dest: D) -> Self {
        self.volumes.insert(from.into(), dest.into());
        self
    }

    pub fn with_env_var<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.env_vars.insert(key.into(), value.into());
        self
    }

    pub fn with_wait_for(mut self, wait_for: WaitFor) -> Self {
        self.wait_for.push(wait_for);
        self
    }

    pub fn with_entrypoint(mut self, entrypoint: &str) -> Self {
        self.entrypoint = Some(entrypoint.to_string());
        self
    }
}

impl Image for GenericImage {
    type Args = Vec<String>;

    fn descriptor(&self) -> String {
        self.descriptor.to_owned()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        self.wait_for.clone()
    }

    fn args(&self) -> Self::Args {
        self.arguments.clone()
    }

    fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.env_vars.iter())
    }

    fn volumes(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.volumes.iter())
    }

    fn with_args(self, arguments: Self::Args) -> Self {
        Self { arguments, ..self }
    }

    fn entrypoint(&self) -> Option<String> {
        self.entrypoint.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_env_vars() {
        let image = GenericImage::new("hello")
            .with_env_var("one-key", "one-value")
            .with_env_var("two-key", "two-value");

        let mut env_vars = image.env_vars();
        let (first_key, first_value) = env_vars.next().unwrap();
        let (second_key, second_value) = env_vars.next().unwrap();

        assert_eq!(first_key, "one-key");
        assert_eq!(first_value, "one-value");
        assert_eq!(second_key, "two-key");
        assert_eq!(second_value, "two-value");
    }
}
