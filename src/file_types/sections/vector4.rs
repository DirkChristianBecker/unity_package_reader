use crate::from_yaml_trait::FromYaml;
use std::fmt::Display;
use unity_yaml_rust::Yaml;

#[derive(Debug, Clone, PartialEq)]
pub struct Vec4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl FromYaml for Vec4 {
    fn from_yaml(yaml: &Yaml) -> Self {
        let mut y = yaml;
        if y["x"] == Yaml::BadValue {
            (_, y) = y.as_hash().unwrap().iter().next().unwrap();
        }

        Vec4 {
            x: y["x"].as_f64().expect("x component should exist"),
            y: y["y"].as_f64().expect("y component should exist"),
            z: y["z"].as_f64().expect("z component should exist"),
            w: y["w"].as_f64().expect("w component should exist"),
        }
    }
}

impl Display for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {} {})", self.x, self.y, self.z, self.w)
    }
}
