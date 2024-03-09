use crate::from_yaml_trait::FromYaml;
use std::fmt::{Debug, Display};
use unity_yaml_rust::Yaml;

use super::{FileReference, Vec2};

#[derive(Debug, Clone, PartialEq)]
pub struct Texture {
    texture: FileReference,
    scale: Vec2,
    offset: Vec2,
}

impl FromYaml for Texture {
    fn from_yaml(yaml: &Yaml) -> Self {
        let mut y = yaml;
        if y["m_Texture"] == Yaml::BadValue {
            (_, y) = y.as_hash().unwrap().iter().next().unwrap();
        }

        Texture {
            texture: FileReference::from_yaml(&y["m_Texture"]),
            scale: Vec2::from_yaml(&y["m_Scale"]),
            offset: Vec2::from_yaml(&y["m_Offset"]),
        }
    }
}

impl Display for Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match Display::fmt(&self.texture, f) {
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

        match Display::fmt(&self.scale, f) {
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

        Display::fmt(&self.offset, f)
    }
}
