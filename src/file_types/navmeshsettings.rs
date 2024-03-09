use crate::from_yaml_trait::*;
use crate::file_types::sections::FileReference;
use unity_yaml_rust::Yaml;

#[derive(Debug, Clone, PartialEq)]
pub struct BuildSettings {
	serialized_version : i64,
	agent_type_i_d : i64,
	agent_radius : f64,
	agent_height : i64,
	agent_slope : i64,
	agent_climb : f64,
	ledge_drop_height : i64,
	max_jump_across_distance : i64,
	min_region_area : i64,
	manual_cell_size : i64,
	cell_size : f64,
	manual_tile_size : i64,
	tile_size : i64,
	accurate_placement : i64,

}

impl FromYaml for BuildSettings {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["BuildSettings"] != Yaml::BadValue {
			y = &yaml["BuildSettings"];
		}

		BuildSettings {
			serialized_version : y["serializedVersion"].as_i64().expect("Could not find item serializedVersion in BuildSettings."),
			agent_type_i_d : y["agentTypeID"].as_i64().expect("Could not find item agentTypeID in BuildSettings."),
			agent_radius : y["agentRadius"].as_f64().expect("Could not find item agentRadius in BuildSettings."),
			agent_height : y["agentHeight"].as_i64().expect("Could not find item agentHeight in BuildSettings."),
			agent_slope : y["agentSlope"].as_i64().expect("Could not find item agentSlope in BuildSettings."),
			agent_climb : y["agentClimb"].as_f64().expect("Could not find item agentClimb in BuildSettings."),
			ledge_drop_height : y["ledgeDropHeight"].as_i64().expect("Could not find item ledgeDropHeight in BuildSettings."),
			max_jump_across_distance : y["maxJumpAcrossDistance"].as_i64().expect("Could not find item maxJumpAcrossDistance in BuildSettings."),
			min_region_area : y["minRegionArea"].as_i64().expect("Could not find item minRegionArea in BuildSettings."),
			manual_cell_size : y["manualCellSize"].as_i64().expect("Could not find item manualCellSize in BuildSettings."),
			cell_size : y["cellSize"].as_f64().expect("Could not find item cellSize in BuildSettings."),
			manual_tile_size : y["manualTileSize"].as_i64().expect("Could not find item manualTileSize in BuildSettings."),
			tile_size : y["tileSize"].as_i64().expect("Could not find item tileSize in BuildSettings."),
			accurate_placement : y["accuratePlacement"].as_i64().expect("Could not find item accuratePlacement in BuildSettings."),
		}
	}

}

impl BuildSettings {
	pub fn get_serialized_version(&self) -> i64 { self.serialized_version }
	pub fn set_serialized_version(&mut self, value : i64) { self.serialized_version = value; }

	pub fn get_agent_type_i_d(&self) -> i64 { self.agent_type_i_d }
	pub fn set_agent_type_i_d(&mut self, value : i64) { self.agent_type_i_d = value; }

	pub fn get_agent_radius(&self) -> f64 { self.agent_radius }
	pub fn set_agent_radius(&mut self, value : f64) { self.agent_radius = value; }

	pub fn get_agent_height(&self) -> i64 { self.agent_height }
	pub fn set_agent_height(&mut self, value : i64) { self.agent_height = value; }

	pub fn get_agent_slope(&self) -> i64 { self.agent_slope }
	pub fn set_agent_slope(&mut self, value : i64) { self.agent_slope = value; }

	pub fn get_agent_climb(&self) -> f64 { self.agent_climb }
	pub fn set_agent_climb(&mut self, value : f64) { self.agent_climb = value; }

	pub fn get_ledge_drop_height(&self) -> i64 { self.ledge_drop_height }
	pub fn set_ledge_drop_height(&mut self, value : i64) { self.ledge_drop_height = value; }

	pub fn get_max_jump_across_distance(&self) -> i64 { self.max_jump_across_distance }
	pub fn set_max_jump_across_distance(&mut self, value : i64) { self.max_jump_across_distance = value; }

	pub fn get_min_region_area(&self) -> i64 { self.min_region_area }
	pub fn set_min_region_area(&mut self, value : i64) { self.min_region_area = value; }

	pub fn get_manual_cell_size(&self) -> i64 { self.manual_cell_size }
	pub fn set_manual_cell_size(&mut self, value : i64) { self.manual_cell_size = value; }

	pub fn get_cell_size(&self) -> f64 { self.cell_size }
	pub fn set_cell_size(&mut self, value : f64) { self.cell_size = value; }

	pub fn get_manual_tile_size(&self) -> i64 { self.manual_tile_size }
	pub fn set_manual_tile_size(&mut self, value : i64) { self.manual_tile_size = value; }

	pub fn get_tile_size(&self) -> i64 { self.tile_size }
	pub fn set_tile_size(&mut self, value : i64) { self.tile_size = value; }

	pub fn get_accurate_placement(&self) -> i64 { self.accurate_placement }
	pub fn set_accurate_placement(&mut self, value : i64) { self.accurate_placement = value; }

}

#[derive(Debug, Clone, PartialEq)]
pub struct NavMeshSettings {
	serialized_version : i64,
	object_hide_flags : i64,
	build_settings : BuildSettings,
	nav_mesh_data : FileReference,

}

impl FromYaml for NavMeshSettings {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["NavMeshSettings"] != Yaml::BadValue {
			y = &yaml["NavMeshSettings"];
		}

		NavMeshSettings {
			serialized_version : y["serializedVersion"].as_i64().expect("Could not find item serializedVersion in NavMeshSettings."),
			object_hide_flags : y["m_ObjectHideFlags"].as_i64().expect("Could not find item m_ObjectHideFlags in NavMeshSettings."),
			build_settings : BuildSettings::from_yaml(&y["m_BuildSettings"]),
			nav_mesh_data : FileReference::from_yaml(&y["m_NavMeshData"]),
		}
	}

}

impl NavMeshSettings {
	pub fn get_serialized_version(&self) -> i64 { self.serialized_version }
	pub fn set_serialized_version(&mut self, value : i64) { self.serialized_version = value; }

	pub fn get_object_hide_flags(&self) -> i64 { self.object_hide_flags }
	pub fn set_object_hide_flags(&mut self, value : i64) { self.object_hide_flags = value; }

	pub fn get_build_settings(&self) -> BuildSettings { self.build_settings.clone() }
	pub fn set_build_settings(&mut self, value : BuildSettings) { self.build_settings = value; }

	pub fn get_nav_mesh_data(&self) -> FileReference { self.nav_mesh_data.clone() }
	pub fn set_nav_mesh_data(&mut self, value : FileReference) { self.nav_mesh_data = value; }

}

#[cfg(test)]
mod nav_mesh_settings_tests {
	use super::NavMeshSettings;
	use crate::file_types::asset_file::*;

	#[test]
	fn test_nav_mesh_settings() {
		let mut path = std::env::current_dir().unwrap();
		path.push("src");
		path.push("file_types");
		path.push("templates");
		path.push("NavMeshSettings.yaml");
		println!("{}", path.display());
		let subject = <dyn AssetFile>::load_asset_file_from_path::<NavMeshSettings>(&path).unwrap();

		//Tests
		assert_eq!(subject.serialized_version, 2);
		assert_eq!(subject.object_hide_flags, 0);
	}
}


