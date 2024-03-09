use unity_yaml_rust::Yaml;

use crate::from_yaml_trait::FromYaml;

#[derive(Debug, Clone, PartialEq)]
pub struct NamedValue<T> {
    name: String,
    value: T,
}

impl FromYaml for NamedValue<i64> {
    fn from_yaml(yaml: &Yaml) -> Self {
        let mut n = String::from("");
        let mut v = 0;
        if let Yaml::Hash(ref h) = yaml {
            if h.map.len() > 1 {
                panic!("A named value is a hash with only one element.");
            }

            for (k, va) in h.iter() {
                n = String::from(k.as_str().unwrap());
                v = va.as_i64().unwrap();
            }
        }

        NamedValue { name: n, value: v }
    }
}

impl FromYaml for NamedValue<f64> {
    fn from_yaml(yaml: &Yaml) -> Self {
        let mut n = String::from("");
        let mut v = 0.0;
        if let Yaml::Hash(ref h) = yaml {
            if h.map.len() > 1 {
                panic!("A named value is a hash with only one element.");
            }

            for (k, va) in h.iter() {
                n = String::from(k.as_str().unwrap());
                v = va.as_f64().unwrap();
            }
        }

        NamedValue { name: n, value: v }
    }
}

impl FromYaml for NamedValue<String> {
    fn from_yaml(yaml: &Yaml) -> Self {
        let mut n = String::from("");
        let mut v = String::from("");
        if let Yaml::Hash(ref h) = yaml {
            if h.map.len() > 1 {
                panic!("A named value is a hash with only one element.");
            }

            for (k, va) in h.iter() {
                n = String::from(k.as_str().unwrap());
                v = String::from(va.as_str().unwrap());
            }
        }

        NamedValue { name: n, value: v }
    }
}

impl FromYaml for NamedValue<bool> {
    fn from_yaml(yaml: &Yaml) -> Self {
        let mut n = String::from("");
        let mut v = false;
        if let Yaml::Hash(ref h) = yaml {
            if h.map.len() > 1 {
                panic!("A named value is a hash with only one element.");
            }

            for (k, va) in h.iter() {
                n = String::from(k.as_str().unwrap());
                v = va.as_bool().unwrap();
            }
        }

        NamedValue { name: n, value: v }
    }
}
