use unity_yaml_rust::Yaml;
use crate::from_yaml_trait::*;
use crate::file_types::sections::FileReference;

#[derive(Debug, Clone, PartialEq)]
pub struct MeshFilter {
	object_hide_flags : i64,
	prefab_parent_object : FileReference,
	prefab_internal : FileReference,
	game_object : FileReference,
	mesh : FileReference,

}

impl FromYaml for MeshFilter {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["MeshFilter"] != Yaml::BadValue {
			y = &yaml["MeshFilter"];
		}

		MeshFilter {
			object_hide_flags : y["m_ObjectHideFlags"].as_i64().expect("Could not find item m_ObjectHideFlags in MeshFilter."),
			prefab_parent_object : FileReference::from_yaml(&y["m_PrefabParentObject"]),
			prefab_internal : FileReference::from_yaml(&y["m_PrefabInternal"]),
			game_object : FileReference::from_yaml(&y["m_GameObject"]),
			mesh : FileReference::from_yaml(&y["m_Mesh"]),
		}
	}

}

impl MeshFilter {
	pub fn get_object_hide_flags(&self) -> i64 { self.object_hide_flags }
	pub fn set_object_hide_flags(&mut self, value : i64) { self.object_hide_flags = value; }

	pub fn get_prefab_parent_object(&self) -> FileReference { self.prefab_parent_object.clone() }
	pub fn set_prefab_parent_object(&mut self, value : FileReference) { self.prefab_parent_object = value; }

	pub fn get_prefab_internal(&self) -> FileReference { self.prefab_internal.clone() }
	pub fn set_prefab_internal(&mut self, value : FileReference) { self.prefab_internal = value; }

	pub fn get_game_object(&self) -> FileReference { self.game_object.clone() }
	pub fn set_game_object(&mut self, value : FileReference) { self.game_object = value; }

	pub fn get_mesh(&self) -> FileReference { self.mesh.clone() }
	pub fn set_mesh(&mut self, value : FileReference) { self.mesh = value; }

}

#[cfg(test)]
mod mesh_filter_tests {
	use super::MeshFilter;
	use crate::file_types::asset_file::*;

	#[test]
	fn test_mesh_filter() {
		let mut path = std::env::current_dir().unwrap();
		path.push("src");
		path.push("file_types");
		path.push("templates");
		path.push("MeshFilter.yaml");
		println!("{}", path.display());
		let subject = <dyn AssetFile>::load_asset_file_from_path::<MeshFilter>(&path).unwrap();

		//Tests
		assert_eq!(subject.object_hide_flags, 0);
	}
}


