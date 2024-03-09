use crate::from_yaml_trait::FromYaml;
use std::fmt::Display;
use unity_yaml_rust::Yaml;

#[derive(Debug, Clone, PartialEq)]
pub struct FileReference {
    file_id: u64,
    guid: Option<String>,
    file_type: Option<u64>,
}

impl FromYaml for FileReference {
    fn from_yaml(yaml: &Yaml) -> Self {
        let mut y = yaml;
        if y["fileID"] == Yaml::BadValue {
            (_, y) = y.as_hash().unwrap().iter().next().unwrap();
        }

        FileReference {
            file_id: y["fileID"]
                .as_i64()
                .expect("The value 'fileID' should exist.") as u64,
            guid: y["guid"].as_str().map(|s| s.to_string()),
            file_type: y["guid"].as_i64().map(|s| s as u64),
        }
    }
}

impl FileReference {
    pub fn is_valid(&self) -> bool {
        self.file_id != 0
    }

    pub fn get_file_id(&self) -> u64 {
        self.file_id
    }
}

impl Display for FileReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guid: String;
        let file_type: u64;

        if let Some(s) = &self.guid {
            guid = s.clone();
        } else {
            guid = String::from("");
        }

        if let Some(u) = self.file_type {
            file_type = u;
        } else {
            file_type = 0;
        }

        write!(f, "{} : {} ({})", self.file_id, guid, file_type)
    }
}
