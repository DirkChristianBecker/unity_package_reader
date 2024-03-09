use crate::from_yaml_trait::*;
use crate::file_types::sections::FalloffTable;
use crate::file_types::sections::FileReference;
use unity_yaml_rust::Yaml;
use crate::file_types::sections::Color;
use crate::file_types::sections::Vec2;

#[derive(Debug, Clone, PartialEq)]
pub struct Shadows {
	shadows_type : i64,
	resolution : i64,
	custom_resolution : i64,
	strength : i64,
	bias : f64,
	normal_bias : f64,
	near_plane : f64,

}

impl FromYaml for Shadows {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["Shadows"] != Yaml::BadValue {
			y = &yaml["Shadows"];
		}

		Shadows {
			shadows_type : y["m_Type"].as_i64().expect("Could not find item m_Type in Shadows."),
			resolution : y["m_Resolution"].as_i64().expect("Could not find item m_Resolution in Shadows."),
			custom_resolution : y["m_CustomResolution"].as_i64().expect("Could not find item m_CustomResolution in Shadows."),
			strength : y["m_Strength"].as_i64().expect("Could not find item m_Strength in Shadows."),
			bias : y["m_Bias"].as_f64().expect("Could not find item m_Bias in Shadows."),
			normal_bias : y["m_NormalBias"].as_f64().expect("Could not find item m_NormalBias in Shadows."),
			near_plane : y["m_NearPlane"].as_f64().expect("Could not find item m_NearPlane in Shadows."),
		}
	}

}

impl Shadows {
	pub fn get_shadows_type(&self) -> i64 { self.shadows_type }
	pub fn set_shadows_type(&mut self, value : i64) { self.shadows_type = value; }

	pub fn get_resolution(&self) -> i64 { self.resolution }
	pub fn set_resolution(&mut self, value : i64) { self.resolution = value; }

	pub fn get_custom_resolution(&self) -> i64 { self.custom_resolution }
	pub fn set_custom_resolution(&mut self, value : i64) { self.custom_resolution = value; }

	pub fn get_strength(&self) -> i64 { self.strength }
	pub fn set_strength(&mut self, value : i64) { self.strength = value; }

	pub fn get_bias(&self) -> f64 { self.bias }
	pub fn set_bias(&mut self, value : f64) { self.bias = value; }

	pub fn get_normal_bias(&self) -> f64 { self.normal_bias }
	pub fn set_normal_bias(&mut self, value : f64) { self.normal_bias = value; }

	pub fn get_near_plane(&self) -> f64 { self.near_plane }
	pub fn set_near_plane(&mut self, value : f64) { self.near_plane = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct CullingMask {
	serialized_version : i64,
	bits : i64,

}

impl FromYaml for CullingMask {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["CullingMask"] != Yaml::BadValue {
			y = &yaml["CullingMask"];
		}

		CullingMask {
			serialized_version : y["serializedVersion"].as_i64().expect("Could not find item serializedVersion in CullingMask."),
			bits : y["m_Bits"].as_i64().expect("Could not find item m_Bits in CullingMask."),
		}
	}

}

impl CullingMask {
	pub fn get_serialized_version(&self) -> i64 { self.serialized_version }
	pub fn set_serialized_version(&mut self, value : i64) { self.serialized_version = value; }

	pub fn get_bits(&self) -> i64 { self.bits }
	pub fn set_bits(&mut self, value : i64) { self.bits = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct Light {
	object_hide_flags : i64,
	prefab_parent_object : FileReference,
	prefab_internal : FileReference,
	game_object : FileReference,
	enabled : i64,
	serialized_version : i64,
	light_type : i64,
	color : Color,
	intensity : f64,
	range : f64,
	spot_angle : i64,
	cookie_size : i64,
	shadows : Shadows,
	cookie : FileReference,
	draw_halo : i64,
	flare : FileReference,
	render_mode : i64,
	culling_mask : CullingMask,
	lightmapping : i64,
	area_size : Vec2,
	bounce_intensity : i64,
	falloff_table : FalloffTable,
	color_temperature : i64,
	use_color_temperature : i64,
	shadow_radius : i64,
	shadow_angle : i64,

}

impl FromYaml for Light {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["Light"] != Yaml::BadValue {
			y = &yaml["Light"];
		}

		Light {
			object_hide_flags : y["m_ObjectHideFlags"].as_i64().expect("Could not find item m_ObjectHideFlags in Light."),
			prefab_parent_object : FileReference::from_yaml(&y["m_PrefabParentObject"]),
			prefab_internal : FileReference::from_yaml(&y["m_PrefabInternal"]),
			game_object : FileReference::from_yaml(&y["m_GameObject"]),
			enabled : y["m_Enabled"].as_i64().expect("Could not find item m_Enabled in Light."),
			serialized_version : y["serializedVersion"].as_i64().expect("Could not find item serializedVersion in Light."),
			light_type : y["m_Type"].as_i64().expect("Could not find item m_Type in Light."),
			color : Color::from_yaml(&y["m_Color"]),
			intensity : y["m_Intensity"].as_f64().expect("Could not find item m_Intensity in Light."),
			range : y["m_Range"].as_f64().expect("Could not find item m_Range in Light."),
			spot_angle : y["m_SpotAngle"].as_i64().expect("Could not find item m_SpotAngle in Light."),
			cookie_size : y["m_CookieSize"].as_i64().expect("Could not find item m_CookieSize in Light."),
			shadows : Shadows::from_yaml(&y["m_Shadows"]),
			cookie : FileReference::from_yaml(&y["m_Cookie"]),
			draw_halo : y["m_DrawHalo"].as_i64().expect("Could not find item m_DrawHalo in Light."),
			flare : FileReference::from_yaml(&y["m_Flare"]),
			render_mode : y["m_RenderMode"].as_i64().expect("Could not find item m_RenderMode in Light."),
			culling_mask : CullingMask::from_yaml(&y["m_CullingMask"]),
			lightmapping : y["m_Lightmapping"].as_i64().expect("Could not find item m_Lightmapping in Light."),
			area_size : Vec2::from_yaml(&y["m_AreaSize"]),
			bounce_intensity : y["m_BounceIntensity"].as_i64().expect("Could not find item m_BounceIntensity in Light."),
			falloff_table : FalloffTable::from_yaml(&y["m_FalloffTable"]),
			color_temperature : y["m_ColorTemperature"].as_i64().expect("Could not find item m_ColorTemperature in Light."),
			use_color_temperature : y["m_UseColorTemperature"].as_i64().expect("Could not find item m_UseColorTemperature in Light."),
			shadow_radius : y["m_ShadowRadius"].as_i64().expect("Could not find item m_ShadowRadius in Light."),
			shadow_angle : y["m_ShadowAngle"].as_i64().expect("Could not find item m_ShadowAngle in Light."),
		}
	}

}

impl Light {
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

	pub fn get_serialized_version(&self) -> i64 { self.serialized_version }
	pub fn set_serialized_version(&mut self, value : i64) { self.serialized_version = value; }

	pub fn get_light_type(&self) -> i64 { self.light_type }
	pub fn set_light_type(&mut self, value : i64) { self.light_type = value; }

	pub fn get_color(&self) -> Color { self.color.clone() }
	pub fn set_color(&mut self, value : Color) { self.color = value; }

	pub fn get_intensity(&self) -> f64 { self.intensity }
	pub fn set_intensity(&mut self, value : f64) { self.intensity = value; }

	pub fn get_range(&self) -> f64 { self.range }
	pub fn set_range(&mut self, value : f64) { self.range = value; }

	pub fn get_spot_angle(&self) -> i64 { self.spot_angle }
	pub fn set_spot_angle(&mut self, value : i64) { self.spot_angle = value; }

	pub fn get_cookie_size(&self) -> i64 { self.cookie_size }
	pub fn set_cookie_size(&mut self, value : i64) { self.cookie_size = value; }

	pub fn get_shadows(&self) -> Shadows { self.shadows.clone() }
	pub fn set_shadows(&mut self, value : Shadows) { self.shadows = value; }

	pub fn get_cookie(&self) -> FileReference { self.cookie.clone() }
	pub fn set_cookie(&mut self, value : FileReference) { self.cookie = value; }

	pub fn get_draw_halo(&self) -> i64 { self.draw_halo }
	pub fn set_draw_halo(&mut self, value : i64) { self.draw_halo = value; }

	pub fn get_flare(&self) -> FileReference { self.flare.clone() }
	pub fn set_flare(&mut self, value : FileReference) { self.flare = value; }

	pub fn get_render_mode(&self) -> i64 { self.render_mode }
	pub fn set_render_mode(&mut self, value : i64) { self.render_mode = value; }

	pub fn get_culling_mask(&self) -> CullingMask { self.culling_mask.clone() }
	pub fn set_culling_mask(&mut self, value : CullingMask) { self.culling_mask = value; }

	pub fn get_lightmapping(&self) -> i64 { self.lightmapping }
	pub fn set_lightmapping(&mut self, value : i64) { self.lightmapping = value; }

	pub fn get_area_size(&self) -> Vec2 { self.area_size.clone() }
	pub fn set_area_size(&mut self, value : Vec2) { self.area_size = value; }

	pub fn get_bounce_intensity(&self) -> i64 { self.bounce_intensity }
	pub fn set_bounce_intensity(&mut self, value : i64) { self.bounce_intensity = value; }

	pub fn get_falloff_table(&self) -> FalloffTable { self.falloff_table.clone() }
	pub fn set_falloff_table(&mut self, value : FalloffTable) { self.falloff_table = value; }

	pub fn get_color_temperature(&self) -> i64 { self.color_temperature }
	pub fn set_color_temperature(&mut self, value : i64) { self.color_temperature = value; }

	pub fn get_use_color_temperature(&self) -> i64 { self.use_color_temperature }
	pub fn set_use_color_temperature(&mut self, value : i64) { self.use_color_temperature = value; }

	pub fn get_shadow_radius(&self) -> i64 { self.shadow_radius }
	pub fn set_shadow_radius(&mut self, value : i64) { self.shadow_radius = value; }

	pub fn get_shadow_angle(&self) -> i64 { self.shadow_angle }
	pub fn set_shadow_angle(&mut self, value : i64) { self.shadow_angle = value; }

}

#[cfg(test)]
mod light_tests {
	use super::Light;
	use crate::file_types::asset_file::*;

	#[test]
	fn test_light() {
		let mut path = std::env::current_dir().unwrap();
		path.push("src");
		path.push("file_types");
		path.push("templates");
		path.push("Light.yaml");
		println!("{}", path.display());
		let subject = <dyn AssetFile>::load_asset_file_from_path::<Light>(&path).unwrap();

		//Tests
		assert_eq!(subject.object_hide_flags, 0);
		assert_eq!(subject.enabled, 1);
		assert_eq!(subject.serialized_version, 8);
		assert_eq!(subject.light_type, 2);
		assert_eq!(subject.intensity, 5.94);
		assert_eq!(subject.range, 2.67);
		assert_eq!(subject.spot_angle, 30);
		assert_eq!(subject.cookie_size, 10);
		assert_eq!(subject.draw_halo, 0);
		assert_eq!(subject.render_mode, 0);
		assert_eq!(subject.lightmapping, 4);
		assert_eq!(subject.bounce_intensity, 1);
		assert_eq!(subject.color_temperature, 6570);
		assert_eq!(subject.use_color_temperature, 0);
		assert_eq!(subject.shadow_radius, 0);
		assert_eq!(subject.shadow_angle, 0);
	}
}


