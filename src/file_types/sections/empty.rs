use crate::from_yaml_trait::FromYaml;
use unity_yaml_rust::Yaml;

#[derive(Debug, Clone, PartialEq)]
pub struct Empty {}

impl FromYaml for Empty {
    fn from_yaml(_yaml: &Yaml) -> Self {
        Empty {}
    }
}
