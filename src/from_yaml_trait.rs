use std::collections::HashMap;

use unity_yaml_rust::Yaml;

pub trait FromYaml {
    fn from_yaml(yaml: &Yaml) -> Self;
}

impl FromYaml for f64 {
    fn from_yaml(yaml: &Yaml) -> Self {
        match yaml {
            Yaml::Real(_) => {
                if let Some(r) = yaml.as_f64() {
                    r
                } else {
                    panic!("Could not read a float value.");
                }
            }

            Yaml::Integer(_) => {
                if let Some(r) = yaml.as_i64() {
                    r as f64
                } else {
                    panic!("Could not read a float value.");
                }
            }

            _ => {
                panic!("Expected a float or integer value but found something else.");
            }
        }
    }
}

impl FromYaml for i64 {
    fn from_yaml(yaml: &Yaml) -> Self {
        match yaml {
            Yaml::Integer(_) => {
                if let Some(r) = yaml.as_i64() {
                    r
                } else {
                    panic!("Could not read an integer value.");
                }
            }

            _ => {
                panic!("Expected an integer but found something else.");
            }
        }
    }
}

impl FromYaml for Vec<i64> {
    fn from_yaml(yaml: &Yaml) -> Self {
        let mut result: Vec<i64> = Vec::new();
        match yaml {
            Yaml::Array(ref a) => {
                for e in a {
                    if let Some(i) = e.as_i64() {
                        result.push(i);
                    } else {
                        panic!("Could not read an integer value.");
                    }
                }
            }

            _ => {
                panic!("Expected a real but found something else.");
            }
        }

        result
    }
}

pub fn read_yaml_vector<T: FromYaml>(yaml: &Yaml, _name: &str) -> Vec<T> {
    let y = yaml;

    match y {
        Yaml::Array(ref array) => {
            let mut result: Vec<T> = Vec::new();
            for e in array {
                let tmp = T::from_yaml(e);
                result.push(tmp);
            }

            result
        }

        _ => {
            panic!("The given yaml is not an array.");
        }
    }
}

fn yaml_value_to_string(y : &Yaml) -> String {
    match y {
        Yaml::Real(ref s) => s.to_string(),
        Yaml::Integer(i) => format!("{}", i),
        Yaml::Boolean(b) => format!("{}", b),
        _ => String::from(""),
    }
}

impl FromYaml for HashMap<String, String> {
    fn from_yaml(yaml: &Yaml) -> Self {
        let mut result : HashMap<String, String> = HashMap::new();

        match yaml {
            Yaml::Hash(ref a) => {
                for (k, v) in a.map.iter() {
                    let name = yaml_value_to_string(k);
                    let value = yaml_value_to_string(v);

                    if name.is_empty() { continue; }

                    result.insert(name, value);
                }
            }

            _ => {
                panic!("Expected an hashmap but found something else.");
            }
        }

        result
    }
}
