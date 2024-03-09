use crate::file_types::sections::Color;
use crate::file_types::sections::FileReference;
use crate::from_yaml_trait::*;
use unity_yaml_rust::Yaml;

#[derive(Debug, Clone, PartialEq)]
pub struct RenderSettings {
	object_hide_flags : i64,
	serialized_version : i64,
	fog : i64,
	fog_color : Color,
	fog_mode : i64,
	fog_density : f64,
	linear_fog_start : i64,
	linear_fog_end : i64,
	ambient_sky_color : Color,
	ambient_equator_color : Color,
	ambient_ground_color : Color,
	ambient_intensity : f64,
	ambient_mode : i64,
	subtractive_shadow_color : Color,
	skybox_material : FileReference,
	halo_strength : f64,
	flare_strength : i64,
	flare_fade_speed : i64,
	halo_texture : FileReference,
	spot_cookie : FileReference,
	default_reflection_mode : i64,
	default_reflection_resolution : i64,
	reflection_bounces : i64,
	reflection_intensity : i64,
	custom_reflection : FileReference,
	sun : FileReference,
	indirect_specular_color : Color,

}

impl FromYaml for RenderSettings {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["RenderSettings"] != Yaml::BadValue {
			y = &yaml["RenderSettings"];
		}

		RenderSettings {
			object_hide_flags : y["m_ObjectHideFlags"].as_i64().expect("Could not find item m_ObjectHideFlags in RenderSettings."),
			serialized_version : y["serializedVersion"].as_i64().expect("Could not find item serializedVersion in RenderSettings."),
			fog : y["m_Fog"].as_i64().expect("Could not find item m_Fog in RenderSettings."),
			fog_color : Color::from_yaml(&y["m_FogColor"]),
			fog_mode : y["m_FogMode"].as_i64().expect("Could not find item m_FogMode in RenderSettings."),
			fog_density : y["m_FogDensity"].as_f64().expect("Could not find item m_FogDensity in RenderSettings."),
			linear_fog_start : y["m_LinearFogStart"].as_i64().expect("Could not find item m_LinearFogStart in RenderSettings."),
			linear_fog_end : y["m_LinearFogEnd"].as_i64().expect("Could not find item m_LinearFogEnd in RenderSettings."),
			ambient_sky_color : Color::from_yaml(&y["m_AmbientSkyColor"]),
			ambient_equator_color : Color::from_yaml(&y["m_AmbientEquatorColor"]),
			ambient_ground_color : Color::from_yaml(&y["m_AmbientGroundColor"]),
			ambient_intensity : y["m_AmbientIntensity"].as_f64().expect("Could not find item m_AmbientIntensity in RenderSettings."),
			ambient_mode : y["m_AmbientMode"].as_i64().expect("Could not find item m_AmbientMode in RenderSettings."),
			subtractive_shadow_color : Color::from_yaml(&y["m_SubtractiveShadowColor"]),
			skybox_material : FileReference::from_yaml(&y["m_SkyboxMaterial"]),
			halo_strength : y["m_HaloStrength"].as_f64().expect("Could not find item m_HaloStrength in RenderSettings."),
			flare_strength : y["m_FlareStrength"].as_i64().expect("Could not find item m_FlareStrength in RenderSettings."),
			flare_fade_speed : y["m_FlareFadeSpeed"].as_i64().expect("Could not find item m_FlareFadeSpeed in RenderSettings."),
			halo_texture : FileReference::from_yaml(&y["m_HaloTexture"]),
			spot_cookie : FileReference::from_yaml(&y["m_SpotCookie"]),
			default_reflection_mode : y["m_DefaultReflectionMode"].as_i64().expect("Could not find item m_DefaultReflectionMode in RenderSettings."),
			default_reflection_resolution : y["m_DefaultReflectionResolution"].as_i64().expect("Could not find item m_DefaultReflectionResolution in RenderSettings."),
			reflection_bounces : y["m_ReflectionBounces"].as_i64().expect("Could not find item m_ReflectionBounces in RenderSettings."),
			reflection_intensity : y["m_ReflectionIntensity"].as_i64().expect("Could not find item m_ReflectionIntensity in RenderSettings."),
			custom_reflection : FileReference::from_yaml(&y["m_CustomReflection"]),
			sun : FileReference::from_yaml(&y["m_Sun"]),
			indirect_specular_color : Color::from_yaml(&y["m_IndirectSpecularColor"]),
		}
	}

}

impl RenderSettings {
	pub fn get_object_hide_flags(&self) -> i64 { self.object_hide_flags }
	pub fn set_object_hide_flags(&mut self, value : i64) { self.object_hide_flags = value; }

	pub fn get_serialized_version(&self) -> i64 { self.serialized_version }
	pub fn set_serialized_version(&mut self, value : i64) { self.serialized_version = value; }

	pub fn get_fog(&self) -> i64 { self.fog }
	pub fn set_fog(&mut self, value : i64) { self.fog = value; }

	pub fn get_fog_color(&self) -> Color { self.fog_color.clone() }
	pub fn set_fog_color(&mut self, value : Color) { self.fog_color = value; }

	pub fn get_fog_mode(&self) -> i64 { self.fog_mode }
	pub fn set_fog_mode(&mut self, value : i64) { self.fog_mode = value; }

	pub fn get_fog_density(&self) -> f64 { self.fog_density }
	pub fn set_fog_density(&mut self, value : f64) { self.fog_density = value; }

	pub fn get_linear_fog_start(&self) -> i64 { self.linear_fog_start }
	pub fn set_linear_fog_start(&mut self, value : i64) { self.linear_fog_start = value; }

	pub fn get_linear_fog_end(&self) -> i64 { self.linear_fog_end }
	pub fn set_linear_fog_end(&mut self, value : i64) { self.linear_fog_end = value; }

	pub fn get_ambient_sky_color(&self) -> Color { self.ambient_sky_color.clone() }
	pub fn set_ambient_sky_color(&mut self, value : Color) { self.ambient_sky_color = value; }

	pub fn get_ambient_equator_color(&self) -> Color { self.ambient_equator_color.clone() }
	pub fn set_ambient_equator_color(&mut self, value : Color) { self.ambient_equator_color = value; }

	pub fn get_ambient_ground_color(&self) -> Color { self.ambient_ground_color.clone() }
	pub fn set_ambient_ground_color(&mut self, value : Color) { self.ambient_ground_color = value; }

	pub fn get_ambient_intensity(&self) -> f64 { self.ambient_intensity }
	pub fn set_ambient_intensity(&mut self, value : f64) { self.ambient_intensity = value; }

	pub fn get_ambient_mode(&self) -> i64 { self.ambient_mode }
	pub fn set_ambient_mode(&mut self, value : i64) { self.ambient_mode = value; }

	pub fn get_subtractive_shadow_color(&self) -> Color { self.subtractive_shadow_color.clone() }
	pub fn set_subtractive_shadow_color(&mut self, value : Color) { self.subtractive_shadow_color = value; }

	pub fn get_skybox_material(&self) -> FileReference { self.skybox_material.clone() }
	pub fn set_skybox_material(&mut self, value : FileReference) { self.skybox_material = value; }

	pub fn get_halo_strength(&self) -> f64 { self.halo_strength }
	pub fn set_halo_strength(&mut self, value : f64) { self.halo_strength = value; }

	pub fn get_flare_strength(&self) -> i64 { self.flare_strength }
	pub fn set_flare_strength(&mut self, value : i64) { self.flare_strength = value; }

	pub fn get_flare_fade_speed(&self) -> i64 { self.flare_fade_speed }
	pub fn set_flare_fade_speed(&mut self, value : i64) { self.flare_fade_speed = value; }

	pub fn get_halo_texture(&self) -> FileReference { self.halo_texture.clone() }
	pub fn set_halo_texture(&mut self, value : FileReference) { self.halo_texture = value; }

	pub fn get_spot_cookie(&self) -> FileReference { self.spot_cookie.clone() }
	pub fn set_spot_cookie(&mut self, value : FileReference) { self.spot_cookie = value; }

	pub fn get_default_reflection_mode(&self) -> i64 { self.default_reflection_mode }
	pub fn set_default_reflection_mode(&mut self, value : i64) { self.default_reflection_mode = value; }

	pub fn get_default_reflection_resolution(&self) -> i64 { self.default_reflection_resolution }
	pub fn set_default_reflection_resolution(&mut self, value : i64) { self.default_reflection_resolution = value; }

	pub fn get_reflection_bounces(&self) -> i64 { self.reflection_bounces }
	pub fn set_reflection_bounces(&mut self, value : i64) { self.reflection_bounces = value; }

	pub fn get_reflection_intensity(&self) -> i64 { self.reflection_intensity }
	pub fn set_reflection_intensity(&mut self, value : i64) { self.reflection_intensity = value; }

	pub fn get_custom_reflection(&self) -> FileReference { self.custom_reflection.clone() }
	pub fn set_custom_reflection(&mut self, value : FileReference) { self.custom_reflection = value; }

	pub fn get_sun(&self) -> FileReference { self.sun.clone() }
	pub fn set_sun(&mut self, value : FileReference) { self.sun = value; }

	pub fn get_indirect_specular_color(&self) -> Color { self.indirect_specular_color.clone() }
	pub fn set_indirect_specular_color(&mut self, value : Color) { self.indirect_specular_color = value; }

}

#[cfg(test)]
mod render_settings_tests {
	use super::RenderSettings;
	use crate::file_types::asset_file::*;

	#[test]
	fn test_render_settings() {
		let mut path = std::env::current_dir().unwrap();
		path.push("src");
		path.push("file_types");
		path.push("templates");
		path.push("RenderSettings.yaml");
		println!("{}", path.display());
		let subject = <dyn AssetFile>::load_asset_file_from_path::<RenderSettings>(&path).unwrap();

		//Tests
		assert_eq!(subject.object_hide_flags, 0);
		assert_eq!(subject.serialized_version, 8);
		assert_eq!(subject.fog, 0);
		assert_eq!(subject.fog_mode, 3);
		assert_eq!(subject.fog_density, 0.01);
		assert_eq!(subject.linear_fog_start, 0);
		assert_eq!(subject.linear_fog_end, 300);
		assert_eq!(subject.ambient_intensity, 0.78);
		assert_eq!(subject.ambient_mode, 3);
		assert_eq!(subject.halo_strength, 0.5);
		assert_eq!(subject.flare_strength, 1);
		assert_eq!(subject.flare_fade_speed, 3);
		assert_eq!(subject.default_reflection_mode, 0);
		assert_eq!(subject.default_reflection_resolution, 128);
		assert_eq!(subject.reflection_bounces, 1);
		assert_eq!(subject.reflection_intensity, 1);
	}
}


