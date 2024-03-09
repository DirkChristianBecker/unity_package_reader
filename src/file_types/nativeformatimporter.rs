use unity_yaml_rust::Yaml;
use crate::from_yaml_trait::*;
use crate::file_types::sections::FileReference;

#[derive(Debug, Clone, PartialEq)]
pub struct NativeFormatImporter {
	main_object_file_i_d : i64,
	user_data : String,
	asset_bundle_name : String,
	asset_bundle_variant : String,

}

impl FromYaml for NativeFormatImporter {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["NativeFormatImporter"] != Yaml::BadValue {
			y = &yaml["NativeFormatImporter"];
		}

		NativeFormatImporter {
			main_object_file_i_d : y["mainObjectFileID"].as_i64().expect("Could not find item mainObjectFileID in NativeFormatImporter."),
			user_data : String::from(""),
			asset_bundle_name : String::from(""),
			asset_bundle_variant : String::from(""),
		}
	}

}

impl NativeFormatImporter {
	pub fn get_main_object_file_i_d(&self) -> i64 { self.main_object_file_i_d }
	pub fn set_main_object_file_i_d(&mut self, value : i64) { self.main_object_file_i_d = value; }

	pub fn get_user_data(&self) -> String { self.user_data.clone() }
	pub fn set_user_data(&mut self, value : String) { self.user_data = value; }

	pub fn get_asset_bundle_name(&self) -> String { self.asset_bundle_name.clone() }
	pub fn set_asset_bundle_name(&mut self, value : String) { self.asset_bundle_name = value; }

	pub fn get_asset_bundle_variant(&self) -> String { self.asset_bundle_variant.clone() }
	pub fn set_asset_bundle_variant(&mut self, value : String) { self.asset_bundle_variant = value; }

}

#[cfg(test)]
mod native_format_importer_tests {
	use super::NativeFormatImporter;
	use crate::file_types::asset_file::*;

	#[test]
	fn test_native_format_importer() {
		let mut path = std::env::current_dir().unwrap();
		path.push("src");
		path.push("file_types");
		path.push("templates");
		path.push("NativeFormatImporter.yaml");
		println!("{}", path.display());
		let subject = <dyn AssetFile>::load_asset_file_from_path::<NativeFormatImporter>(&path).unwrap();

		//Tests
		assert_eq!(subject.main_object_file_i_d, 2100000);
	}
}


