use crate::from_yaml_trait::FromYaml;
use std::fmt::{Debug, Display};
use unity_yaml_rust::Yaml;

#[derive(Debug, Clone, PartialEq)]
pub struct FalloffTable {
    table: Vec<f64>,
}

impl FromYaml for FalloffTable {
    fn from_yaml(yaml: &Yaml) -> Self {
        let mut r = FalloffTable { table: Vec::new() };

        if let Some(array) = yaml.as_vec() {
            for a in array {
                match &a {
                    Yaml::Integer(i) => {
                        r.table.push(*i as f64);
                    }
                    Yaml::Real(s) => {
                        r.table.push(s.parse::<f64>().unwrap());
                    }
                    _ => {
                        panic!("Could not read falloff table");
                    }
                }
            }
        }

        r
    }
}

impl Display for FalloffTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("");
        for f in &self.table {
            s.push_str(&f.to_string());
        }

        Display::fmt(&s, f)
    }
}
