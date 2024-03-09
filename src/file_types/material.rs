use std::vec::Vec;
use unity_yaml_rust::Yaml;
use crate::from_yaml_trait::*;
use crate::file_types::sections::Empty;
use crate::file_types::sections::NamedValue;
use crate::file_types::sections::Color;
use crate::file_types::sections::FileReference;
use crate::file_types::sections::Texture;

#[derive(Debug, Clone, PartialEq)]
pub struct SavedProperties {
	serialized_version : i64,
	tex_envs : Vec<Texture>,
	floats : Vec<NamedValue<f64>>,
	colors : Vec<Color>,

}

impl FromYaml for SavedProperties {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["SavedProperties"] != Yaml::BadValue {
			y = &yaml["SavedProperties"];
		}

		SavedProperties {
			serialized_version : y["serializedVersion"].as_i64().expect("Could not find item serializedVersion in SavedProperties."),
			tex_envs : read_yaml_vector(&y["m_TexEnvs"], "m_TexEnvs"),
			floats : read_yaml_vector(&y["m_Floats"], "m_Floats"),
			colors : read_yaml_vector(&y["m_Colors"], "m_Colors"),
		}
	}

}

impl SavedProperties {
	pub fn get_serialized_version(&self) -> i64 { self.serialized_version }
	pub fn set_serialized_version(&mut self, value : i64) { self.serialized_version = value; }

	pub fn get_tex_envs(&self) -> Vec<Texture> { self.tex_envs.clone() }
	pub fn set_tex_envs(&mut self, value : Vec<Texture>) { self.tex_envs = value; }

	pub fn get_floats(&self) -> Vec<NamedValue<f64>> { self.floats.clone() }
	pub fn set_floats(&mut self, value : Vec<NamedValue<f64>>) { self.floats = value; }

	pub fn get_colors(&self) -> Vec<Color> { self.colors.clone() }
	pub fn set_colors(&mut self, value : Vec<Color>) { self.colors = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
	serialized_version : i64,
	object_hide_flags : i64,
	corresponding_source_object : FileReference,
	prefab_instance : FileReference,
	prefab_asset : FileReference,
	name : String,
	shader : FileReference,
	shader_keywords : String,
	lightmap_flags : i64,
	enable_instancing_variants : i64,
	double_sided_g_i : i64,
	custom_render_queue : i64,
	string_tag_map : Empty,
	disabled_shader_passes : Vec<Empty>,
	saved_properties : SavedProperties,
	build_texture_stacks : Vec<Empty>,

}

impl FromYaml for Material {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["Material"] != Yaml::BadValue {
			y = &yaml["Material"];
		}

		Material {
			serialized_version : y["serializedVersion"].as_i64().expect("Could not find item serializedVersion in Material."),
			object_hide_flags : y["m_ObjectHideFlags"].as_i64().expect("Could not find item m_ObjectHideFlags in Material."),
			corresponding_source_object : FileReference::from_yaml(&y["m_CorrespondingSourceObject"]),
			prefab_instance : FileReference::from_yaml(&y["m_PrefabInstance"]),
			prefab_asset : FileReference::from_yaml(&y["m_PrefabAsset"]),
			name : String::from(y["m_Name"].as_str().expect("Could not find item m_Name in Material.")),
			shader : FileReference::from_yaml(&y["m_Shader"]),
			shader_keywords : String::from(y["m_ShaderKeywords"].as_str().expect("Could not find item m_ShaderKeywords in Material.")),
			lightmap_flags : y["m_LightmapFlags"].as_i64().expect("Could not find item m_LightmapFlags in Material."),
			enable_instancing_variants : y["m_EnableInstancingVariants"].as_i64().expect("Could not find item m_EnableInstancingVariants in Material."),
			double_sided_g_i : y["m_DoubleSidedGI"].as_i64().expect("Could not find item m_DoubleSidedGI in Material."),
			custom_render_queue : y["m_CustomRenderQueue"].as_i64().expect("Could not find item m_CustomRenderQueue in Material."),
			string_tag_map : Empty::from_yaml(&y["stringTagMap"]),
			disabled_shader_passes : Vec::new(),
			saved_properties : SavedProperties::from_yaml(&y["m_SavedProperties"]),
			build_texture_stacks : Vec::new(),
		}
	}

}

impl Material {
	pub fn get_serialized_version(&self) -> i64 { self.serialized_version }
	pub fn set_serialized_version(&mut self, value : i64) { self.serialized_version = value; }

	pub fn get_object_hide_flags(&self) -> i64 { self.object_hide_flags }
	pub fn set_object_hide_flags(&mut self, value : i64) { self.object_hide_flags = value; }

	pub fn get_corresponding_source_object(&self) -> FileReference { self.corresponding_source_object.clone() }
	pub fn set_corresponding_source_object(&mut self, value : FileReference) { self.corresponding_source_object = value; }

	pub fn get_prefab_instance(&self) -> FileReference { self.prefab_instance.clone() }
	pub fn set_prefab_instance(&mut self, value : FileReference) { self.prefab_instance = value; }

	pub fn get_prefab_asset(&self) -> FileReference { self.prefab_asset.clone() }
	pub fn set_prefab_asset(&mut self, value : FileReference) { self.prefab_asset = value; }

	pub fn get_name(&self) -> String { self.name.clone() }
	pub fn set_name(&mut self, value : String) { self.name = value; }

	pub fn get_shader(&self) -> FileReference { self.shader.clone() }
	pub fn set_shader(&mut self, value : FileReference) { self.shader = value; }

	pub fn get_shader_keywords(&self) -> String { self.shader_keywords.clone() }
	pub fn set_shader_keywords(&mut self, value : String) { self.shader_keywords = value; }

	pub fn get_lightmap_flags(&self) -> i64 { self.lightmap_flags }
	pub fn set_lightmap_flags(&mut self, value : i64) { self.lightmap_flags = value; }

	pub fn get_enable_instancing_variants(&self) -> i64 { self.enable_instancing_variants }
	pub fn set_enable_instancing_variants(&mut self, value : i64) { self.enable_instancing_variants = value; }

	pub fn get_double_sided_g_i(&self) -> i64 { self.double_sided_g_i }
	pub fn set_double_sided_g_i(&mut self, value : i64) { self.double_sided_g_i = value; }

	pub fn get_custom_render_queue(&self) -> i64 { self.custom_render_queue }
	pub fn set_custom_render_queue(&mut self, value : i64) { self.custom_render_queue = value; }

	pub fn get_string_tag_map(&self) -> Empty { self.string_tag_map.clone() }
	pub fn set_string_tag_map(&mut self, value : Empty) { self.string_tag_map = value; }

	pub fn get_disabled_shader_passes(&self) -> Vec<Empty> { self.disabled_shader_passes.clone() }
	pub fn set_disabled_shader_passes(&mut self, value : Vec<Empty>) { self.disabled_shader_passes = value; }

	pub fn get_saved_properties(&self) -> SavedProperties { self.saved_properties.clone() }
	pub fn set_saved_properties(&mut self, value : SavedProperties) { self.saved_properties = value; }

	pub fn get_build_texture_stacks(&self) -> Vec<Empty> { self.build_texture_stacks.clone() }
	pub fn set_build_texture_stacks(&mut self, value : Vec<Empty>) { self.build_texture_stacks = value; }

}

#[cfg(test)]
mod material_tests {
	use super::Material;
	use crate::file_types::asset_file::*;

	#[test]
	fn test_material() {
		let mut path = std::env::current_dir().unwrap();
		path.push("src");
		path.push("file_types");
		path.push("templates");
		path.push("Material.yaml");
		println!("{}", path.display());
		let subject = <dyn AssetFile>::load_asset_file_from_path::<Material>(&path).unwrap();

		//Tests
		assert_eq!(subject.serialized_version, 6);
		assert_eq!(subject.object_hide_flags, 0);
		assert_eq!(subject.name, String::from("PolygonApocalypse_Material_01_A"));
		assert_eq!(subject.shader_keywords, String::from("_NORMALMAP"));
		assert_eq!(subject.lightmap_flags, 4);
		assert_eq!(subject.enable_instancing_variants, 0);
		assert_eq!(subject.double_sided_g_i, 0);
		assert_eq!(subject.custom_render_queue, -1);
	}
}


