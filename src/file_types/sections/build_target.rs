use std::fmt::Display;

use unity_yaml_rust::Yaml;

use crate::from_yaml_trait::FromYaml;


#[derive(Debug, Clone, PartialEq)]
pub struct BuildTarget {
    build_target: String,
    max_texture_size: i64,
    texture_format: i64,
    texture_compression: i64,
    compression_quality: i64,
    crunched_compression: i64,
    allows_alpha_splitting: i64,
    overridden: i64,
}

impl FromYaml for BuildTarget {
    fn from_yaml(yaml: &Yaml) -> Self {
        let mut y = yaml;
        if y["buildTarget"] == Yaml::BadValue {
            (_, y) = y.as_hash().unwrap().iter().next().unwrap();
        }

        // print!("{:?}", y);

        BuildTarget {
            build_target: String::from(y["buildTarget"].as_str().expect("buildTarget component should exist")),
            max_texture_size: y["maxTextureSize"].as_i64().expect("maxTextureSize component should exist"),
            texture_format: y["textureFormat"].as_i64().expect("textureFormat component should exist"),
            texture_compression: y["textureCompression"].as_i64().expect("textureCompression component should exist"),
            compression_quality: y["compressionQuality"].as_i64().expect("compressionQuality component should exist"),
            crunched_compression: y["crunchedCompression"].as_i64().expect("crunchedCompression component should exist"),
            allows_alpha_splitting: y["allowsAlphaSplitting"].as_i64().expect("allowsAlphaSplitting component should exist"),
            overridden: y["overridden"].as_i64().expect("overridden component should exist"),
        }
    }
}

impl Display for BuildTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Target: {}\n
            Max Texture Size: {}\n
            Textureformat: {}\n
            Texturecompression: {}\n
            Compression Quality: {}\n
            Crunched Compression: {}\n
            Allows alpha splitting: {}\n
            Overridden: {})", 
            self.build_target, 
            self.max_texture_size, 
            self.texture_format, 
            self.texture_compression,
            self.compression_quality,
            self.crunched_compression,
            self.allows_alpha_splitting,
            self.overridden)
    }
}