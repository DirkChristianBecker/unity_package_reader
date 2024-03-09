use crate::from_yaml_trait::FromYaml;
use std::fmt::Display;
use unity_yaml_rust::Yaml;

use super::FileReference;

#[derive(Debug, Clone, PartialEq)]
pub struct PrefabModification {
    target: FileReference,
    property_path: String,
    value: f64,
    object_reference: FileReference,
}

impl FromYaml for PrefabModification {
    fn from_yaml(yaml: &Yaml) -> Self {
        let mut y = yaml;
        if y["propertyPath"] == Yaml::BadValue {
            (_, y) = y.as_hash().unwrap().iter().next().unwrap();
        }

        PrefabModification {
            target: FileReference::from_yaml(&y["target"]),
            property_path: y["propertyPath"]
                .as_str()
                .expect("Element not found")
                .to_string(),
            value: y["value"].as_f64().expect("Element not found"),
            object_reference: FileReference::from_yaml(&y["objectReference"]),
        }
    }
}

impl Display for PrefabModification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match Display::fmt(&self.target, f) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        match writeln!(f) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        match writeln!(f, "Property path: {} -> {}", self.property_path, self.value) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        Display::fmt(&self.object_reference, f)
    }
}
