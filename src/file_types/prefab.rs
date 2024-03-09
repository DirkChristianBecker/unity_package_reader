use std::vec::Vec;
use crate::from_yaml_trait::*;
use crate::file_types::sections::Empty;
use crate::file_types::sections::FileReference;
use unity_yaml_rust::Yaml;
use crate::file_types::sections::PrefabModification;

#[derive(Debug, Clone, PartialEq)]
pub struct Modification {
	transform_parent : FileReference,
	modifications : Vec<PrefabModification>,
	removed_components : Vec<Empty>,

}

impl FromYaml for Modification {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["Modification"] != Yaml::BadValue {
			y = &yaml["Modification"];
		}

		Modification {
			transform_parent : FileReference::from_yaml(&y["m_TransformParent"]),
			modifications : read_yaml_vector(&y["m_Modifications"], "m_Modifications"),
			removed_components : Vec::new(),
		}
	}

}

impl Modification {
	pub fn get_transform_parent(&self) -> FileReference { self.transform_parent.clone() }
	pub fn set_transform_parent(&mut self, value : FileReference) { self.transform_parent = value; }

	pub fn get_modifications(&self) -> Vec<PrefabModification> { self.modifications.clone() }
	pub fn set_modifications(&mut self, value : Vec<PrefabModification>) { self.modifications = value; }

	pub fn get_removed_components(&self) -> Vec<Empty> { self.removed_components.clone() }
	pub fn set_removed_components(&mut self, value : Vec<Empty>) { self.removed_components = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct Prefab {
	object_hide_flags : i64,
	serialized_version : i64,
	modification : Modification,
	parent_prefab : FileReference,
	root_game_object : FileReference,
	is_prefab_parent : i64,

}

impl FromYaml for Prefab {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["Prefab"] != Yaml::BadValue {
			y = &yaml["Prefab"];
		}

		Prefab {
			object_hide_flags : y["m_ObjectHideFlags"].as_i64().expect("Could not find item m_ObjectHideFlags in Prefab."),
			serialized_version : y["serializedVersion"].as_i64().expect("Could not find item serializedVersion in Prefab."),
			modification : Modification::from_yaml(&y["m_Modification"]),
			parent_prefab : FileReference::from_yaml(&y["m_ParentPrefab"]),
			root_game_object : FileReference::from_yaml(&y["m_RootGameObject"]),
			is_prefab_parent : y["m_IsPrefabParent"].as_i64().expect("Could not find item m_IsPrefabParent in Prefab."),
		}
	}

}

impl Prefab {
	pub fn get_object_hide_flags(&self) -> i64 { self.object_hide_flags }
	pub fn set_object_hide_flags(&mut self, value : i64) { self.object_hide_flags = value; }

	pub fn get_serialized_version(&self) -> i64 { self.serialized_version }
	pub fn set_serialized_version(&mut self, value : i64) { self.serialized_version = value; }

	pub fn get_modification(&self) -> Modification { self.modification.clone() }
	pub fn set_modification(&mut self, value : Modification) { self.modification = value; }

	pub fn get_parent_prefab(&self) -> FileReference { self.parent_prefab.clone() }
	pub fn set_parent_prefab(&mut self, value : FileReference) { self.parent_prefab = value; }

	pub fn get_root_game_object(&self) -> FileReference { self.root_game_object.clone() }
	pub fn set_root_game_object(&mut self, value : FileReference) { self.root_game_object = value; }

	pub fn get_is_prefab_parent(&self) -> i64 { self.is_prefab_parent }
	pub fn set_is_prefab_parent(&mut self, value : i64) { self.is_prefab_parent = value; }

}

#[cfg(test)]
mod prefab_tests {
	use super::Prefab;
	use crate::file_types::asset_file::*;

	#[test]
	fn test_prefab() {
		let mut path = std::env::current_dir().unwrap();
		path.push("src");
		path.push("file_types");
		path.push("templates");
		path.push("Prefab.yaml");
		println!("{}", path.display());
		let subject = <dyn AssetFile>::load_asset_file_from_path::<Prefab>(&path).unwrap();

		//Tests
		assert_eq!(subject.object_hide_flags, 0);
		assert_eq!(subject.serialized_version, 2);
		assert_eq!(subject.is_prefab_parent, 0);
	}
}


