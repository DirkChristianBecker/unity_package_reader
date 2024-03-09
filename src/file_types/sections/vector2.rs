use crate::from_yaml_trait::FromYaml;
use std::fmt::Display;
use unity_yaml_rust::Yaml;

#[derive(Debug, Clone, PartialEq)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl FromYaml for Vec2 {
    fn from_yaml(yaml: &Yaml) -> Self {
        let mut y = yaml;
        if y["x"] == Yaml::BadValue {
            (_, y) = y.as_hash().unwrap().iter().next().unwrap();
        }

        Vec2 {
            x: y["x"].as_f64().expect("x component should exist"),
            y: y["y"].as_f64().expect("y component should exist"),
        }
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
