use crate::file_types::sections::Vec4;
use crate::from_yaml_trait::*;
use crate::file_types::sections::Vec3;
use unity_yaml_rust::Yaml;
use crate::file_types::sections::FileReference;
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
	object_hide_flags : i64,
	prefab_parent_object : FileReference,
	prefab_internal : FileReference,
	game_object : FileReference,
	local_rotation : Vec4,
	local_position : Vec3,
	local_scale : Vec3,
	children : Vec<FileReference>,
	father : FileReference,
	root_order : i64,
	local_euler_angles_hint : Vec3,

}

impl FromYaml for Transform {
	fn from_yaml(yaml : &Yaml) -> Self {
		let mut y = yaml;
		if yaml["Transform"] != Yaml::BadValue {
			y = &yaml["Transform"];
		}

		Transform {
			object_hide_flags : y["m_ObjectHideFlags"].as_i64().expect("Could not find item m_ObjectHideFlags in Transform."),
			prefab_parent_object : FileReference::from_yaml(&y["m_PrefabParentObject"]),
			prefab_internal : FileReference::from_yaml(&y["m_PrefabInternal"]),
			game_object : FileReference::from_yaml(&y["m_GameObject"]),
			local_rotation : Vec4::from_yaml(&y["m_LocalRotation"]),
			local_position : Vec3::from_yaml(&y["m_LocalPosition"]),
			local_scale : Vec3::from_yaml(&y["m_LocalScale"]),
			children : read_yaml_vector(&y["m_Children"], "m_Children"),
			father : FileReference::from_yaml(&y["m_Father"]),
			root_order : y["m_RootOrder"].as_i64().expect("Could not find item m_RootOrder in Transform."),
			local_euler_angles_hint : Vec3::from_yaml(&y["m_LocalEulerAnglesHint"]),
		}
	}

}

impl Transform {
	pub fn get_object_hide_flags(&self) -> i64 { self.object_hide_flags }
	pub fn set_object_hide_flags(&mut self, value : i64) { self.object_hide_flags = value; }

	pub fn get_prefab_parent_object(&self) -> FileReference { self.prefab_parent_object.clone() }
	pub fn set_prefab_parent_object(&mut self, value : FileReference) { self.prefab_parent_object = value; }

	pub fn get_prefab_internal(&self) -> FileReference { self.prefab_internal.clone() }
	pub fn set_prefab_internal(&mut self, value : FileReference) { self.prefab_internal = value; }

	pub fn get_game_object(&self) -> FileReference { self.game_object.clone() }
	pub fn set_game_object(&mut self, value : FileReference) { self.game_object = value; }

	pub fn get_local_rotation(&self) -> Vec4 { self.local_rotation.clone() }
	pub fn set_local_rotation(&mut self, value : Vec4) { self.local_rotation = value; }

	pub fn get_local_position(&self) -> Vec3 { self.local_position.clone() }
	pub fn set_local_position(&mut self, value : Vec3) { self.local_position = value; }

	pub fn get_local_scale(&self) -> Vec3 { self.local_scale.clone() }
	pub fn set_local_scale(&mut self, value : Vec3) { self.local_scale = value; }

	pub fn get_children(&self) -> Vec<FileReference> { self.children.clone() }
	pub fn set_children(&mut self, value : Vec<FileReference>) { self.children = value; }

	pub fn get_father(&self) -> FileReference { self.father.clone() }
	pub fn set_father(&mut self, value : FileReference) { self.father = value; }

	pub fn get_root_order(&self) -> i64 { self.root_order }
	pub fn set_root_order(&mut self, value : i64) { self.root_order = value; }

	pub fn get_local_euler_angles_hint(&self) -> Vec3 { self.local_euler_angles_hint.clone() }
	pub fn set_local_euler_angles_hint(&mut self, value : Vec3) { self.local_euler_angles_hint = value; }

}

#[cfg(test)]
mod transform_tests {
	use super::Transform;
	use crate::file_types::asset_file::*;

	#[test]
	fn test_transform() {
		let mut path = std::env::current_dir().unwrap();
		path.push("src");
		path.push("file_types");
		path.push("templates");
		path.push("Transform.yaml");
		println!("{}", path.display());
		let subject = <dyn AssetFile>::load_asset_file_from_path::<Transform>(&path).unwrap();

		//Tests
		assert_eq!(subject.object_hide_flags, 0);
		assert_eq!(subject.children[0].get_file_id(), 1748628747);
		assert_eq!(subject.children[1].get_file_id(), 183721988);
		assert_eq!(subject.children[2].get_file_id(), 689288899);
		assert_eq!(subject.children[3].get_file_id(), 2147160321);
		assert_eq!(subject.children[4].get_file_id(), 938755939);
		assert_eq!(subject.children[5].get_file_id(), 313933430);
		assert_eq!(subject.children[6].get_file_id(), 1031628990);
		assert_eq!(subject.children[7].get_file_id(), 611164213);
		assert_eq!(subject.children[8].get_file_id(), 1370624015);
		assert_eq!(subject.children[9].get_file_id(), 1942132880);
		assert_eq!(subject.children[10].get_file_id(), 601973292);
		assert_eq!(subject.children[11].get_file_id(), 1105893663);
		assert_eq!(subject.children[12].get_file_id(), 993209406);
		assert_eq!(subject.children[13].get_file_id(), 1037936086);
		assert_eq!(subject.children[14].get_file_id(), 866722117);
		assert_eq!(subject.children[15].get_file_id(), 1813860022);
		assert_eq!(subject.children[16].get_file_id(), 2080211418);
		assert_eq!(subject.children[17].get_file_id(), 1628464829);
		assert_eq!(subject.children[18].get_file_id(), 230494526);
		assert_eq!(subject.children[19].get_file_id(), 2057692222);
		assert_eq!(subject.children[20].get_file_id(), 968466858);
		assert_eq!(subject.children[21].get_file_id(), 1184336827);
		assert_eq!(subject.children[22].get_file_id(), 648376325);
		assert_eq!(subject.children[23].get_file_id(), 1891003711);
		assert_eq!(subject.children[24].get_file_id(), 944304931);
		assert_eq!(subject.children[25].get_file_id(), 8127960);
		assert_eq!(subject.children[26].get_file_id(), 1190266619);
		assert_eq!(subject.children[27].get_file_id(), 390852523);
		assert_eq!(subject.children[28].get_file_id(), 1532783121);
		assert_eq!(subject.children[29].get_file_id(), 1750451121);
		assert_eq!(subject.children[30].get_file_id(), 1601291051);
		assert_eq!(subject.children[31].get_file_id(), 1467180309);
		assert_eq!(subject.children[32].get_file_id(), 1313849013);
		assert_eq!(subject.children[33].get_file_id(), 1623000807);
		assert_eq!(subject.children[34].get_file_id(), 1436129167);
		assert_eq!(subject.children[35].get_file_id(), 244223352);
		assert_eq!(subject.children[36].get_file_id(), 1331919703);
		assert_eq!(subject.children[37].get_file_id(), 218567362);
		assert_eq!(subject.children[38].get_file_id(), 1119039488);
		assert_eq!(subject.children[39].get_file_id(), 1689563919);
		assert_eq!(subject.children[40].get_file_id(), 1788155498);
		assert_eq!(subject.children[41].get_file_id(), 478838311);
		assert_eq!(subject.children[42].get_file_id(), 1580138967);
		assert_eq!(subject.children[43].get_file_id(), 1050019619);
		assert_eq!(subject.children[44].get_file_id(), 234844877);
		assert_eq!(subject.children[45].get_file_id(), 337640555);
		assert_eq!(subject.children[46].get_file_id(), 1575864048);
		assert_eq!(subject.children[47].get_file_id(), 2117599470);
		assert_eq!(subject.children[48].get_file_id(), 1553980337);
		assert_eq!(subject.children[49].get_file_id(), 736950887);
		assert_eq!(subject.children[50].get_file_id(), 1208840059);
		assert_eq!(subject.children[51].get_file_id(), 1860799900);
		assert_eq!(subject.children[52].get_file_id(), 1958770828);
		assert_eq!(subject.children[53].get_file_id(), 1466836370);
		assert_eq!(subject.children[54].get_file_id(), 343001153);
		assert_eq!(subject.children[55].get_file_id(), 24159643);
		assert_eq!(subject.children[56].get_file_id(), 1497580541);
		assert_eq!(subject.children[57].get_file_id(), 1556487169);
		assert_eq!(subject.children[58].get_file_id(), 1797411350);
		assert_eq!(subject.children[59].get_file_id(), 912115401);
		assert_eq!(subject.children[60].get_file_id(), 760247744);
		assert_eq!(subject.children[61].get_file_id(), 1452373809);
		assert_eq!(subject.children[62].get_file_id(), 1937220904);
		assert_eq!(subject.children[63].get_file_id(), 1768297928);
		assert_eq!(subject.children[64].get_file_id(), 2021676728);
		assert_eq!(subject.children[65].get_file_id(), 1077719015);
		assert_eq!(subject.children[66].get_file_id(), 2034005739);
		assert_eq!(subject.children[67].get_file_id(), 2121130844);
		assert_eq!(subject.children[68].get_file_id(), 1408068900);
		assert_eq!(subject.children[69].get_file_id(), 122146757);
		assert_eq!(subject.children[70].get_file_id(), 1027038241);
		assert_eq!(subject.children[71].get_file_id(), 727763373);
		assert_eq!(subject.children[72].get_file_id(), 853328980);
		assert_eq!(subject.children[73].get_file_id(), 1270759069);
		assert_eq!(subject.children[74].get_file_id(), 1955350946);
		assert_eq!(subject.children[75].get_file_id(), 852848155);
		assert_eq!(subject.children[76].get_file_id(), 920254529);
		assert_eq!(subject.children[77].get_file_id(), 458273341);
		assert_eq!(subject.children[78].get_file_id(), 682588130);
		assert_eq!(subject.children[79].get_file_id(), 2034036667);
		assert_eq!(subject.children[80].get_file_id(), 918540044);
		assert_eq!(subject.children[81].get_file_id(), 180745310);
		assert_eq!(subject.children[82].get_file_id(), 2127177171);
		assert_eq!(subject.children[83].get_file_id(), 1120477194);
		assert_eq!(subject.children[84].get_file_id(), 321252467);
		assert_eq!(subject.children[85].get_file_id(), 1435778548);
		assert_eq!(subject.children[86].get_file_id(), 1772794545);
		assert_eq!(subject.children[87].get_file_id(), 1648942076);
		assert_eq!(subject.children[88].get_file_id(), 1510291356);
		assert_eq!(subject.children[89].get_file_id(), 1399754182);
		assert_eq!(subject.children[90].get_file_id(), 140316874);
		assert_eq!(subject.children[91].get_file_id(), 1021350563);
		assert_eq!(subject.children[92].get_file_id(), 53566050);
		assert_eq!(subject.children[93].get_file_id(), 1110365436);
		assert_eq!(subject.children[94].get_file_id(), 1847213700);
		assert_eq!(subject.children[95].get_file_id(), 627101629);
		assert_eq!(subject.children[96].get_file_id(), 308151231);
		assert_eq!(subject.children[97].get_file_id(), 263150264);
		assert_eq!(subject.children[98].get_file_id(), 266781206);
		assert_eq!(subject.children[99].get_file_id(), 602057372);
		assert_eq!(subject.children[100].get_file_id(), 1408819837);
		assert_eq!(subject.children[101].get_file_id(), 2017193308);
		assert_eq!(subject.children[102].get_file_id(), 1404442260);
		assert_eq!(subject.children[103].get_file_id(), 722432057);
		assert_eq!(subject.children[104].get_file_id(), 2064590507);
		assert_eq!(subject.children[105].get_file_id(), 1296674847);
		assert_eq!(subject.children[106].get_file_id(), 1192060071);
		assert_eq!(subject.children[107].get_file_id(), 1141195088);
		assert_eq!(subject.children[108].get_file_id(), 708385283);
		assert_eq!(subject.children[109].get_file_id(), 2046643659);
		assert_eq!(subject.children[110].get_file_id(), 1847034241);
		assert_eq!(subject.children[111].get_file_id(), 433132150);
		assert_eq!(subject.children[112].get_file_id(), 749525701);
		assert_eq!(subject.children[113].get_file_id(), 936101131);
		assert_eq!(subject.children[114].get_file_id(), 1377439288);
		assert_eq!(subject.children[115].get_file_id(), 1100651172);
		assert_eq!(subject.children[116].get_file_id(), 195691183);
		assert_eq!(subject.children[117].get_file_id(), 1646290245);
		assert_eq!(subject.children[118].get_file_id(), 1615438978);
		assert_eq!(subject.children[119].get_file_id(), 1914270280);
		assert_eq!(subject.children[120].get_file_id(), 1964209578);
		assert_eq!(subject.children[121].get_file_id(), 779593432);
		assert_eq!(subject.children[122].get_file_id(), 1335684121);
		assert_eq!(subject.children[123].get_file_id(), 1826456543);
		assert_eq!(subject.children[124].get_file_id(), 188508203);
		assert_eq!(subject.children[125].get_file_id(), 1246702372);
		assert_eq!(subject.children[126].get_file_id(), 606995485);
		assert_eq!(subject.children[127].get_file_id(), 766483882);
		assert_eq!(subject.children[128].get_file_id(), 886077549);
		assert_eq!(subject.root_order, 25);
	}
}


