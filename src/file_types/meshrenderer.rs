use crate::file_types::sections::FileReference;
use crate::from_yaml_trait::*;
use std::vec::Vec;
use unity_yaml_rust::Yaml;

#[derive(Debug, Clone, PartialEq)]
pub struct StaticBatchInfo {
	first_sub_mesh : i64,
	sub_mesh_count : i64,

}

impl FromYaml for StaticBatchInfo {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["StaticBatchInfo"] != Yaml::BadValue {
			y = &yaml["StaticBatchInfo"];
		}

		StaticBatchInfo {
			first_sub_mesh : y["firstSubMesh"].as_i64().expect("Could not find item firstSubMesh in StaticBatchInfo."),
			sub_mesh_count : y["subMeshCount"].as_i64().expect("Could not find item subMeshCount in StaticBatchInfo."),
		}
	}

}

impl StaticBatchInfo {
	pub fn get_first_sub_mesh(&self) -> i64 { self.first_sub_mesh }
	pub fn set_first_sub_mesh(&mut self, value : i64) { self.first_sub_mesh = value; }

	pub fn get_sub_mesh_count(&self) -> i64 { self.sub_mesh_count }
	pub fn set_sub_mesh_count(&mut self, value : i64) { self.sub_mesh_count = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct MeshRenderer {
	object_hide_flags : i64,
	prefab_parent_object : FileReference,
	prefab_internal : FileReference,
	game_object : FileReference,
	enabled : i64,
	cast_shadows : i64,
	receive_shadows : i64,
	motion_vectors : i64,
	light_probe_usage : i64,
	reflection_probe_usage : i64,
	materials : Vec<FileReference>,
	static_batch_info : StaticBatchInfo,
	static_batch_root : FileReference,
	probe_anchor : FileReference,
	light_probe_volume_override : FileReference,
	scale_in_lightmap : i64,
	preserve_u_vs : i64,
	ignore_normals_for_chart_detection : i64,
	important_g_i : i64,
	selected_editor_render_state : i64,
	minimum_chart_size : i64,
	auto_u_v_max_distance : f64,
	auto_u_v_max_angle : i64,
	lightmap_parameters : FileReference,
	sorting_layer_i_d : i64,
	sorting_layer : i64,
	sorting_order : i64,

}

impl FromYaml for MeshRenderer {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["MeshRenderer"] != Yaml::BadValue {
			y = &yaml["MeshRenderer"];
		}

		MeshRenderer {
			object_hide_flags : y["m_ObjectHideFlags"].as_i64().expect("Could not find item m_ObjectHideFlags in MeshRenderer."),
			prefab_parent_object : FileReference::from_yaml(&y["m_PrefabParentObject"]),
			prefab_internal : FileReference::from_yaml(&y["m_PrefabInternal"]),
			game_object : FileReference::from_yaml(&y["m_GameObject"]),
			enabled : y["m_Enabled"].as_i64().expect("Could not find item m_Enabled in MeshRenderer."),
			cast_shadows : y["m_CastShadows"].as_i64().expect("Could not find item m_CastShadows in MeshRenderer."),
			receive_shadows : y["m_ReceiveShadows"].as_i64().expect("Could not find item m_ReceiveShadows in MeshRenderer."),
			motion_vectors : y["m_MotionVectors"].as_i64().expect("Could not find item m_MotionVectors in MeshRenderer."),
			light_probe_usage : y["m_LightProbeUsage"].as_i64().expect("Could not find item m_LightProbeUsage in MeshRenderer."),
			reflection_probe_usage : y["m_ReflectionProbeUsage"].as_i64().expect("Could not find item m_ReflectionProbeUsage in MeshRenderer."),
			materials : read_yaml_vector(&y["m_Materials"], "m_Materials"),
			static_batch_info : StaticBatchInfo::from_yaml(&y["m_StaticBatchInfo"]),
			static_batch_root : FileReference::from_yaml(&y["m_StaticBatchRoot"]),
			probe_anchor : FileReference::from_yaml(&y["m_ProbeAnchor"]),
			light_probe_volume_override : FileReference::from_yaml(&y["m_LightProbeVolumeOverride"]),
			scale_in_lightmap : y["m_ScaleInLightmap"].as_i64().expect("Could not find item m_ScaleInLightmap in MeshRenderer."),
			preserve_u_vs : y["m_PreserveUVs"].as_i64().expect("Could not find item m_PreserveUVs in MeshRenderer."),
			ignore_normals_for_chart_detection : y["m_IgnoreNormalsForChartDetection"].as_i64().expect("Could not find item m_IgnoreNormalsForChartDetection in MeshRenderer."),
			important_g_i : y["m_ImportantGI"].as_i64().expect("Could not find item m_ImportantGI in MeshRenderer."),
			selected_editor_render_state : y["m_SelectedEditorRenderState"].as_i64().expect("Could not find item m_SelectedEditorRenderState in MeshRenderer."),
			minimum_chart_size : y["m_MinimumChartSize"].as_i64().expect("Could not find item m_MinimumChartSize in MeshRenderer."),
			auto_u_v_max_distance : y["m_AutoUVMaxDistance"].as_f64().expect("Could not find item m_AutoUVMaxDistance in MeshRenderer."),
			auto_u_v_max_angle : y["m_AutoUVMaxAngle"].as_i64().expect("Could not find item m_AutoUVMaxAngle in MeshRenderer."),
			lightmap_parameters : FileReference::from_yaml(&y["m_LightmapParameters"]),
			sorting_layer_i_d : y["m_SortingLayerID"].as_i64().expect("Could not find item m_SortingLayerID in MeshRenderer."),
			sorting_layer : y["m_SortingLayer"].as_i64().expect("Could not find item m_SortingLayer in MeshRenderer."),
			sorting_order : y["m_SortingOrder"].as_i64().expect("Could not find item m_SortingOrder in MeshRenderer."),
		}
	}

}

impl MeshRenderer {
	pub fn get_object_hide_flags(&self) -> i64 { self.object_hide_flags }
	pub fn set_object_hide_flags(&mut self, value : i64) { self.object_hide_flags = value; }

	pub fn get_prefab_parent_object(&self) -> FileReference { self.prefab_parent_object.clone() }
	pub fn set_prefab_parent_object(&mut self, value : FileReference) { self.prefab_parent_object = value; }

	pub fn get_prefab_internal(&self) -> FileReference { self.prefab_internal.clone() }
	pub fn set_prefab_internal(&mut self, value : FileReference) { self.prefab_internal = value; }

	pub fn get_game_object(&self) -> FileReference { self.game_object.clone() }
	pub fn set_game_object(&mut self, value : FileReference) { self.game_object = value; }

	pub fn get_enabled(&self) -> i64 { self.enabled }
	pub fn set_enabled(&mut self, value : i64) { self.enabled = value; }

	pub fn get_cast_shadows(&self) -> i64 { self.cast_shadows }
	pub fn set_cast_shadows(&mut self, value : i64) { self.cast_shadows = value; }

	pub fn get_receive_shadows(&self) -> i64 { self.receive_shadows }
	pub fn set_receive_shadows(&mut self, value : i64) { self.receive_shadows = value; }

	pub fn get_motion_vectors(&self) -> i64 { self.motion_vectors }
	pub fn set_motion_vectors(&mut self, value : i64) { self.motion_vectors = value; }

	pub fn get_light_probe_usage(&self) -> i64 { self.light_probe_usage }
	pub fn set_light_probe_usage(&mut self, value : i64) { self.light_probe_usage = value; }

	pub fn get_reflection_probe_usage(&self) -> i64 { self.reflection_probe_usage }
	pub fn set_reflection_probe_usage(&mut self, value : i64) { self.reflection_probe_usage = value; }

	pub fn get_materials(&self) -> Vec<FileReference> { self.materials.clone() }
	pub fn set_materials(&mut self, value : Vec<FileReference>) { self.materials = value; }

	pub fn get_static_batch_info(&self) -> StaticBatchInfo { self.static_batch_info.clone() }
	pub fn set_static_batch_info(&mut self, value : StaticBatchInfo) { self.static_batch_info = value; }

	pub fn get_static_batch_root(&self) -> FileReference { self.static_batch_root.clone() }
	pub fn set_static_batch_root(&mut self, value : FileReference) { self.static_batch_root = value; }

	pub fn get_probe_anchor(&self) -> FileReference { self.probe_anchor.clone() }
	pub fn set_probe_anchor(&mut self, value : FileReference) { self.probe_anchor = value; }

	pub fn get_light_probe_volume_override(&self) -> FileReference { self.light_probe_volume_override.clone() }
	pub fn set_light_probe_volume_override(&mut self, value : FileReference) { self.light_probe_volume_override = value; }

	pub fn get_scale_in_lightmap(&self) -> i64 { self.scale_in_lightmap }
	pub fn set_scale_in_lightmap(&mut self, value : i64) { self.scale_in_lightmap = value; }

	pub fn get_preserve_u_vs(&self) -> i64 { self.preserve_u_vs }
	pub fn set_preserve_u_vs(&mut self, value : i64) { self.preserve_u_vs = value; }

	pub fn get_ignore_normals_for_chart_detection(&self) -> i64 { self.ignore_normals_for_chart_detection }
	pub fn set_ignore_normals_for_chart_detection(&mut self, value : i64) { self.ignore_normals_for_chart_detection = value; }

	pub fn get_important_g_i(&self) -> i64 { self.important_g_i }
	pub fn set_important_g_i(&mut self, value : i64) { self.important_g_i = value; }

	pub fn get_selected_editor_render_state(&self) -> i64 { self.selected_editor_render_state }
	pub fn set_selected_editor_render_state(&mut self, value : i64) { self.selected_editor_render_state = value; }

	pub fn get_minimum_chart_size(&self) -> i64 { self.minimum_chart_size }
	pub fn set_minimum_chart_size(&mut self, value : i64) { self.minimum_chart_size = value; }

	pub fn get_auto_u_v_max_distance(&self) -> f64 { self.auto_u_v_max_distance }
	pub fn set_auto_u_v_max_distance(&mut self, value : f64) { self.auto_u_v_max_distance = value; }

	pub fn get_auto_u_v_max_angle(&self) -> i64 { self.auto_u_v_max_angle }
	pub fn set_auto_u_v_max_angle(&mut self, value : i64) { self.auto_u_v_max_angle = value; }

	pub fn get_lightmap_parameters(&self) -> FileReference { self.lightmap_parameters.clone() }
	pub fn set_lightmap_parameters(&mut self, value : FileReference) { self.lightmap_parameters = value; }

	pub fn get_sorting_layer_i_d(&self) -> i64 { self.sorting_layer_i_d }
	pub fn set_sorting_layer_i_d(&mut self, value : i64) { self.sorting_layer_i_d = value; }

	pub fn get_sorting_layer(&self) -> i64 { self.sorting_layer }
	pub fn set_sorting_layer(&mut self, value : i64) { self.sorting_layer = value; }

	pub fn get_sorting_order(&self) -> i64 { self.sorting_order }
	pub fn set_sorting_order(&mut self, value : i64) { self.sorting_order = value; }

}

#[cfg(test)]
mod mesh_renderer_tests {
	use super::MeshRenderer;
	use crate::file_types::asset_file::*;

	#[test]
	fn test_mesh_renderer() {
		let mut path = std::env::current_dir().unwrap();
		path.push("src");
		path.push("file_types");
		path.push("templates");
		path.push("MeshRenderer.yaml");
		println!("{}", path.display());
		let subject = <dyn AssetFile>::load_asset_file_from_path::<MeshRenderer>(&path).unwrap();

		//Tests
		assert_eq!(subject.object_hide_flags, 0);
		assert_eq!(subject.enabled, 1);
		assert_eq!(subject.cast_shadows, 1);
		assert_eq!(subject.receive_shadows, 1);
		assert_eq!(subject.motion_vectors, 1);
		assert_eq!(subject.light_probe_usage, 1);
		assert_eq!(subject.reflection_probe_usage, 1);
		assert_eq!(subject.materials[0].get_file_id(), 2100000);
		assert_eq!(subject.scale_in_lightmap, 1);
		assert_eq!(subject.preserve_u_vs, 0);
		assert_eq!(subject.ignore_normals_for_chart_detection, 0);
		assert_eq!(subject.important_g_i, 0);
		assert_eq!(subject.selected_editor_render_state, 3);
		assert_eq!(subject.minimum_chart_size, 4);
		assert_eq!(subject.auto_u_v_max_distance, 0.5);
		assert_eq!(subject.auto_u_v_max_angle, 89);
		assert_eq!(subject.sorting_layer_i_d, 0);
		assert_eq!(subject.sorting_layer, 0);
		assert_eq!(subject.sorting_order, 0);
	}
}


