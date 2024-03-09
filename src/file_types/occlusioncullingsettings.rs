use crate::from_yaml_trait::*;
use unity_yaml_rust::Yaml;
use crate::file_types::sections::FileReference;

#[derive(Debug, Clone, PartialEq)]
pub struct OcclusionBakeSettings {
	smallest_occluder : i64,
	smallest_hole : f64,
	backface_threshold : i64,

}

impl FromYaml for OcclusionBakeSettings {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["OcclusionBakeSettings"] != Yaml::BadValue {
			y = &yaml["OcclusionBakeSettings"];
		}

		OcclusionBakeSettings {
			smallest_occluder : y["smallestOccluder"].as_i64().expect("Could not find item smallestOccluder in OcclusionBakeSettings."),
			smallest_hole : y["smallestHole"].as_f64().expect("Could not find item smallestHole in OcclusionBakeSettings."),
			backface_threshold : y["backfaceThreshold"].as_i64().expect("Could not find item backfaceThreshold in OcclusionBakeSettings."),
		}
	}

}

impl OcclusionBakeSettings {
	pub fn get_smallest_occluder(&self) -> i64 { self.smallest_occluder }
	pub fn set_smallest_occluder(&mut self, value : i64) { self.smallest_occluder = value; }

	pub fn get_smallest_hole(&self) -> f64 { self.smallest_hole }
	pub fn set_smallest_hole(&mut self, value : f64) { self.smallest_hole = value; }

	pub fn get_backface_threshold(&self) -> i64 { self.backface_threshold }
	pub fn set_backface_threshold(&mut self, value : i64) { self.backface_threshold = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct OcclusionCullingSettings {
	object_hide_flags : i64,
	serialized_version : i64,
	occlusion_bake_settings : OcclusionBakeSettings,
	scene_g_u_i_d : i64,
	occlusion_culling_data : FileReference,

}

impl FromYaml for OcclusionCullingSettings {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["OcclusionCullingSettings"] != Yaml::BadValue {
			y = &yaml["OcclusionCullingSettings"];
		}

		OcclusionCullingSettings {
			object_hide_flags : y["m_ObjectHideFlags"].as_i64().expect("Could not find item m_ObjectHideFlags in OcclusionCullingSettings."),
			serialized_version : y["serializedVersion"].as_i64().expect("Could not find item serializedVersion in OcclusionCullingSettings."),
			occlusion_bake_settings : OcclusionBakeSettings::from_yaml(&y["m_OcclusionBakeSettings"]),
			scene_g_u_i_d : y["m_SceneGUID"].as_i64().expect("Could not find item m_SceneGUID in OcclusionCullingSettings."),
			occlusion_culling_data : FileReference::from_yaml(&y["m_OcclusionCullingData"]),
		}
	}

}

impl OcclusionCullingSettings {
	pub fn get_object_hide_flags(&self) -> i64 { self.object_hide_flags }
	pub fn set_object_hide_flags(&mut self, value : i64) { self.object_hide_flags = value; }

	pub fn get_serialized_version(&self) -> i64 { self.serialized_version }
	pub fn set_serialized_version(&mut self, value : i64) { self.serialized_version = value; }

	pub fn get_occlusion_bake_settings(&self) -> OcclusionBakeSettings { self.occlusion_bake_settings.clone() }
	pub fn set_occlusion_bake_settings(&mut self, value : OcclusionBakeSettings) { self.occlusion_bake_settings = value; }

	pub fn get_scene_g_u_i_d(&self) -> i64 { self.scene_g_u_i_d }
	pub fn set_scene_g_u_i_d(&mut self, value : i64) { self.scene_g_u_i_d = value; }

	pub fn get_occlusion_culling_data(&self) -> FileReference { self.occlusion_culling_data.clone() }
	pub fn set_occlusion_culling_data(&mut self, value : FileReference) { self.occlusion_culling_data = value; }

}

#[cfg(test)]
mod occlusion_culling_settings_tests {
	use super::OcclusionCullingSettings;
	use crate::file_types::asset_file::*;

	#[test]
	fn test_occlusion_culling_settings() {
		let mut path = std::env::current_dir().unwrap();
		path.push("src");
		path.push("file_types");
		path.push("templates");
		path.push("OcclusionCullingSettings.yaml");
		println!("{}", path.display());
		let subject = <dyn AssetFile>::load_asset_file_from_path::<OcclusionCullingSettings>(&path).unwrap();

		//Tests
		assert_eq!(subject.object_hide_flags, 0);
		assert_eq!(subject.serialized_version, 2);
		assert_eq!(subject.scene_g_u_i_d, 0);
	}
}


