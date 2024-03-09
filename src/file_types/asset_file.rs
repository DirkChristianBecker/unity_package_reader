use crate::from_yaml_trait::FromYaml;
use std::path::{Path, PathBuf};
use unity_yaml_rust::{AssetType, Yaml, YamlLoader};

pub trait AssetFile {
    fn get_file_id(&self) -> u64;
    fn set_file_id(&mut self, id: u64);

    fn get_guid(&self) -> String;
    fn set_guid(&mut self, guid: &str);
}

impl dyn AssetFile {
    /// Get the path were the templates are stored inside the project.
    /// The file name is the name of the file we want to load.
    pub fn get_default_path(file_name: &str) -> PathBuf {
        if let Ok(mut r) = std::env::current_dir() {
            r.push("src");
            r.push("file_types");
            r.push("templates");
            r.push(file_name);

            return r;
        }

        panic!("Current path return no path");
    }

    /// Determine the asset type that can be found inside a given file.
    pub fn get_asset_type_from_file(path: &Path) -> AssetType {
        match YamlLoader::load_from_path(&path.to_path_buf()) {
            Ok(r) => {
                for e in r {
                    match e {
                        Yaml::DocumentMeta(t, _fid) => match AssetType::try_from(t) {
                            Ok(s) => {
                                return s;
                            }
                            Err(_) => {
                                panic!("Could not convert a value to asset type");
                            }
                        },

                        _ => {
                            continue;
                        }
                    }
                }

                AssetType::Object
            }
            Err(_) => AssetType::Object,
        }
    }

    /// Load an yaml file from the given path.
    pub fn load_asset_file_from_path<T: FromYaml>(path: &std::path::Path) -> Option<T> {
        match YamlLoader::load_from_path(&path.to_path_buf()) {
            Ok(r) => {
                for e in r {
                    match e {
                        Yaml::Hash(ref _hash) => {
                            return Some(T::from_yaml(&e));
                        }

                        _ => {
                            continue;
                        }
                    }
                }

                None
            }

            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::file_types::{asset_file::*, prelude::GameObject};
    use std::collections::HashMap;
    use unity_yaml_rust::AssetType;

    #[test]
    fn test_default_file_path_for_file_name() {
        let files = vec![
            "GameObject.yaml",
            "Light.yaml",
            "LightmapSettings.yaml",
            "Material.yaml",
            "MeshFilter.yaml",
            "MeshRenderer.yaml",
            "NavMeshSettings.yaml",
            "OcclusionCullingSettings.yaml",
            "Prefab.yaml",
            "Transform.yaml",
        ];

        for f in files {
            let p = <dyn AssetFile>::get_default_path(f);
            println!("Path: {}", p.clone().to_str().unwrap());
            assert!(p.as_path().exists());
        }
    }

    #[test]
    fn test_get_asset_type_from_file() {
        let mut files: HashMap<&str, AssetType> = HashMap::new();
        files.insert("GameObject.yaml", AssetType::GameObject);
        files.insert("Light.yaml", AssetType::Light);
        files.insert("LightmapSettings.yaml", AssetType::LightmapSettings);
        files.insert("Material.yaml", AssetType::Material);
        files.insert("MeshFilter.yaml", AssetType::MeshFilter);
        files.insert("MeshRenderer.yaml", AssetType::MeshRenderer);
        files.insert("NavMeshSettings.yaml", AssetType::NavMeshSettings);
        files.insert(
            "OcclusionCullingSettings.yaml",
            AssetType::OcclusionCullingSettings,
        );
        files.insert("Prefab.yaml", AssetType::PrefabInstance);
        files.insert("Transform.yaml", AssetType::Transform);

        for (k, v) in files {
            let x =
                <dyn AssetFile>::get_asset_type_from_file(&<dyn AssetFile>::get_default_path(k));
            assert_eq!(v, x);
        }
    }

    #[test]
    fn test_asset_file_loading() {
        let path = &<dyn AssetFile>::get_default_path("GameObject.yaml");
        let asset = <dyn AssetFile>::load_asset_file_from_path::<GameObject>(path).unwrap();
        assert_eq!(asset.get_is_active(), 1);
        assert_eq!(asset.get_object_hide_flags(), 0);
        assert_eq!(asset.get_serialized_version(), 5);
    }
}
