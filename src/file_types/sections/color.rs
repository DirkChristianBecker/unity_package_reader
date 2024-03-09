use crate::from_yaml_trait::FromYaml;
use std::fmt::Display;
use unity_yaml_rust::Yaml;

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

impl FromYaml for Color {
    fn from_yaml(yaml: &Yaml) -> Self {
        let mut y = yaml;
        if y["r"] == Yaml::BadValue {
            (_, y) = y.as_hash().unwrap().iter().next().unwrap();
        }

        Color {
            r: y["r"].as_f64().expect("r component should exist"),
            g: y["g"].as_f64().expect("g component should exist"),
            b: y["b"].as_f64().expect("b component should exist"),
            a: y["a"].as_f64().expect("a component should exist"),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {} {})", self.r, self.g, self.b, self.a)
    }
}
