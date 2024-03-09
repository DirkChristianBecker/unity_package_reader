use crate::from_yaml_trait::*;
use crate::file_types::sections::FileReference;
use unity_yaml_rust::Yaml;

#[derive(Debug, Clone, PartialEq)]
pub struct GISettings {
	serialized_version : i64,
	bounce_scale : i64,
	indirect_output_scale : i64,
	albedo_boost : i64,
	temporal_coherence_threshold : i64,
	environment_lighting_mode : i64,
	enable_baked_lightmaps : i64,
	enable_realtime_lightmaps : i64,

}

impl FromYaml for GISettings {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["GISettings"] != Yaml::BadValue {
			y = &yaml["GISettings"];
		}

		GISettings {
			serialized_version : y["serializedVersion"].as_i64().expect("Could not find item serializedVersion in GISettings."),
			bounce_scale : y["m_BounceScale"].as_i64().expect("Could not find item m_BounceScale in GISettings."),
			indirect_output_scale : y["m_IndirectOutputScale"].as_i64().expect("Could not find item m_IndirectOutputScale in GISettings."),
			albedo_boost : y["m_AlbedoBoost"].as_i64().expect("Could not find item m_AlbedoBoost in GISettings."),
			temporal_coherence_threshold : y["m_TemporalCoherenceThreshold"].as_i64().expect("Could not find item m_TemporalCoherenceThreshold in GISettings."),
			environment_lighting_mode : y["m_EnvironmentLightingMode"].as_i64().expect("Could not find item m_EnvironmentLightingMode in GISettings."),
			enable_baked_lightmaps : y["m_EnableBakedLightmaps"].as_i64().expect("Could not find item m_EnableBakedLightmaps in GISettings."),
			enable_realtime_lightmaps : y["m_EnableRealtimeLightmaps"].as_i64().expect("Could not find item m_EnableRealtimeLightmaps in GISettings."),
		}
	}

}

impl GISettings {
	pub fn get_serialized_version(&self) -> i64 { self.serialized_version }
	pub fn set_serialized_version(&mut self, value : i64) { self.serialized_version = value; }

	pub fn get_bounce_scale(&self) -> i64 { self.bounce_scale }
	pub fn set_bounce_scale(&mut self, value : i64) { self.bounce_scale = value; }

	pub fn get_indirect_output_scale(&self) -> i64 { self.indirect_output_scale }
	pub fn set_indirect_output_scale(&mut self, value : i64) { self.indirect_output_scale = value; }

	pub fn get_albedo_boost(&self) -> i64 { self.albedo_boost }
	pub fn set_albedo_boost(&mut self, value : i64) { self.albedo_boost = value; }

	pub fn get_temporal_coherence_threshold(&self) -> i64 { self.temporal_coherence_threshold }
	pub fn set_temporal_coherence_threshold(&mut self, value : i64) { self.temporal_coherence_threshold = value; }

	pub fn get_environment_lighting_mode(&self) -> i64 { self.environment_lighting_mode }
	pub fn set_environment_lighting_mode(&mut self, value : i64) { self.environment_lighting_mode = value; }

	pub fn get_enable_baked_lightmaps(&self) -> i64 { self.enable_baked_lightmaps }
	pub fn set_enable_baked_lightmaps(&mut self, value : i64) { self.enable_baked_lightmaps = value; }

	pub fn get_enable_realtime_lightmaps(&self) -> i64 { self.enable_realtime_lightmaps }
	pub fn set_enable_realtime_lightmaps(&mut self, value : i64) { self.enable_realtime_lightmaps = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct LightmapEditorSettings {
	serialized_version : i64,
	resolution : i64,
	bake_resolution : i64,
	texture_width : i64,
	texture_height : i64,
	a_o : i64,
	a_o_max_distance : i64,
	comp_a_o_exponent : i64,
	comp_a_o_exponent_direct : i64,
	padding : i64,
	lightmap_parameters : FileReference,
	lightmaps_bake_mode : i64,
	texture_compression : i64,
	final_gather : i64,
	final_gather_filtering : i64,
	final_gather_ray_count : i64,
	reflection_compression : i64,
	mixed_bake_mode : i64,
	bake_backend : i64,
	p_v_r_sampling : i64,
	p_v_r_direct_sample_count : i64,
	p_v_r_sample_count : i64,
	p_v_r_bounces : i64,
	p_v_r_filtering : i64,
	p_v_r_filtering_mode : i64,
	p_v_r_culling : i64,
	p_v_r_filtering_gauss_radius_direct : i64,
	p_v_r_filtering_gauss_radius_indirect : i64,
	p_v_r_filtering_gauss_radius_a_o : i64,
	p_v_r_filtering_atrous_color_sigma : i64,
	p_v_r_filtering_atrous_normal_sigma : i64,
	p_v_r_filtering_atrous_position_sigma : i64,

}

impl FromYaml for LightmapEditorSettings {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["LightmapEditorSettings"] != Yaml::BadValue {
			y = &yaml["LightmapEditorSettings"];
		}

		LightmapEditorSettings {
			serialized_version : y["serializedVersion"].as_i64().expect("Could not find item serializedVersion in LightmapEditorSettings."),
			resolution : y["m_Resolution"].as_i64().expect("Could not find item m_Resolution in LightmapEditorSettings."),
			bake_resolution : y["m_BakeResolution"].as_i64().expect("Could not find item m_BakeResolution in LightmapEditorSettings."),
			texture_width : y["m_TextureWidth"].as_i64().expect("Could not find item m_TextureWidth in LightmapEditorSettings."),
			texture_height : y["m_TextureHeight"].as_i64().expect("Could not find item m_TextureHeight in LightmapEditorSettings."),
			a_o : y["m_AO"].as_i64().expect("Could not find item m_AO in LightmapEditorSettings."),
			a_o_max_distance : y["m_AOMaxDistance"].as_i64().expect("Could not find item m_AOMaxDistance in LightmapEditorSettings."),
			comp_a_o_exponent : y["m_CompAOExponent"].as_i64().expect("Could not find item m_CompAOExponent in LightmapEditorSettings."),
			comp_a_o_exponent_direct : y["m_CompAOExponentDirect"].as_i64().expect("Could not find item m_CompAOExponentDirect in LightmapEditorSettings."),
			padding : y["m_Padding"].as_i64().expect("Could not find item m_Padding in LightmapEditorSettings."),
			lightmap_parameters : FileReference::from_yaml(&y["m_LightmapParameters"]),
			lightmaps_bake_mode : y["m_LightmapsBakeMode"].as_i64().expect("Could not find item m_LightmapsBakeMode in LightmapEditorSettings."),
			texture_compression : y["m_TextureCompression"].as_i64().expect("Could not find item m_TextureCompression in LightmapEditorSettings."),
			final_gather : y["m_FinalGather"].as_i64().expect("Could not find item m_FinalGather in LightmapEditorSettings."),
			final_gather_filtering : y["m_FinalGatherFiltering"].as_i64().expect("Could not find item m_FinalGatherFiltering in LightmapEditorSettings."),
			final_gather_ray_count : y["m_FinalGatherRayCount"].as_i64().expect("Could not find item m_FinalGatherRayCount in LightmapEditorSettings."),
			reflection_compression : y["m_ReflectionCompression"].as_i64().expect("Could not find item m_ReflectionCompression in LightmapEditorSettings."),
			mixed_bake_mode : y["m_MixedBakeMode"].as_i64().expect("Could not find item m_MixedBakeMode in LightmapEditorSettings."),
			bake_backend : y["m_BakeBackend"].as_i64().expect("Could not find item m_BakeBackend in LightmapEditorSettings."),
			p_v_r_sampling : y["m_PVRSampling"].as_i64().expect("Could not find item m_PVRSampling in LightmapEditorSettings."),
			p_v_r_direct_sample_count : y["m_PVRDirectSampleCount"].as_i64().expect("Could not find item m_PVRDirectSampleCount in LightmapEditorSettings."),
			p_v_r_sample_count : y["m_PVRSampleCount"].as_i64().expect("Could not find item m_PVRSampleCount in LightmapEditorSettings."),
			p_v_r_bounces : y["m_PVRBounces"].as_i64().expect("Could not find item m_PVRBounces in LightmapEditorSettings."),
			p_v_r_filtering : y["m_PVRFiltering"].as_i64().expect("Could not find item m_PVRFiltering in LightmapEditorSettings."),
			p_v_r_filtering_mode : y["m_PVRFilteringMode"].as_i64().expect("Could not find item m_PVRFilteringMode in LightmapEditorSettings."),
			p_v_r_culling : y["m_PVRCulling"].as_i64().expect("Could not find item m_PVRCulling in LightmapEditorSettings."),
			p_v_r_filtering_gauss_radius_direct : y["m_PVRFilteringGaussRadiusDirect"].as_i64().expect("Could not find item m_PVRFilteringGaussRadiusDirect in LightmapEditorSettings."),
			p_v_r_filtering_gauss_radius_indirect : y["m_PVRFilteringGaussRadiusIndirect"].as_i64().expect("Could not find item m_PVRFilteringGaussRadiusIndirect in LightmapEditorSettings."),
			p_v_r_filtering_gauss_radius_a_o : y["m_PVRFilteringGaussRadiusAO"].as_i64().expect("Could not find item m_PVRFilteringGaussRadiusAO in LightmapEditorSettings."),
			p_v_r_filtering_atrous_color_sigma : y["m_PVRFilteringAtrousColorSigma"].as_i64().expect("Could not find item m_PVRFilteringAtrousColorSigma in LightmapEditorSettings."),
			p_v_r_filtering_atrous_normal_sigma : y["m_PVRFilteringAtrousNormalSigma"].as_i64().expect("Could not find item m_PVRFilteringAtrousNormalSigma in LightmapEditorSettings."),
			p_v_r_filtering_atrous_position_sigma : y["m_PVRFilteringAtrousPositionSigma"].as_i64().expect("Could not find item m_PVRFilteringAtrousPositionSigma in LightmapEditorSettings."),
		}
	}

}

impl LightmapEditorSettings {
	pub fn get_serialized_version(&self) -> i64 { self.serialized_version }
	pub fn set_serialized_version(&mut self, value : i64) { self.serialized_version = value; }

	pub fn get_resolution(&self) -> i64 { self.resolution }
	pub fn set_resolution(&mut self, value : i64) { self.resolution = value; }

	pub fn get_bake_resolution(&self) -> i64 { self.bake_resolution }
	pub fn set_bake_resolution(&mut self, value : i64) { self.bake_resolution = value; }

	pub fn get_texture_width(&self) -> i64 { self.texture_width }
	pub fn set_texture_width(&mut self, value : i64) { self.texture_width = value; }

	pub fn get_texture_height(&self) -> i64 { self.texture_height }
	pub fn set_texture_height(&mut self, value : i64) { self.texture_height = value; }

	pub fn get_a_o(&self) -> i64 { self.a_o }
	pub fn set_a_o(&mut self, value : i64) { self.a_o = value; }

	pub fn get_a_o_max_distance(&self) -> i64 { self.a_o_max_distance }
	pub fn set_a_o_max_distance(&mut self, value : i64) { self.a_o_max_distance = value; }

	pub fn get_comp_a_o_exponent(&self) -> i64 { self.comp_a_o_exponent }
	pub fn set_comp_a_o_exponent(&mut self, value : i64) { self.comp_a_o_exponent = value; }

	pub fn get_comp_a_o_exponent_direct(&self) -> i64 { self.comp_a_o_exponent_direct }
	pub fn set_comp_a_o_exponent_direct(&mut self, value : i64) { self.comp_a_o_exponent_direct = value; }

	pub fn get_padding(&self) -> i64 { self.padding }
	pub fn set_padding(&mut self, value : i64) { self.padding = value; }

	pub fn get_lightmap_parameters(&self) -> FileReference { self.lightmap_parameters.clone() }
	pub fn set_lightmap_parameters(&mut self, value : FileReference) { self.lightmap_parameters = value; }

	pub fn get_lightmaps_bake_mode(&self) -> i64 { self.lightmaps_bake_mode }
	pub fn set_lightmaps_bake_mode(&mut self, value : i64) { self.lightmaps_bake_mode = value; }

	pub fn get_texture_compression(&self) -> i64 { self.texture_compression }
	pub fn set_texture_compression(&mut self, value : i64) { self.texture_compression = value; }

	pub fn get_final_gather(&self) -> i64 { self.final_gather }
	pub fn set_final_gather(&mut self, value : i64) { self.final_gather = value; }

	pub fn get_final_gather_filtering(&self) -> i64 { self.final_gather_filtering }
	pub fn set_final_gather_filtering(&mut self, value : i64) { self.final_gather_filtering = value; }

	pub fn get_final_gather_ray_count(&self) -> i64 { self.final_gather_ray_count }
	pub fn set_final_gather_ray_count(&mut self, value : i64) { self.final_gather_ray_count = value; }

	pub fn get_reflection_compression(&self) -> i64 { self.reflection_compression }
	pub fn set_reflection_compression(&mut self, value : i64) { self.reflection_compression = value; }

	pub fn get_mixed_bake_mode(&self) -> i64 { self.mixed_bake_mode }
	pub fn set_mixed_bake_mode(&mut self, value : i64) { self.mixed_bake_mode = value; }

	pub fn get_bake_backend(&self) -> i64 { self.bake_backend }
	pub fn set_bake_backend(&mut self, value : i64) { self.bake_backend = value; }

	pub fn get_p_v_r_sampling(&self) -> i64 { self.p_v_r_sampling }
	pub fn set_p_v_r_sampling(&mut self, value : i64) { self.p_v_r_sampling = value; }

	pub fn get_p_v_r_direct_sample_count(&self) -> i64 { self.p_v_r_direct_sample_count }
	pub fn set_p_v_r_direct_sample_count(&mut self, value : i64) { self.p_v_r_direct_sample_count = value; }

	pub fn get_p_v_r_sample_count(&self) -> i64 { self.p_v_r_sample_count }
	pub fn set_p_v_r_sample_count(&mut self, value : i64) { self.p_v_r_sample_count = value; }

	pub fn get_p_v_r_bounces(&self) -> i64 { self.p_v_r_bounces }
	pub fn set_p_v_r_bounces(&mut self, value : i64) { self.p_v_r_bounces = value; }

	pub fn get_p_v_r_filtering(&self) -> i64 { self.p_v_r_filtering }
	pub fn set_p_v_r_filtering(&mut self, value : i64) { self.p_v_r_filtering = value; }

	pub fn get_p_v_r_filtering_mode(&self) -> i64 { self.p_v_r_filtering_mode }
	pub fn set_p_v_r_filtering_mode(&mut self, value : i64) { self.p_v_r_filtering_mode = value; }

	pub fn get_p_v_r_culling(&self) -> i64 { self.p_v_r_culling }
	pub fn set_p_v_r_culling(&mut self, value : i64) { self.p_v_r_culling = value; }

	pub fn get_p_v_r_filtering_gauss_radius_direct(&self) -> i64 { self.p_v_r_filtering_gauss_radius_direct }
	pub fn set_p_v_r_filtering_gauss_radius_direct(&mut self, value : i64) { self.p_v_r_filtering_gauss_radius_direct = value; }

	pub fn get_p_v_r_filtering_gauss_radius_indirect(&self) -> i64 { self.p_v_r_filtering_gauss_radius_indirect }
	pub fn set_p_v_r_filtering_gauss_radius_indirect(&mut self, value : i64) { self.p_v_r_filtering_gauss_radius_indirect = value; }

	pub fn get_p_v_r_filtering_gauss_radius_a_o(&self) -> i64 { self.p_v_r_filtering_gauss_radius_a_o }
	pub fn set_p_v_r_filtering_gauss_radius_a_o(&mut self, value : i64) { self.p_v_r_filtering_gauss_radius_a_o = value; }

	pub fn get_p_v_r_filtering_atrous_color_sigma(&self) -> i64 { self.p_v_r_filtering_atrous_color_sigma }
	pub fn set_p_v_r_filtering_atrous_color_sigma(&mut self, value : i64) { self.p_v_r_filtering_atrous_color_sigma = value; }

	pub fn get_p_v_r_filtering_atrous_normal_sigma(&self) -> i64 { self.p_v_r_filtering_atrous_normal_sigma }
	pub fn set_p_v_r_filtering_atrous_normal_sigma(&mut self, value : i64) { self.p_v_r_filtering_atrous_normal_sigma = value; }

	pub fn get_p_v_r_filtering_atrous_position_sigma(&self) -> i64 { self.p_v_r_filtering_atrous_position_sigma }
	pub fn set_p_v_r_filtering_atrous_position_sigma(&mut self, value : i64) { self.p_v_r_filtering_atrous_position_sigma = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct LightmapSettings {
	object_hide_flags : i64,
	serialized_version : i64,
	g_i_workflow_mode : i64,
	g_i_settings : GISettings,
	lightmap_editor_settings : LightmapEditorSettings,
	lighting_data_asset : FileReference,
	use_shadowmask : i64,

}

impl FromYaml for LightmapSettings {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["LightmapSettings"] != Yaml::BadValue {
			y = &yaml["LightmapSettings"];
		}

		LightmapSettings {
			object_hide_flags : y["m_ObjectHideFlags"].as_i64().expect("Could not find item m_ObjectHideFlags in LightmapSettings."),
			serialized_version : y["serializedVersion"].as_i64().expect("Could not find item serializedVersion in LightmapSettings."),
			g_i_workflow_mode : y["m_GIWorkflowMode"].as_i64().expect("Could not find item m_GIWorkflowMode in LightmapSettings."),
			g_i_settings : GISettings::from_yaml(&y["m_GISettings"]),
			lightmap_editor_settings : LightmapEditorSettings::from_yaml(&y["m_LightmapEditorSettings"]),
			lighting_data_asset : FileReference::from_yaml(&y["m_LightingDataAsset"]),
			use_shadowmask : y["m_UseShadowmask"].as_i64().expect("Could not find item m_UseShadowmask in LightmapSettings."),
		}
	}

}

impl LightmapSettings {
	pub fn get_object_hide_flags(&self) -> i64 { self.object_hide_flags }
	pub fn set_object_hide_flags(&mut self, value : i64) { self.object_hide_flags = value; }

	pub fn get_serialized_version(&self) -> i64 { self.serialized_version }
	pub fn set_serialized_version(&mut self, value : i64) { self.serialized_version = value; }

	pub fn get_g_i_workflow_mode(&self) -> i64 { self.g_i_workflow_mode }
	pub fn set_g_i_workflow_mode(&mut self, value : i64) { self.g_i_workflow_mode = value; }

	pub fn get_g_i_settings(&self) -> GISettings { self.g_i_settings.clone() }
	pub fn set_g_i_settings(&mut self, value : GISettings) { self.g_i_settings = value; }

	pub fn get_lightmap_editor_settings(&self) -> LightmapEditorSettings { self.lightmap_editor_settings.clone() }
	pub fn set_lightmap_editor_settings(&mut self, value : LightmapEditorSettings) { self.lightmap_editor_settings = value; }

	pub fn get_lighting_data_asset(&self) -> FileReference { self.lighting_data_asset.clone() }
	pub fn set_lighting_data_asset(&mut self, value : FileReference) { self.lighting_data_asset = value; }

	pub fn get_use_shadowmask(&self) -> i64 { self.use_shadowmask }
	pub fn set_use_shadowmask(&mut self, value : i64) { self.use_shadowmask = value; }

}

#[cfg(test)]
mod lightmap_settings_tests {
	use super::LightmapSettings;
	use crate::file_types::asset_file::*;

	#[test]
	fn test_lightmap_settings() {
		let mut path = std::env::current_dir().unwrap();
		path.push("src");
		path.push("file_types");
		path.push("templates");
		path.push("LightmapSettings.yaml");
		println!("{}", path.display());
		let subject = <dyn AssetFile>::load_asset_file_from_path::<LightmapSettings>(&path).unwrap();

		//Tests
		assert_eq!(subject.object_hide_flags, 0);
		assert_eq!(subject.serialized_version, 11);
		assert_eq!(subject.g_i_workflow_mode, 0);
		assert_eq!(subject.use_shadowmask, 1);
	}
}


