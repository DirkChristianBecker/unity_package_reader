use crate::from_yaml_trait::*;
use unity_yaml_rust::Yaml;
use std::collections::HashMap;
use crate::file_types::sections::Empty;
use crate::file_types::sections::Vec4;
use crate::file_types::sections::FileReference;
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Materials {
	import_materials : i64,
	material_name : i64,
	material_search : i64,

}

impl FromYaml for Materials {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["Materials"] != Yaml::BadValue {
			y = &yaml["Materials"];
		}

		Materials {
			import_materials : y["importMaterials"].as_i64().expect("Could not find item importMaterials in Materials."),
			material_name : y["materialName"].as_i64().expect("Could not find item materialName in Materials."),
			material_search : y["materialSearch"].as_i64().expect("Could not find item materialSearch in Materials."),
		}
	}

}

impl Materials {
	pub fn get_import_materials(&self) -> i64 { self.import_materials }
	pub fn set_import_materials(&mut self, value : i64) { self.import_materials = value; }

	pub fn get_material_name(&self) -> i64 { self.material_name }
	pub fn set_material_name(&mut self, value : i64) { self.material_name = value; }

	pub fn get_material_search(&self) -> i64 { self.material_search }
	pub fn set_material_search(&mut self, value : i64) { self.material_search = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct Animations {
	legacy_generate_animations : i64,
	bake_simulation : i64,
	resample_curves : i64,
	optimize_game_objects : i64,
	motion_node_name : String,
	rig_import_errors : String,
	rig_import_warnings : String,
	animation_import_errors : String,
	animation_import_warnings : String,
	animation_retargeting_warnings : String,
	animation_do_retargeting_warnings : i64,
	animation_compression : i64,
	animation_rotation_error : f64,
	animation_position_error : f64,
	animation_scale_error : f64,
	animation_wrap_mode : i64,
	extra_exposed_transform_paths : Vec<Empty>,
	clip_animations : Vec<Empty>,
	is_readable : i64,

}

impl FromYaml for Animations {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["Animations"] != Yaml::BadValue {
			y = &yaml["Animations"];
		}

		Animations {
			legacy_generate_animations : y["legacyGenerateAnimations"].as_i64().expect("Could not find item legacyGenerateAnimations in Animations."),
			bake_simulation : y["bakeSimulation"].as_i64().expect("Could not find item bakeSimulation in Animations."),
			resample_curves : y["resampleCurves"].as_i64().expect("Could not find item resampleCurves in Animations."),
			optimize_game_objects : y["optimizeGameObjects"].as_i64().expect("Could not find item optimizeGameObjects in Animations."),
			motion_node_name : String::from(""),
			rig_import_errors : String::from(""),
			rig_import_warnings : String::from(""),
			animation_import_errors : String::from(""),
			animation_import_warnings : String::from(""),
			animation_retargeting_warnings : String::from(""),
			animation_do_retargeting_warnings : y["animationDoRetargetingWarnings"].as_i64().expect("Could not find item animationDoRetargetingWarnings in Animations."),
			animation_compression : y["animationCompression"].as_i64().expect("Could not find item animationCompression in Animations."),
			animation_rotation_error : y["animationRotationError"].as_f64().expect("Could not find item animationRotationError in Animations."),
			animation_position_error : y["animationPositionError"].as_f64().expect("Could not find item animationPositionError in Animations."),
			animation_scale_error : y["animationScaleError"].as_f64().expect("Could not find item animationScaleError in Animations."),
			animation_wrap_mode : y["animationWrapMode"].as_i64().expect("Could not find item animationWrapMode in Animations."),
			extra_exposed_transform_paths : Vec::new(),
			clip_animations : Vec::new(),
			is_readable : y["isReadable"].as_i64().expect("Could not find item isReadable in Animations."),
		}
	}

}

impl Animations {
	pub fn get_legacy_generate_animations(&self) -> i64 { self.legacy_generate_animations }
	pub fn set_legacy_generate_animations(&mut self, value : i64) { self.legacy_generate_animations = value; }

	pub fn get_bake_simulation(&self) -> i64 { self.bake_simulation }
	pub fn set_bake_simulation(&mut self, value : i64) { self.bake_simulation = value; }

	pub fn get_resample_curves(&self) -> i64 { self.resample_curves }
	pub fn set_resample_curves(&mut self, value : i64) { self.resample_curves = value; }

	pub fn get_optimize_game_objects(&self) -> i64 { self.optimize_game_objects }
	pub fn set_optimize_game_objects(&mut self, value : i64) { self.optimize_game_objects = value; }

	pub fn get_motion_node_name(&self) -> String { self.motion_node_name.clone() }
	pub fn set_motion_node_name(&mut self, value : String) { self.motion_node_name = value; }

	pub fn get_rig_import_errors(&self) -> String { self.rig_import_errors.clone() }
	pub fn set_rig_import_errors(&mut self, value : String) { self.rig_import_errors = value; }

	pub fn get_rig_import_warnings(&self) -> String { self.rig_import_warnings.clone() }
	pub fn set_rig_import_warnings(&mut self, value : String) { self.rig_import_warnings = value; }

	pub fn get_animation_import_errors(&self) -> String { self.animation_import_errors.clone() }
	pub fn set_animation_import_errors(&mut self, value : String) { self.animation_import_errors = value; }

	pub fn get_animation_import_warnings(&self) -> String { self.animation_import_warnings.clone() }
	pub fn set_animation_import_warnings(&mut self, value : String) { self.animation_import_warnings = value; }

	pub fn get_animation_retargeting_warnings(&self) -> String { self.animation_retargeting_warnings.clone() }
	pub fn set_animation_retargeting_warnings(&mut self, value : String) { self.animation_retargeting_warnings = value; }

	pub fn get_animation_do_retargeting_warnings(&self) -> i64 { self.animation_do_retargeting_warnings }
	pub fn set_animation_do_retargeting_warnings(&mut self, value : i64) { self.animation_do_retargeting_warnings = value; }

	pub fn get_animation_compression(&self) -> i64 { self.animation_compression }
	pub fn set_animation_compression(&mut self, value : i64) { self.animation_compression = value; }

	pub fn get_animation_rotation_error(&self) -> f64 { self.animation_rotation_error }
	pub fn set_animation_rotation_error(&mut self, value : f64) { self.animation_rotation_error = value; }

	pub fn get_animation_position_error(&self) -> f64 { self.animation_position_error }
	pub fn set_animation_position_error(&mut self, value : f64) { self.animation_position_error = value; }

	pub fn get_animation_scale_error(&self) -> f64 { self.animation_scale_error }
	pub fn set_animation_scale_error(&mut self, value : f64) { self.animation_scale_error = value; }

	pub fn get_animation_wrap_mode(&self) -> i64 { self.animation_wrap_mode }
	pub fn set_animation_wrap_mode(&mut self, value : i64) { self.animation_wrap_mode = value; }

	pub fn get_extra_exposed_transform_paths(&self) -> Vec<Empty> { self.extra_exposed_transform_paths.clone() }
	pub fn set_extra_exposed_transform_paths(&mut self, value : Vec<Empty>) { self.extra_exposed_transform_paths = value; }

	pub fn get_clip_animations(&self) -> Vec<Empty> { self.clip_animations.clone() }
	pub fn set_clip_animations(&mut self, value : Vec<Empty>) { self.clip_animations = value; }

	pub fn get_is_readable(&self) -> i64 { self.is_readable }
	pub fn set_is_readable(&mut self, value : i64) { self.is_readable = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct Meshes {
	l_o_d_screen_percentages : Vec<Empty>,
	global_scale : i64,
	mesh_compression : i64,
	add_colliders : i64,
	import_blend_shapes : i64,
	swap_u_v_channels : i64,
	generate_secondary_u_v : i64,
	use_file_units : i64,
	optimize_mesh_for_g_p_u : i64,
	keep_quads : i64,
	weld_vertices : i64,
	secondary_u_v_angle_distortion : i64,
	secondary_u_v_area_distortion : f64,
	secondary_u_v_hard_angle : i64,
	secondary_u_v_pack_margin : i64,
	use_file_scale : i64,

}

impl FromYaml for Meshes {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["Meshes"] != Yaml::BadValue {
			y = &yaml["Meshes"];
		}

		Meshes {
			l_o_d_screen_percentages : Vec::new(),
			global_scale : y["globalScale"].as_i64().expect("Could not find item globalScale in Meshes."),
			mesh_compression : y["meshCompression"].as_i64().expect("Could not find item meshCompression in Meshes."),
			add_colliders : y["addColliders"].as_i64().expect("Could not find item addColliders in Meshes."),
			import_blend_shapes : y["importBlendShapes"].as_i64().expect("Could not find item importBlendShapes in Meshes."),
			swap_u_v_channels : y["swapUVChannels"].as_i64().expect("Could not find item swapUVChannels in Meshes."),
			generate_secondary_u_v : y["generateSecondaryUV"].as_i64().expect("Could not find item generateSecondaryUV in Meshes."),
			use_file_units : y["useFileUnits"].as_i64().expect("Could not find item useFileUnits in Meshes."),
			optimize_mesh_for_g_p_u : y["optimizeMeshForGPU"].as_i64().expect("Could not find item optimizeMeshForGPU in Meshes."),
			keep_quads : y["keepQuads"].as_i64().expect("Could not find item keepQuads in Meshes."),
			weld_vertices : y["weldVertices"].as_i64().expect("Could not find item weldVertices in Meshes."),
			secondary_u_v_angle_distortion : y["secondaryUVAngleDistortion"].as_i64().expect("Could not find item secondaryUVAngleDistortion in Meshes."),
			secondary_u_v_area_distortion : y["secondaryUVAreaDistortion"].as_f64().expect("Could not find item secondaryUVAreaDistortion in Meshes."),
			secondary_u_v_hard_angle : y["secondaryUVHardAngle"].as_i64().expect("Could not find item secondaryUVHardAngle in Meshes."),
			secondary_u_v_pack_margin : y["secondaryUVPackMargin"].as_i64().expect("Could not find item secondaryUVPackMargin in Meshes."),
			use_file_scale : y["useFileScale"].as_i64().expect("Could not find item useFileScale in Meshes."),
		}
	}

}

impl Meshes {
	pub fn get_l_o_d_screen_percentages(&self) -> Vec<Empty> { self.l_o_d_screen_percentages.clone() }
	pub fn set_l_o_d_screen_percentages(&mut self, value : Vec<Empty>) { self.l_o_d_screen_percentages = value; }

	pub fn get_global_scale(&self) -> i64 { self.global_scale }
	pub fn set_global_scale(&mut self, value : i64) { self.global_scale = value; }

	pub fn get_mesh_compression(&self) -> i64 { self.mesh_compression }
	pub fn set_mesh_compression(&mut self, value : i64) { self.mesh_compression = value; }

	pub fn get_add_colliders(&self) -> i64 { self.add_colliders }
	pub fn set_add_colliders(&mut self, value : i64) { self.add_colliders = value; }

	pub fn get_import_blend_shapes(&self) -> i64 { self.import_blend_shapes }
	pub fn set_import_blend_shapes(&mut self, value : i64) { self.import_blend_shapes = value; }

	pub fn get_swap_u_v_channels(&self) -> i64 { self.swap_u_v_channels }
	pub fn set_swap_u_v_channels(&mut self, value : i64) { self.swap_u_v_channels = value; }

	pub fn get_generate_secondary_u_v(&self) -> i64 { self.generate_secondary_u_v }
	pub fn set_generate_secondary_u_v(&mut self, value : i64) { self.generate_secondary_u_v = value; }

	pub fn get_use_file_units(&self) -> i64 { self.use_file_units }
	pub fn set_use_file_units(&mut self, value : i64) { self.use_file_units = value; }

	pub fn get_optimize_mesh_for_g_p_u(&self) -> i64 { self.optimize_mesh_for_g_p_u }
	pub fn set_optimize_mesh_for_g_p_u(&mut self, value : i64) { self.optimize_mesh_for_g_p_u = value; }

	pub fn get_keep_quads(&self) -> i64 { self.keep_quads }
	pub fn set_keep_quads(&mut self, value : i64) { self.keep_quads = value; }

	pub fn get_weld_vertices(&self) -> i64 { self.weld_vertices }
	pub fn set_weld_vertices(&mut self, value : i64) { self.weld_vertices = value; }

	pub fn get_secondary_u_v_angle_distortion(&self) -> i64 { self.secondary_u_v_angle_distortion }
	pub fn set_secondary_u_v_angle_distortion(&mut self, value : i64) { self.secondary_u_v_angle_distortion = value; }

	pub fn get_secondary_u_v_area_distortion(&self) -> f64 { self.secondary_u_v_area_distortion }
	pub fn set_secondary_u_v_area_distortion(&mut self, value : f64) { self.secondary_u_v_area_distortion = value; }

	pub fn get_secondary_u_v_hard_angle(&self) -> i64 { self.secondary_u_v_hard_angle }
	pub fn set_secondary_u_v_hard_angle(&mut self, value : i64) { self.secondary_u_v_hard_angle = value; }

	pub fn get_secondary_u_v_pack_margin(&self) -> i64 { self.secondary_u_v_pack_margin }
	pub fn set_secondary_u_v_pack_margin(&mut self, value : i64) { self.secondary_u_v_pack_margin = value; }

	pub fn get_use_file_scale(&self) -> i64 { self.use_file_scale }
	pub fn set_use_file_scale(&mut self, value : i64) { self.use_file_scale = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct TangentSpace {
	normal_smooth_angle : i64,
	normal_import_mode : i64,
	tangent_import_mode : i64,

}

impl FromYaml for TangentSpace {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["TangentSpace"] != Yaml::BadValue {
			y = &yaml["TangentSpace"];
		}

		TangentSpace {
			normal_smooth_angle : y["normalSmoothAngle"].as_i64().expect("Could not find item normalSmoothAngle in TangentSpace."),
			normal_import_mode : y["normalImportMode"].as_i64().expect("Could not find item normalImportMode in TangentSpace."),
			tangent_import_mode : y["tangentImportMode"].as_i64().expect("Could not find item tangentImportMode in TangentSpace."),
		}
	}

}

impl TangentSpace {
	pub fn get_normal_smooth_angle(&self) -> i64 { self.normal_smooth_angle }
	pub fn set_normal_smooth_angle(&mut self, value : i64) { self.normal_smooth_angle = value; }

	pub fn get_normal_import_mode(&self) -> i64 { self.normal_import_mode }
	pub fn set_normal_import_mode(&mut self, value : i64) { self.normal_import_mode = value; }

	pub fn get_tangent_import_mode(&self) -> i64 { self.tangent_import_mode }
	pub fn set_tangent_import_mode(&mut self, value : i64) { self.tangent_import_mode = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct HumanDescription {
	serialized_version : i64,
	human : Vec<Empty>,
	skeleton : Vec<Empty>,
	arm_twist : f64,
	fore_arm_twist : f64,
	upper_leg_twist : f64,
	leg_twist : f64,
	arm_stretch : f64,
	leg_stretch : f64,
	feet_spacing : i64,
	root_motion_bone_name : String,
	root_motion_bone_rotation : Vec4,
	has_translation_do_f : i64,
	has_extra_root : i64,
	skeleton_has_parents : i64,

}

impl FromYaml for HumanDescription {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["HumanDescription"] != Yaml::BadValue {
			y = &yaml["HumanDescription"];
		}

		HumanDescription {
			serialized_version : y["serializedVersion"].as_i64().expect("Could not find item serializedVersion in HumanDescription."),
			human : Vec::new(),
			skeleton : Vec::new(),
			arm_twist : y["armTwist"].as_f64().expect("Could not find item armTwist in HumanDescription."),
			fore_arm_twist : y["foreArmTwist"].as_f64().expect("Could not find item foreArmTwist in HumanDescription."),
			upper_leg_twist : y["upperLegTwist"].as_f64().expect("Could not find item upperLegTwist in HumanDescription."),
			leg_twist : y["legTwist"].as_f64().expect("Could not find item legTwist in HumanDescription."),
			arm_stretch : y["armStretch"].as_f64().expect("Could not find item armStretch in HumanDescription."),
			leg_stretch : y["legStretch"].as_f64().expect("Could not find item legStretch in HumanDescription."),
			feet_spacing : y["feetSpacing"].as_i64().expect("Could not find item feetSpacing in HumanDescription."),
			root_motion_bone_name : String::from(""),
			root_motion_bone_rotation : Vec4::from_yaml(&y["rootMotionBoneRotation"]),
			has_translation_do_f : y["hasTranslationDoF"].as_i64().expect("Could not find item hasTranslationDoF in HumanDescription."),
			has_extra_root : y["hasExtraRoot"].as_i64().expect("Could not find item hasExtraRoot in HumanDescription."),
			skeleton_has_parents : y["skeletonHasParents"].as_i64().expect("Could not find item skeletonHasParents in HumanDescription."),
		}
	}

}

impl HumanDescription {
	pub fn get_serialized_version(&self) -> i64 { self.serialized_version }
	pub fn set_serialized_version(&mut self, value : i64) { self.serialized_version = value; }

	pub fn get_human(&self) -> Vec<Empty> { self.human.clone() }
	pub fn set_human(&mut self, value : Vec<Empty>) { self.human = value; }

	pub fn get_skeleton(&self) -> Vec<Empty> { self.skeleton.clone() }
	pub fn set_skeleton(&mut self, value : Vec<Empty>) { self.skeleton = value; }

	pub fn get_arm_twist(&self) -> f64 { self.arm_twist }
	pub fn set_arm_twist(&mut self, value : f64) { self.arm_twist = value; }

	pub fn get_fore_arm_twist(&self) -> f64 { self.fore_arm_twist }
	pub fn set_fore_arm_twist(&mut self, value : f64) { self.fore_arm_twist = value; }

	pub fn get_upper_leg_twist(&self) -> f64 { self.upper_leg_twist }
	pub fn set_upper_leg_twist(&mut self, value : f64) { self.upper_leg_twist = value; }

	pub fn get_leg_twist(&self) -> f64 { self.leg_twist }
	pub fn set_leg_twist(&mut self, value : f64) { self.leg_twist = value; }

	pub fn get_arm_stretch(&self) -> f64 { self.arm_stretch }
	pub fn set_arm_stretch(&mut self, value : f64) { self.arm_stretch = value; }

	pub fn get_leg_stretch(&self) -> f64 { self.leg_stretch }
	pub fn set_leg_stretch(&mut self, value : f64) { self.leg_stretch = value; }

	pub fn get_feet_spacing(&self) -> i64 { self.feet_spacing }
	pub fn set_feet_spacing(&mut self, value : i64) { self.feet_spacing = value; }

	pub fn get_root_motion_bone_name(&self) -> String { self.root_motion_bone_name.clone() }
	pub fn set_root_motion_bone_name(&mut self, value : String) { self.root_motion_bone_name = value; }

	pub fn get_root_motion_bone_rotation(&self) -> Vec4 { self.root_motion_bone_rotation.clone() }
	pub fn set_root_motion_bone_rotation(&mut self, value : Vec4) { self.root_motion_bone_rotation = value; }

	pub fn get_has_translation_do_f(&self) -> i64 { self.has_translation_do_f }
	pub fn set_has_translation_do_f(&mut self, value : i64) { self.has_translation_do_f = value; }

	pub fn get_has_extra_root(&self) -> i64 { self.has_extra_root }
	pub fn set_has_extra_root(&mut self, value : i64) { self.has_extra_root = value; }

	pub fn get_skeleton_has_parents(&self) -> i64 { self.skeleton_has_parents }
	pub fn set_skeleton_has_parents(&mut self, value : i64) { self.skeleton_has_parents = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct LastHumanDescriptionAvatarSource {
	instance_i_d : i64,

}

impl FromYaml for LastHumanDescriptionAvatarSource {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["LastHumanDescriptionAvatarSource"] != Yaml::BadValue {
			y = &yaml["LastHumanDescriptionAvatarSource"];
		}

		LastHumanDescriptionAvatarSource {
			instance_i_d : y["instanceID"].as_i64().expect("Could not find item instanceID in LastHumanDescriptionAvatarSource."),
		}
	}

}

impl LastHumanDescriptionAvatarSource {
	pub fn get_instance_i_d(&self) -> i64 { self.instance_i_d }
	pub fn set_instance_i_d(&mut self, value : i64) { self.instance_i_d = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct ModelImporter {
	serialized_version : i64,
	file_i_d_to_recycle_name : HashMap<String, String>,
	materials : Materials,
	animations : Animations,
	meshes : Meshes,
	tangent_space : TangentSpace,
	import_animation : i64,
	copy_avatar : i64,
	human_description : HumanDescription,
	last_human_description_avatar_source : LastHumanDescriptionAvatarSource,
	animation_type : i64,
	humanoid_oversampling : i64,
	additional_bone : i64,
	user_data : String,
	asset_bundle_name : String,
	asset_bundle_variant : String,

}

impl FromYaml for ModelImporter {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["ModelImporter"] != Yaml::BadValue {
			y = &yaml["ModelImporter"];
		}

		ModelImporter {
			serialized_version : y["serializedVersion"].as_i64().expect("Could not find item serializedVersion in ModelImporter."),
			file_i_d_to_recycle_name : HashMap::from_yaml(&y["fileIDToRecycleName"]),
			materials : Materials::from_yaml(&y["materials"]),
			animations : Animations::from_yaml(&y["animations"]),
			meshes : Meshes::from_yaml(&y["meshes"]),
			tangent_space : TangentSpace::from_yaml(&y["tangentSpace"]),
			import_animation : y["importAnimation"].as_i64().expect("Could not find item importAnimation in ModelImporter."),
			copy_avatar : y["copyAvatar"].as_i64().expect("Could not find item copyAvatar in ModelImporter."),
			human_description : HumanDescription::from_yaml(&y["humanDescription"]),
			last_human_description_avatar_source : LastHumanDescriptionAvatarSource::from_yaml(&y["lastHumanDescriptionAvatarSource"]),
			animation_type : y["animationType"].as_i64().expect("Could not find item animationType in ModelImporter."),
			humanoid_oversampling : y["humanoidOversampling"].as_i64().expect("Could not find item humanoidOversampling in ModelImporter."),
			additional_bone : y["additionalBone"].as_i64().expect("Could not find item additionalBone in ModelImporter."),
			user_data : String::from(""),
			asset_bundle_name : String::from(""),
			asset_bundle_variant : String::from(""),
		}
	}

}

impl ModelImporter {
	pub fn get_serialized_version(&self) -> i64 { self.serialized_version }
	pub fn set_serialized_version(&mut self, value : i64) { self.serialized_version = value; }

	pub fn get_file_i_d_to_recycle_name(&self) -> HashMap<String, String> { self.file_i_d_to_recycle_name.clone() }
	pub fn set_file_i_d_to_recycle_name(&mut self, value : HashMap<String, String>) { self.file_i_d_to_recycle_name = value; }

	pub fn get_materials(&self) -> Materials { self.materials.clone() }
	pub fn set_materials(&mut self, value : Materials) { self.materials = value; }

	pub fn get_animations(&self) -> Animations { self.animations.clone() }
	pub fn set_animations(&mut self, value : Animations) { self.animations = value; }

	pub fn get_meshes(&self) -> Meshes { self.meshes.clone() }
	pub fn set_meshes(&mut self, value : Meshes) { self.meshes = value; }

	pub fn get_tangent_space(&self) -> TangentSpace { self.tangent_space.clone() }
	pub fn set_tangent_space(&mut self, value : TangentSpace) { self.tangent_space = value; }

	pub fn get_import_animation(&self) -> i64 { self.import_animation }
	pub fn set_import_animation(&mut self, value : i64) { self.import_animation = value; }

	pub fn get_copy_avatar(&self) -> i64 { self.copy_avatar }
	pub fn set_copy_avatar(&mut self, value : i64) { self.copy_avatar = value; }

	pub fn get_human_description(&self) -> HumanDescription { self.human_description.clone() }
	pub fn set_human_description(&mut self, value : HumanDescription) { self.human_description = value; }

	pub fn get_last_human_description_avatar_source(&self) -> LastHumanDescriptionAvatarSource { self.last_human_description_avatar_source.clone() }
	pub fn set_last_human_description_avatar_source(&mut self, value : LastHumanDescriptionAvatarSource) { self.last_human_description_avatar_source = value; }

	pub fn get_animation_type(&self) -> i64 { self.animation_type }
	pub fn set_animation_type(&mut self, value : i64) { self.animation_type = value; }

	pub fn get_humanoid_oversampling(&self) -> i64 { self.humanoid_oversampling }
	pub fn set_humanoid_oversampling(&mut self, value : i64) { self.humanoid_oversampling = value; }

	pub fn get_additional_bone(&self) -> i64 { self.additional_bone }
	pub fn set_additional_bone(&mut self, value : i64) { self.additional_bone = value; }

	pub fn get_user_data(&self) -> String { self.user_data.clone() }
	pub fn set_user_data(&mut self, value : String) { self.user_data = value; }

	pub fn get_asset_bundle_name(&self) -> String { self.asset_bundle_name.clone() }
	pub fn set_asset_bundle_name(&mut self, value : String) { self.asset_bundle_name = value; }

	pub fn get_asset_bundle_variant(&self) -> String { self.asset_bundle_variant.clone() }
	pub fn set_asset_bundle_variant(&mut self, value : String) { self.asset_bundle_variant = value; }

}

#[cfg(test)]
mod model_importer_tests {
	use super::ModelImporter;
	use crate::file_types::asset_file::*;

	#[test]
	fn test_model_importer() {
		let mut path = std::env::current_dir().unwrap();
		path.push("src");
		path.push("file_types");
		path.push("templates");
		path.push("ModelImporter.yaml");
		println!("{}", path.display());
		let subject = <dyn AssetFile>::load_asset_file_from_path::<ModelImporter>(&path).unwrap();

		//Tests
		assert_eq!(subject.serialized_version, 19);
		assert_eq!(subject.import_animation, 0);
		assert_eq!(subject.copy_avatar, 0);
		assert_eq!(subject.animation_type, 0);
		assert_eq!(subject.humanoid_oversampling, 1);
		assert_eq!(subject.additional_bone, 0);
	}
}


