use unity_yaml_rust::Yaml;
use crate::file_types::sections::Empty;
use crate::file_types::sections::FileReference;
use crate::from_yaml_trait::*;
use crate::file_types::sections::BuildTarget;
use crate::file_types::sections::Vec4;
use std::vec::Vec;
use crate::file_types::sections::Vec2;

#[derive(Debug, Clone, PartialEq)]
pub struct Mipmaps {
	mip_map_mode : i64,
	enable_mip_map : i64,
	s_r_g_b_texture : i64,
	linear_texture : i64,
	fade_out : i64,
	border_mip_map : i64,
	mip_map_fade_distance_start : i64,
	mip_map_fade_distance_end : i64,

}

impl FromYaml for Mipmaps {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["Mipmaps"] != Yaml::BadValue {
			y = &yaml["Mipmaps"];
		}

		Mipmaps {
			mip_map_mode : y["mipMapMode"].as_i64().expect("Could not find item mipMapMode in Mipmaps."),
			enable_mip_map : y["enableMipMap"].as_i64().expect("Could not find item enableMipMap in Mipmaps."),
			s_r_g_b_texture : y["sRGBTexture"].as_i64().expect("Could not find item sRGBTexture in Mipmaps."),
			linear_texture : y["linearTexture"].as_i64().expect("Could not find item linearTexture in Mipmaps."),
			fade_out : y["fadeOut"].as_i64().expect("Could not find item fadeOut in Mipmaps."),
			border_mip_map : y["borderMipMap"].as_i64().expect("Could not find item borderMipMap in Mipmaps."),
			mip_map_fade_distance_start : y["mipMapFadeDistanceStart"].as_i64().expect("Could not find item mipMapFadeDistanceStart in Mipmaps."),
			mip_map_fade_distance_end : y["mipMapFadeDistanceEnd"].as_i64().expect("Could not find item mipMapFadeDistanceEnd in Mipmaps."),
		}
	}

}

impl Mipmaps {
	pub fn get_mip_map_mode(&self) -> i64 { self.mip_map_mode }
	pub fn set_mip_map_mode(&mut self, value : i64) { self.mip_map_mode = value; }

	pub fn get_enable_mip_map(&self) -> i64 { self.enable_mip_map }
	pub fn set_enable_mip_map(&mut self, value : i64) { self.enable_mip_map = value; }

	pub fn get_s_r_g_b_texture(&self) -> i64 { self.s_r_g_b_texture }
	pub fn set_s_r_g_b_texture(&mut self, value : i64) { self.s_r_g_b_texture = value; }

	pub fn get_linear_texture(&self) -> i64 { self.linear_texture }
	pub fn set_linear_texture(&mut self, value : i64) { self.linear_texture = value; }

	pub fn get_fade_out(&self) -> i64 { self.fade_out }
	pub fn set_fade_out(&mut self, value : i64) { self.fade_out = value; }

	pub fn get_border_mip_map(&self) -> i64 { self.border_mip_map }
	pub fn set_border_mip_map(&mut self, value : i64) { self.border_mip_map = value; }

	pub fn get_mip_map_fade_distance_start(&self) -> i64 { self.mip_map_fade_distance_start }
	pub fn set_mip_map_fade_distance_start(&mut self, value : i64) { self.mip_map_fade_distance_start = value; }

	pub fn get_mip_map_fade_distance_end(&self) -> i64 { self.mip_map_fade_distance_end }
	pub fn set_mip_map_fade_distance_end(&mut self, value : i64) { self.mip_map_fade_distance_end = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct Bumpmap {
	convert_to_normal_map : i64,
	external_normal_map : i64,
	height_scale : f64,
	normal_map_filter : i64,

}

impl FromYaml for Bumpmap {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["Bumpmap"] != Yaml::BadValue {
			y = &yaml["Bumpmap"];
		}

		Bumpmap {
			convert_to_normal_map : y["convertToNormalMap"].as_i64().expect("Could not find item convertToNormalMap in Bumpmap."),
			external_normal_map : y["externalNormalMap"].as_i64().expect("Could not find item externalNormalMap in Bumpmap."),
			height_scale : y["heightScale"].as_f64().expect("Could not find item heightScale in Bumpmap."),
			normal_map_filter : y["normalMapFilter"].as_i64().expect("Could not find item normalMapFilter in Bumpmap."),
		}
	}

}

impl Bumpmap {
	pub fn get_convert_to_normal_map(&self) -> i64 { self.convert_to_normal_map }
	pub fn set_convert_to_normal_map(&mut self, value : i64) { self.convert_to_normal_map = value; }

	pub fn get_external_normal_map(&self) -> i64 { self.external_normal_map }
	pub fn set_external_normal_map(&mut self, value : i64) { self.external_normal_map = value; }

	pub fn get_height_scale(&self) -> f64 { self.height_scale }
	pub fn set_height_scale(&mut self, value : f64) { self.height_scale = value; }

	pub fn get_normal_map_filter(&self) -> i64 { self.normal_map_filter }
	pub fn set_normal_map_filter(&mut self, value : i64) { self.normal_map_filter = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct TextureSettings {
	filter_mode : i64,
	aniso : i64,
	mip_bias : i64,
	wrap_mode : i64,

}

impl FromYaml for TextureSettings {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["TextureSettings"] != Yaml::BadValue {
			y = &yaml["TextureSettings"];
		}

		TextureSettings {
			filter_mode : y["filterMode"].as_i64().expect("Could not find item filterMode in TextureSettings."),
			aniso : y["aniso"].as_i64().expect("Could not find item aniso in TextureSettings."),
			mip_bias : y["mipBias"].as_i64().expect("Could not find item mipBias in TextureSettings."),
			wrap_mode : y["wrapMode"].as_i64().expect("Could not find item wrapMode in TextureSettings."),
		}
	}

}

impl TextureSettings {
	pub fn get_filter_mode(&self) -> i64 { self.filter_mode }
	pub fn set_filter_mode(&mut self, value : i64) { self.filter_mode = value; }

	pub fn get_aniso(&self) -> i64 { self.aniso }
	pub fn set_aniso(&mut self, value : i64) { self.aniso = value; }

	pub fn get_mip_bias(&self) -> i64 { self.mip_bias }
	pub fn set_mip_bias(&mut self, value : i64) { self.mip_bias = value; }

	pub fn get_wrap_mode(&self) -> i64 { self.wrap_mode }
	pub fn set_wrap_mode(&mut self, value : i64) { self.wrap_mode = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct SpriteSheet {
	serialized_version : i64,
	sprites : Vec<Empty>,
	outline : Vec<Empty>,

}

impl FromYaml for SpriteSheet {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["SpriteSheet"] != Yaml::BadValue {
			y = &yaml["SpriteSheet"];
		}

		SpriteSheet {
			serialized_version : y["serializedVersion"].as_i64().expect("Could not find item serializedVersion in SpriteSheet."),
			sprites : Vec::new(),
			outline : Vec::new(),
		}
	}

}

impl SpriteSheet {
	pub fn get_serialized_version(&self) -> i64 { self.serialized_version }
	pub fn set_serialized_version(&mut self, value : i64) { self.serialized_version = value; }

	pub fn get_sprites(&self) -> Vec<Empty> { self.sprites.clone() }
	pub fn set_sprites(&mut self, value : Vec<Empty>) { self.sprites = value; }

	pub fn get_outline(&self) -> Vec<Empty> { self.outline.clone() }
	pub fn set_outline(&mut self, value : Vec<Empty>) { self.outline = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct TextureImporter {
	file_i_d_to_recycle_name : Empty,
	serialized_version : i64,
	mipmaps : Mipmaps,
	bumpmap : Bumpmap,
	is_readable : i64,
	gray_scale_to_alpha : i64,
	generate_cubemap : i64,
	cubemap_convolution : i64,
	seamless_cubemap : i64,
	texture_format : i64,
	max_texture_size : i64,
	texture_settings : TextureSettings,
	n_p_o_t_scale : i64,
	lightmap : i64,
	compression_quality : i64,
	sprite_mode : i64,
	sprite_extrude : i64,
	sprite_mesh_type : i64,
	alignment : i64,
	sprite_pivot : Vec2,
	sprite_border : Vec4,
	sprite_pixels_to_units : i64,
	alpha_usage : i64,
	alpha_is_transparency : i64,
	sprite_tessellation_detail : i64,
	texture_type : i64,
	texture_shape : i64,
	max_texture_size_set : i64,
	compression_quality_set : i64,
	texture_format_set : i64,
	platform_settings : Vec<BuildTarget>,
	sprite_sheet : SpriteSheet,
	sprite_packing_tag : String,
	user_data : String,
	asset_bundle_name : String,
	asset_bundle_variant : String,

}

impl FromYaml for TextureImporter {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["TextureImporter"] != Yaml::BadValue {
			y = &yaml["TextureImporter"];
		}

		TextureImporter {
			file_i_d_to_recycle_name : Empty::from_yaml(&y["fileIDToRecycleName"]),
			serialized_version : y["serializedVersion"].as_i64().expect("Could not find item serializedVersion in TextureImporter."),
			mipmaps : Mipmaps::from_yaml(&y["mipmaps"]),
			bumpmap : Bumpmap::from_yaml(&y["bumpmap"]),
			is_readable : y["isReadable"].as_i64().expect("Could not find item isReadable in TextureImporter."),
			gray_scale_to_alpha : y["grayScaleToAlpha"].as_i64().expect("Could not find item grayScaleToAlpha in TextureImporter."),
			generate_cubemap : y["generateCubemap"].as_i64().expect("Could not find item generateCubemap in TextureImporter."),
			cubemap_convolution : y["cubemapConvolution"].as_i64().expect("Could not find item cubemapConvolution in TextureImporter."),
			seamless_cubemap : y["seamlessCubemap"].as_i64().expect("Could not find item seamlessCubemap in TextureImporter."),
			texture_format : y["textureFormat"].as_i64().expect("Could not find item textureFormat in TextureImporter."),
			max_texture_size : y["maxTextureSize"].as_i64().expect("Could not find item maxTextureSize in TextureImporter."),
			texture_settings : TextureSettings::from_yaml(&y["textureSettings"]),
			n_p_o_t_scale : y["nPOTScale"].as_i64().expect("Could not find item nPOTScale in TextureImporter."),
			lightmap : y["lightmap"].as_i64().expect("Could not find item lightmap in TextureImporter."),
			compression_quality : y["compressionQuality"].as_i64().expect("Could not find item compressionQuality in TextureImporter."),
			sprite_mode : y["spriteMode"].as_i64().expect("Could not find item spriteMode in TextureImporter."),
			sprite_extrude : y["spriteExtrude"].as_i64().expect("Could not find item spriteExtrude in TextureImporter."),
			sprite_mesh_type : y["spriteMeshType"].as_i64().expect("Could not find item spriteMeshType in TextureImporter."),
			alignment : y["alignment"].as_i64().expect("Could not find item alignment in TextureImporter."),
			sprite_pivot : Vec2::from_yaml(&y["spritePivot"]),
			sprite_border : Vec4::from_yaml(&y["spriteBorder"]),
			sprite_pixels_to_units : y["spritePixelsToUnits"].as_i64().expect("Could not find item spritePixelsToUnits in TextureImporter."),
			alpha_usage : y["alphaUsage"].as_i64().expect("Could not find item alphaUsage in TextureImporter."),
			alpha_is_transparency : y["alphaIsTransparency"].as_i64().expect("Could not find item alphaIsTransparency in TextureImporter."),
			sprite_tessellation_detail : y["spriteTessellationDetail"].as_i64().expect("Could not find item spriteTessellationDetail in TextureImporter."),
			texture_type : y["textureType"].as_i64().expect("Could not find item textureType in TextureImporter."),
			texture_shape : y["textureShape"].as_i64().expect("Could not find item textureShape in TextureImporter."),
			max_texture_size_set : y["maxTextureSizeSet"].as_i64().expect("Could not find item maxTextureSizeSet in TextureImporter."),
			compression_quality_set : y["compressionQualitySet"].as_i64().expect("Could not find item compressionQualitySet in TextureImporter."),
			texture_format_set : y["textureFormatSet"].as_i64().expect("Could not find item textureFormatSet in TextureImporter."),
			platform_settings : read_yaml_vector(&y["platformSettings"], "platformSettings"),
			sprite_sheet : SpriteSheet::from_yaml(&y["spriteSheet"]),
			sprite_packing_tag : String::from(""),
			user_data : String::from(""),
			asset_bundle_name : String::from(""),
			asset_bundle_variant : String::from(""),
		}
	}

}

impl TextureImporter {
	pub fn get_file_i_d_to_recycle_name(&self) -> Empty { self.file_i_d_to_recycle_name.clone() }
	pub fn set_file_i_d_to_recycle_name(&mut self, value : Empty) { self.file_i_d_to_recycle_name = value; }

	pub fn get_serialized_version(&self) -> i64 { self.serialized_version }
	pub fn set_serialized_version(&mut self, value : i64) { self.serialized_version = value; }

	pub fn get_mipmaps(&self) -> Mipmaps { self.mipmaps.clone() }
	pub fn set_mipmaps(&mut self, value : Mipmaps) { self.mipmaps = value; }

	pub fn get_bumpmap(&self) -> Bumpmap { self.bumpmap.clone() }
	pub fn set_bumpmap(&mut self, value : Bumpmap) { self.bumpmap = value; }

	pub fn get_is_readable(&self) -> i64 { self.is_readable }
	pub fn set_is_readable(&mut self, value : i64) { self.is_readable = value; }

	pub fn get_gray_scale_to_alpha(&self) -> i64 { self.gray_scale_to_alpha }
	pub fn set_gray_scale_to_alpha(&mut self, value : i64) { self.gray_scale_to_alpha = value; }

	pub fn get_generate_cubemap(&self) -> i64 { self.generate_cubemap }
	pub fn set_generate_cubemap(&mut self, value : i64) { self.generate_cubemap = value; }

	pub fn get_cubemap_convolution(&self) -> i64 { self.cubemap_convolution }
	pub fn set_cubemap_convolution(&mut self, value : i64) { self.cubemap_convolution = value; }

	pub fn get_seamless_cubemap(&self) -> i64 { self.seamless_cubemap }
	pub fn set_seamless_cubemap(&mut self, value : i64) { self.seamless_cubemap = value; }

	pub fn get_texture_format(&self) -> i64 { self.texture_format }
	pub fn set_texture_format(&mut self, value : i64) { self.texture_format = value; }

	pub fn get_max_texture_size(&self) -> i64 { self.max_texture_size }
	pub fn set_max_texture_size(&mut self, value : i64) { self.max_texture_size = value; }

	pub fn get_texture_settings(&self) -> TextureSettings { self.texture_settings.clone() }
	pub fn set_texture_settings(&mut self, value : TextureSettings) { self.texture_settings = value; }

	pub fn get_n_p_o_t_scale(&self) -> i64 { self.n_p_o_t_scale }
	pub fn set_n_p_o_t_scale(&mut self, value : i64) { self.n_p_o_t_scale = value; }

	pub fn get_lightmap(&self) -> i64 { self.lightmap }
	pub fn set_lightmap(&mut self, value : i64) { self.lightmap = value; }

	pub fn get_compression_quality(&self) -> i64 { self.compression_quality }
	pub fn set_compression_quality(&mut self, value : i64) { self.compression_quality = value; }

	pub fn get_sprite_mode(&self) -> i64 { self.sprite_mode }
	pub fn set_sprite_mode(&mut self, value : i64) { self.sprite_mode = value; }

	pub fn get_sprite_extrude(&self) -> i64 { self.sprite_extrude }
	pub fn set_sprite_extrude(&mut self, value : i64) { self.sprite_extrude = value; }

	pub fn get_sprite_mesh_type(&self) -> i64 { self.sprite_mesh_type }
	pub fn set_sprite_mesh_type(&mut self, value : i64) { self.sprite_mesh_type = value; }

	pub fn get_alignment(&self) -> i64 { self.alignment }
	pub fn set_alignment(&mut self, value : i64) { self.alignment = value; }

	pub fn get_sprite_pivot(&self) -> Vec2 { self.sprite_pivot.clone() }
	pub fn set_sprite_pivot(&mut self, value : Vec2) { self.sprite_pivot = value; }

	pub fn get_sprite_border(&self) -> Vec4 { self.sprite_border.clone() }
	pub fn set_sprite_border(&mut self, value : Vec4) { self.sprite_border = value; }

	pub fn get_sprite_pixels_to_units(&self) -> i64 { self.sprite_pixels_to_units }
	pub fn set_sprite_pixels_to_units(&mut self, value : i64) { self.sprite_pixels_to_units = value; }

	pub fn get_alpha_usage(&self) -> i64 { self.alpha_usage }
	pub fn set_alpha_usage(&mut self, value : i64) { self.alpha_usage = value; }

	pub fn get_alpha_is_transparency(&self) -> i64 { self.alpha_is_transparency }
	pub fn set_alpha_is_transparency(&mut self, value : i64) { self.alpha_is_transparency = value; }

	pub fn get_sprite_tessellation_detail(&self) -> i64 { self.sprite_tessellation_detail }
	pub fn set_sprite_tessellation_detail(&mut self, value : i64) { self.sprite_tessellation_detail = value; }

	pub fn get_texture_type(&self) -> i64 { self.texture_type }
	pub fn set_texture_type(&mut self, value : i64) { self.texture_type = value; }

	pub fn get_texture_shape(&self) -> i64 { self.texture_shape }
	pub fn set_texture_shape(&mut self, value : i64) { self.texture_shape = value; }

	pub fn get_max_texture_size_set(&self) -> i64 { self.max_texture_size_set }
	pub fn set_max_texture_size_set(&mut self, value : i64) { self.max_texture_size_set = value; }

	pub fn get_compression_quality_set(&self) -> i64 { self.compression_quality_set }
	pub fn set_compression_quality_set(&mut self, value : i64) { self.compression_quality_set = value; }

	pub fn get_texture_format_set(&self) -> i64 { self.texture_format_set }
	pub fn set_texture_format_set(&mut self, value : i64) { self.texture_format_set = value; }

	pub fn get_platform_settings(&self) -> Vec<BuildTarget> { self.platform_settings.clone() }
	pub fn set_platform_settings(&mut self, value : Vec<BuildTarget>) { self.platform_settings = value; }

	pub fn get_sprite_sheet(&self) -> SpriteSheet { self.sprite_sheet.clone() }
	pub fn set_sprite_sheet(&mut self, value : SpriteSheet) { self.sprite_sheet = value; }

	pub fn get_sprite_packing_tag(&self) -> String { self.sprite_packing_tag.clone() }
	pub fn set_sprite_packing_tag(&mut self, value : String) { self.sprite_packing_tag = value; }

	pub fn get_user_data(&self) -> String { self.user_data.clone() }
	pub fn set_user_data(&mut self, value : String) { self.user_data = value; }

	pub fn get_asset_bundle_name(&self) -> String { self.asset_bundle_name.clone() }
	pub fn set_asset_bundle_name(&mut self, value : String) { self.asset_bundle_name = value; }

	pub fn get_asset_bundle_variant(&self) -> String { self.asset_bundle_variant.clone() }
	pub fn set_asset_bundle_variant(&mut self, value : String) { self.asset_bundle_variant = value; }

}

#[cfg(test)]
mod texture_importer_tests {
	use super::TextureImporter;
	use crate::file_types::asset_file::*;

	#[test]
	fn test_texture_importer() {
		let mut path = std::env::current_dir().unwrap();
		path.push("src");
		path.push("file_types");
		path.push("templates");
		path.push("TextureImporter.yaml");
		println!("{}", path.display());
		let subject = <dyn AssetFile>::load_asset_file_from_path::<TextureImporter>(&path).unwrap();

		//Tests
		assert_eq!(subject.serialized_version, 4);
		assert_eq!(subject.is_readable, 0);
		assert_eq!(subject.gray_scale_to_alpha, 0);
		assert_eq!(subject.generate_cubemap, 6);
		assert_eq!(subject.cubemap_convolution, 0);
		assert_eq!(subject.seamless_cubemap, 0);
		assert_eq!(subject.texture_format, 1);
		assert_eq!(subject.max_texture_size, 2048);
		assert_eq!(subject.n_p_o_t_scale, 1);
		assert_eq!(subject.lightmap, 0);
		assert_eq!(subject.compression_quality, 50);
		assert_eq!(subject.sprite_mode, 0);
		assert_eq!(subject.sprite_extrude, 1);
		assert_eq!(subject.sprite_mesh_type, 1);
		assert_eq!(subject.alignment, 0);
		assert_eq!(subject.sprite_pixels_to_units, 100);
		assert_eq!(subject.alpha_usage, 1);
		assert_eq!(subject.alpha_is_transparency, 0);
		assert_eq!(subject.sprite_tessellation_detail, -1);
		assert_eq!(subject.texture_type, 1);
		assert_eq!(subject.texture_shape, 1);
		assert_eq!(subject.max_texture_size_set, 0);
		assert_eq!(subject.compression_quality_set, 0);
		assert_eq!(subject.texture_format_set, 0);
	}
}


