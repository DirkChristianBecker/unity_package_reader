use crate::from_yaml_trait::*;
use std::vec::Vec;
use crate::file_types::sections::FileReference;
use unity_yaml_rust::Yaml;

#[derive(Debug, Clone, PartialEq)]
pub struct GameObject {
	object_hide_flags : i64,
	prefab_parent_object : FileReference,
	prefab_internal : FileReference,
	serialized_version : i64,
	component : Vec<FileReference>,
	layer : i64,
	name : String,
	tag_string : String,
	icon : FileReference,
	nav_mesh_layer : i64,
	static_editor_flags : i64,
	is_active : i64,

}

impl FromYaml for GameObject {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["GameObject"] != Yaml::BadValue {
			y = &yaml["GameObject"];
		}

		GameObject {
			object_hide_flags : y["m_ObjectHideFlags"].as_i64().expect("Could not find item m_ObjectHideFlags in GameObject."),
			prefab_parent_object : FileReference::from_yaml(&y["m_PrefabParentObject"]),
			prefab_internal : FileReference::from_yaml(&y["m_PrefabInternal"]),
			serialized_version : y["serializedVersion"].as_i64().expect("Could not find item serializedVersion in GameObject."),
			component : read_yaml_vector(&y["m_Component"], "m_Component"),
			layer : y["m_Layer"].as_i64().expect("Could not find item m_Layer in GameObject."),
			name : String::from(y["m_Name"].as_str().expect("Could not find item m_Name in GameObject.")),
			tag_string : String::from(y["m_TagString"].as_str().expect("Could not find item m_TagString in GameObject.")),
			icon : FileReference::from_yaml(&y["m_Icon"]),
			nav_mesh_layer : y["m_NavMeshLayer"].as_i64().expect("Could not find item m_NavMeshLayer in GameObject."),
			static_editor_flags : y["m_StaticEditorFlags"].as_i64().expect("Could not find item m_StaticEditorFlags in GameObject."),
			is_active : y["m_IsActive"].as_i64().expect("Could not find item m_IsActive in GameObject."),
		}
	}

}

impl GameObject {
	pub fn get_object_hide_flags(&self) -> i64 { self.object_hide_flags }
	pub fn set_object_hide_flags(&mut self, value : i64) { self.object_hide_flags = value; }

	pub fn get_prefab_parent_object(&self) -> FileReference { self.prefab_parent_object.clone() }
	pub fn set_prefab_parent_object(&mut self, value : FileReference) { self.prefab_parent_object = value; }

	pub fn get_prefab_internal(&self) -> FileReference { self.prefab_internal.clone() }
	pub fn set_prefab_internal(&mut self, value : FileReference) { self.prefab_internal = value; }

	pub fn get_serialized_version(&self) -> i64 { self.serialized_version }
	pub fn set_serialized_version(&mut self, value : i64) { self.serialized_version = value; }

	pub fn get_component(&self) -> Vec<FileReference> { self.component.clone() }
	pub fn set_component(&mut self, value : Vec<FileReference>) { self.component = value; }

	pub fn get_layer(&self) -> i64 { self.layer }
	pub fn set_layer(&mut self, value : i64) { self.layer = value; }

	pub fn get_name(&self) -> String { self.name.clone() }
	pub fn set_name(&mut self, value : String) { self.name = value; }

	pub fn get_tag_string(&self) -> String { self.tag_string.clone() }
	pub fn set_tag_string(&mut self, value : String) { self.tag_string = value; }

	pub fn get_icon(&self) -> FileReference { self.icon.clone() }
	pub fn set_icon(&mut self, value : FileReference) { self.icon = value; }

	pub fn get_nav_mesh_layer(&self) -> i64 { self.nav_mesh_layer }
	pub fn set_nav_mesh_layer(&mut self, value : i64) { self.nav_mesh_layer = value; }

	pub fn get_static_editor_flags(&self) -> i64 { self.static_editor_flags }
	pub fn set_static_editor_flags(&mut self, value : i64) { self.static_editor_flags = value; }

	pub fn get_is_active(&self) -> i64 { self.is_active }
	pub fn set_is_active(&mut self, value : i64) { self.is_active = value; }

}

#[cfg(test)]
mod game_object_tests {
	use super::GameObject;
	use crate::file_types::asset_file::*;

	#[test]
	fn test_game_object() {
		let mut path = std::env::current_dir().unwrap();
		path.push("src");
		path.push("file_types");
		path.push("templates");
		path.push("GameObject.yaml");
		println!("{}", path.display());
		let subject = <dyn AssetFile>::load_asset_file_from_path::<GameObject>(&path).unwrap();

		//Tests
		assert_eq!(subject.object_hide_flags, 0);
		assert_eq!(subject.serialized_version, 5);
		assert_eq!(subject.component[0].get_file_id(), 1184336827);
		assert_eq!(subject.component[1].get_file_id(), 4696828);
		assert_eq!(subject.component[2].get_file_id(), 4696827);
		assert_eq!(subject.layer, 0);
		assert_eq!(subject.name, String::from("SM_Prop_Papers_01 (1)"));
		assert_eq!(subject.tag_string, String::from("Untagged"));
		assert_eq!(subject.nav_mesh_layer, 0);
		assert_eq!(subject.static_editor_flags, 0);
		assert_eq!(subject.is_active, 1);
	}
}


