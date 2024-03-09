use crate::from_yaml_trait::FromYaml;
use std::fmt::Display;
use unity_yaml_rust::Yaml;

#[derive(Debug, Clone, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl FromYaml for Vec3 {
    fn from_yaml(yaml: &Yaml) -> Self {
        let mut y = yaml;
        if y["x"] == Yaml::BadValue {
            (_, y) = y.as_hash().unwrap().iter().next().unwrap();
        }

        Vec3 {
            x: y["x"].as_f64().expect("x component should exist"),
            y: y["y"].as_f64().expect("y component should exist"),
            z: y["z"].as_f64().expect("z component should exist"),
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
