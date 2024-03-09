use rust_tools::prelude::{camel_case_to_snake_case, capitalize_first_letter};
use std::io::prelude::*;
use std::path::PathBuf;
use std::{collections::HashSet, fs::OpenOptions};
use unity_yaml_rust::{AssetType, Yaml, YamlLoader};

const USING_VEC: &str = "use std::vec::Vec;";
const USING_EMPTY: &str = "use crate::file_types::sections::Empty;";
const USING_FILE_REFERENCE: &str = "use crate::file_types::sections::FileReference;";
const USING_TEXTURE: &str = "use crate::file_types::sections::Texture;";
const USING_VEC2: &str = "use crate::file_types::sections::Vec2;";
const USING_VEC3: &str = "use crate::file_types::sections::Vec3;";
const USING_VEC4: &str = "use crate::file_types::sections::Vec4;";
const USING_COLOR: &str = "use crate::file_types::sections::Color;";
const USING_MODIFICATIOM: &str = "use crate::file_types::sections::PrefabModification;";
const USING_FALLOFF_TABLE: &str = "use crate::file_types::sections::FalloffTable;";
const USING_NAMED_VALUE: &str = "use crate::file_types::sections::NamedValue;";
const USING_BUILD_TARGET: &str = "use crate::file_types::sections::BuildTarget;";
const USING_HASHMAP: &str = "use std::collections::HashMap;";

fn get_templates_path() -> PathBuf {
    let mut p = std::env::current_dir().unwrap();
    p.push("src");
    p.push("file_types");
    p.push("templates");

    p
}

fn get_output_path() -> PathBuf {
    let mut p = std::env::current_dir().unwrap();
    p.push("src");
    p.push("file_types");

    p
}

fn log(message: &str) {
    let mut path = get_output_path();
    path.push("debug_output.txt");

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    if let Err(e) = write!(file, "{}", message) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn write_file(content: &str, yaml_type: &str) {
    let mut path = get_output_path();
    let mut t = yaml_type.to_lowercase();
    t.push_str(".rs");

    path.push(&t);

    if path.exists() {
        match std::fs::remove_file(path.clone()) {
            Ok(_) => {}
            Err(e) => {
                panic!("Could not delete old target file. {}", e)
            }
        }
    }

    let parent = path.parent().expect("Parent should always exist.");
    if !parent.exists() {
        match std::fs::create_dir_all(parent) {
            Ok(_) => {}
            Err(e) => {
                panic!(
                    "Could not create path for target file. {} \n{:?}",
                    e, parent
                )
            }
        }
    }

    match std::fs::File::create(path.clone()) {
        Ok(_) => {}
        Err(e) => {
            panic!("Could not create target file. {}\n{:?}", e, path)
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", content) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

/// Ensures, that we always have consistent types when reading arrays.
/// Compares the last type we encountered with the new one.
/// If they match returns true and the current type. If either of the
/// two given types is a f64 and the other one an i64, f64 is returned.
/// Because it is save to store a i64 in a f64, but not the other way around.
fn check_if_continue_equal(a: &String, b: &String) -> (bool, String) {
    if !a.is_empty() {
        if a == "i64" && b == "f64" {
            return (true, b.to_owned());
        }

        if a == "f64" && b == "i64" {
            return (true, a.to_owned());
        }

        if a == b {
            return (true, b.to_owned());
        }

        return (false, a.to_owned());
    }

    (true, b.to_owned())
}

/// Returns the found type and a using if one is needed.
fn analyze_vector_type(yaml: &Yaml, _i: usize) -> (String, Option<&str>) {
    match yaml {
        Yaml::Real(_) => (String::from("f64"), None),
        Yaml::Integer(_) => (String::from("i64"), None),
        Yaml::String(_) => (String::from("String"), None),
        Yaml::Boolean(_) => (String::from("bool"), None),
        Yaml::Array(ref a) => {
            if a.is_empty() {
                return (String::from("Vec<Empty>"), Some(USING_EMPTY));
            }

            let mut s = String::from("");
            let mut u = "";
            for x in a {
                let (y, z) = analyze_vector_type(x, 0);
                if let Some(using) = z {
                    u = using;
                }
                let (go_on, value) = check_if_continue_equal(&s, &y);
                if !go_on {
                    panic!("Could not determine the type of an array.");
                }

                s = value;
            }

            let t = format!("Vec<{}>", s);
            if u.is_empty() {
                (t, None)
            } else {
                (t, Some(u))
            }
        }

        Yaml::Hash(ref hash) => {
            if hash.has_all(vec!["m_Texture", "m_Scale", "m_Offset"]) {
                return (String::from("Texture"), Some(USING_TEXTURE));
            }

            if hash.has_all(vec!["r", "g", "b", "a"]) {
                return (String::from("Color"), Some(USING_COLOR));
            }

            if hash.has_all(vec!["x", "y", "z", "w"]) {
                return (String::from("Vec4"), Some(USING_VEC4));
            }

            if hash.has_all(vec!["x", "y", "z"]) {
                return (String::from("Vec3"), Some(USING_VEC3));
            }

            if hash.has_all(vec!["x", "y"]) {
                return (String::from("Vec2"), Some(USING_VEC2));
            }

            if hash.has_all(vec!["target", "propertyPath", "value", "objectReference"]) {
                return (String::from("PrefabModification"), Some(USING_MODIFICATIOM));
            }

            if hash.has_all(vec![
                "buildTarget",
                "maxTextureSize",
                "textureFormat",
                "textureCompression",
                "compressionQuality",
                "crunchedCompression",
                "allowsAlphaSplitting",
                "overridden",
            ]) {
                return (String::from("BuildTarget"), Some(USING_BUILD_TARGET));
            }

            if hash.has_all(vec!["fileID"]) {
                return (String::from("FileReference"), Some(USING_FILE_REFERENCE));
            }

            let mut s = String::from("");
            let mut u = "";

            for (k, v) in hash.iter() {
                let (y, z) = analyze_vector_type(v, 0);
                if let Some(using) = z {
                    u = using;
                }

                let (go_on, value) = check_if_continue_equal(&s, &y);
                if !go_on {
                    let message = format!(
                        "Could not determine the type of an array. s: {} y: {} Key: {:?} -> Value: {:?}\n",
                        s, y, k, v
                    );
                    log(&message);
                    panic!("{}", message);
                }

                s = value;
            }

            if s == "i64" || s == "f64" || s == "bool" || s == "String" {
                if s == "i64" {
                    s = "f64".to_string();
                }

                return (format!("NamedValue<{}>", s), Some(USING_NAMED_VALUE));
            }

            if u.is_empty() {
                (s, None)
            } else {
                (s, Some(u))
            }
        }
        _ => {
            panic!("Illegal value");
        }
    }
}

/// Returns
/// the name of the type
/// all the usings it needs to compile
/// and the code for the struct
fn handle_yaml_hash(
    hash: &unity_yaml_rust::yaml::Hash,
    asset_type_name: &str,
    level: u64,
) -> (String, HashSet<String>, String) {
    if hash.map.is_empty() {
        return (
            String::from("Empty"),
            std::collections::HashSet::new(),
            String::from(""),
        );
    }

    let mut asset_name = asset_type_name.to_string();
    if asset_name.starts_with("m_") {
        asset_name.remove(0);
        asset_name.remove(0);
    }
    asset_name = capitalize_first_letter(&asset_name);

    let mut usings: HashSet<String> = std::collections::HashSet::new();
    if asset_name == "FileIDToRecycleName" {
        usings.insert(USING_HASHMAP.to_string());
        return ("HashMap<String, String>".to_string(), usings, String::from(""));
    }

    usings.insert("use unity_yaml_rust::Yaml;".to_string());
    usings.insert("use crate::from_yaml_trait::*;".to_string());

    // Default types
    if hash.has_all(vec!["r", "g", "b", "a"]) {
        usings.insert(USING_COLOR.to_string());
        return ("Color".to_string(), usings, String::from(""));
    }

    if hash.has_all(vec!["x", "y", "z", "w"]) {
        usings.insert(USING_VEC4.to_string());
        return ("Vec4".to_string(), usings, String::from(""));
    }

    if hash.has_all(vec!["x", "y", "z"]) {
        usings.insert(USING_VEC3.to_string());
        return ("Vec3".to_string(), usings, String::from(""));
    }

    if hash.has_all(vec!["x", "y"]) {
        usings.insert(USING_VEC2.to_string());
        return ("Vec2".to_string(), usings, String::from(""));
    }

    if asset_type_name == "m_FalloffTable" {
        usings.insert(USING_FALLOFF_TABLE.to_string());
        return ("FalloffTable".to_string(), usings, String::from(""));
    }

    let snake_case_asset_name = camel_case_to_snake_case(&asset_name);

    let mut implementation = format!("impl FromYaml for {asset_name} {{\n");
    let mut from_yaml_function = "\tfn from_yaml(yaml : &Yaml) -> Self {\n".to_string();
    from_yaml_function.push_str("\t\tlet mut y = yaml;\n");
    from_yaml_function.push_str(&format!(
        "\t\tif yaml[\"{asset_name}\"] != Yaml::BadValue {{\n"
    ));
    from_yaml_function.push_str(&format!("\t\t\ty = &yaml[\"{asset_name}\"];\n\t\t}}\n"));
    from_yaml_function.push_str(&format!("\n\t\t{asset_name} {{\n"));

    let mut getter_setter = format!("impl {asset_name} {{\n");
    let mut code = format!("#[derive(Debug, Clone, PartialEq)]\npub struct {asset_name} {{\n");
    let mut test =
        format!("#[cfg(test)]\nmod {snake_case_asset_name}_tests {{\n\tuse super::{asset_name};\n");

    test.push_str("\tuse crate::file_types::asset_file::*;\n\n");
    test.push_str(&format!(
        "\t#[test]\n\tfn test_{snake_case_asset_name}() {{\n"
    ));
    test.push_str("\t\tlet mut path = std::env::current_dir().unwrap();\n");
    test.push_str("\t\tpath.push(\"src\");\n");
    test.push_str("\t\tpath.push(\"file_types\");\n");
    test.push_str("\t\tpath.push(\"templates\");\n");
    test.push_str(&format!("\t\tpath.push(\"{asset_name}.yaml\");\n"));
    test.push_str("\t\tprintln!(\"{}\", path.display());\n");
    test.push_str(&format!("\t\tlet subject = <dyn AssetFile>::load_asset_file_from_path::<{asset_name}>(&path).unwrap();\n\n\t\t//Tests\n"));

    let mut this_file = String::from("");

    for (k, v) in &hash.map {
        let mut is_reference = true;
        let name = format!("{}", k);
        let mut typ = String::from("");

        if name == "fileID" {
            if !usings.contains(USING_FILE_REFERENCE) {
                usings.contains(USING_FILE_REFERENCE);
            }
            return (String::from("FileReference"), usings, String::from(""));
        }

        let mut snake_case_name = camel_case_to_snake_case(&name);

        // type is a rust key word
        if snake_case_name == "type" {
            snake_case_name = format!("{snake_case_asset_name}_type")
        }

        let error_msg = format!("\"Could not find item {name} in {asset_name}.\"");
        match v {
            Yaml::Real(r) => {
                typ.push_str("f64");
                is_reference = false;

                from_yaml_function.push_str(&format!(
                    "\t\t\t{snake_case_name} : y[\"{name}\"].as_f64().expect({error_msg}),\n"
                ));

                test.push_str(&format!(
                    "\t\tassert_eq!(subject.{snake_case_name}, {r});\n"
                ));
            }
            Yaml::Integer(i) => {
                typ.push_str("i64");
                is_reference = false;

                from_yaml_function.push_str(&format!(
                    "\t\t\t{snake_case_name} : y[\"{name}\"].as_i64().expect({error_msg}),\n"
                ));

                test.push_str(&format!(
                    "\t\tassert_eq!(subject.{snake_case_name}, {i});\n"
                ));
            }
            Yaml::String(s) => {
                typ.push_str("String");
                is_reference = true;

                from_yaml_function.push_str(&format!(
                    "\t\t\t{snake_case_name} : String::from(y[\"{name}\"].as_str().expect({error_msg})),\n"));

                test.push_str(&format!(
                    "\t\tassert_eq!(subject.{}, String::from(\"{}\"));\n",
                    snake_case_name, s
                ));
            }
            Yaml::Boolean(b) => {
                typ.push_str("bool");
                is_reference = false;

                from_yaml_function.push_str(&format!(
                    "\t\t\t{snake_case_name} : y[\"{name}\"].as_bool().expect({error_msg}),\n"
                ));

                test.push_str(&format!(
                    "\t\tassert_eq!(subject.{}, String::from(\"{}\"));\n",
                    snake_case_name, b
                ));
            }
            Yaml::Null => {
                is_reference = true;
                typ.push_str("String");

                from_yaml_function
                    .push_str(&format!("\t\t\t{snake_case_name} : String::from(\"\"),\n"));
            }
            Yaml::Array(ref a) => {
                if !usings.contains(USING_VEC) {
                    usings.insert(String::from(USING_VEC));
                }

                if a.is_empty() {
                    if !usings.contains(USING_EMPTY) {
                        usings.insert(String::from(USING_EMPTY));
                    }
                    typ.push_str("Vec<Empty>");

                    from_yaml_function
                        .push_str(&format!("\t\t\t{snake_case_name} : Vec::new(),\n"));
                } else {
                    from_yaml_function.push_str(&format!(
                        "\t\t\t{snake_case_name} : read_yaml_vector(&y[\"{name}\"], \"{name}\"),\n"
                    ));

                    let mut x = String::from("");
                    let mut u = "";
                    for (i, e) in a.iter().enumerate() {
                        let (y, z) = analyze_vector_type(e, i);
                        if let Some(using) = z {
                            u = using;
                        }

                        let (go_on, value) = check_if_continue_equal(&x, &y);
                        if !go_on {
                            panic!("Could not determine the type of an array.");
                        }

                        if y == "FileReference" {
                            // log(&format!("{}", e));
                            let value2;
                            if e["fileID"] == Yaml::BadValue {
                                let (_, val) = e.as_hash().unwrap().iter().next().unwrap();
                                value2 = val["fileID"].as_i64().unwrap() as u64;
                                test.push_str(&format!(
                                    "\t\tassert_eq!(subject.{}[{}].get_file_id(), {});\n",
                                    snake_case_name, i, value2
                                ));
                            } else {
                                value2 = e["fileID"].as_i64().unwrap() as u64;
                                test.push_str(&format!(
                                    "\t\tassert_eq!(subject.{}[{}].get_file_id(), {});\n",
                                    snake_case_name, i, value2
                                ));
                            }
                        }
                        x = value;
                    }

                    if !u.is_empty() && !usings.contains(u) {
                        usings.insert(u.to_string());
                    }

                    typ.push_str(&format!("Vec<{}>", x));
                }
            }

            Yaml::Hash(ref h) => {
                let (t, us, c) = handle_yaml_hash(h, &name, level + 1);
                usings.extend(us);
                this_file.push_str(&c);
                typ = t;

                if typ.is_empty() {
                    typ = String::from("String");
                }

                let typ2 = typ.replace("<String, String>", "");

                from_yaml_function.push_str(&format!(
                    "\t\t\t{snake_case_name} : {typ2}::from_yaml(&y[\"{name}\"]),\n"
                ));

                if level == 0 {
                    return (typ, usings, this_file);
                }
            }
            _ => {}
        }

        code.push_str(&format!("\t{} : {},\n", snake_case_name, typ));

        if is_reference {
            getter_setter.push_str(&format!(
                "\tpub fn get_{}(&self) -> {} {{ self.{}.clone() }}\n",
                &snake_case_name, typ, &snake_case_name
            ));
            getter_setter.push_str(&format!(
                "\tpub fn set_{}(&mut self, value : {}) {{ self.{} = value; }}\n\n",
                &snake_case_name, typ, &snake_case_name
            ));
        } else {
            getter_setter.push_str(&format!(
                "\tpub fn get_{}(&self) -> {} {{ self.{} }}\n",
                &snake_case_name, typ, &snake_case_name
            ));
            getter_setter.push_str(&format!(
                "\tpub fn set_{}(&mut self, value : {}) {{ self.{} = value; }}\n\n",
                &snake_case_name, typ, &snake_case_name
            ));
        }
    }

    test.push_str("\t}\n}\n\n");
    getter_setter.push_str("}\n\n");
    from_yaml_function.push_str("\t\t}\n\t}\n\n");
    implementation.push_str(&from_yaml_function);
    implementation.push_str("}\n\n");
    implementation.push_str(&getter_setter);
    code.push_str("\n}\n\n");
    code.push_str(&implementation);

    if level == 1 {
        code.push_str(&test);
    }
    this_file.push_str(&code);

    if level == 1 {
        if !usings.contains(USING_FILE_REFERENCE) {
            usings.insert(USING_FILE_REFERENCE.to_string());
        }

        let mut usings_str = usings.into_iter().collect::<Vec<String>>().join("\n");
        usings_str.push_str("\n\n");
        usings_str.push_str(&this_file);

        // log(&usings_str);
        write_file(&usings_str, &asset_name);

        (asset_name, HashSet::new(), usings_str)
    } else {
        (asset_name, usings, this_file)
    }
}

fn handle_yaml_file(path: &PathBuf) -> Vec<String> {
    let docs = YamlLoader::load_from_path(path).unwrap();
    let mut asset_type = String::from("");
    let mut items: Vec<String> = Vec::new();

    for doc in docs {
        match doc {
            Yaml::Hash(ref hash) => {
                let (mut typ, _usings, _code) = handle_yaml_hash(hash, &asset_type, 0);
                if typ.is_empty() {
                    typ = String::from("String");
                }
                items.push(typ);
            }

            Yaml::DocumentMeta(t, _) => match AssetType::try_from(t) {
                Ok(s) => {
                    asset_type = s.to_string();
                }

                Err(_) => {
                    panic!("Could not convert a value to asset type");
                }
            },

            _ => {}
        }
    }

    items
}

fn main() {
    // Tell Cargo to rerun this build script, if the file changes.
    let paths = std::fs::read_dir(get_templates_path()).unwrap();
    let mut files: Vec<String> = Vec::new();

    for path in paths.flatten() {
        let f = handle_yaml_file(&path.path());
        let file_name = path.path().display().to_string();
        let mut cmd = String::from("cargo:rerun-if-changed=");
        cmd.push_str(&file_name);
        cmd.push('\n');
        // log(&cmd);

        println!("{}", cmd);
        files.extend(f);
    }

    let mut mod_content = String::new();
    mod_content.push_str("/// Default modules:\n");
    mod_content.push_str("mod sections;\n");
    mod_content.push_str("mod asset_file;\n\n");

    mod_content.push_str("/// Generated modules:\n");
    for x in &files {
        mod_content.push_str(&format!("mod {};\n", x.to_lowercase()));
    }

    mod_content.push('\n');
    mod_content.push_str("pub mod prelude {\n");
    mod_content.push_str("\tpub use super::sections::Vec2;\n");
    mod_content.push_str("\tpub use super::sections::Vec3;\n");
    mod_content.push_str("\tpub use super::sections::Vec4;\n");
    mod_content.push_str("\tpub use super::sections::Color;\n");
    mod_content.push_str("\tpub use super::sections::Empty;\n");
    mod_content.push_str("\tpub use super::sections::FileReference;\n");
    mod_content.push_str("\tpub use super::sections::FalloffTable;\n");
    mod_content.push_str("\tpub use super::asset_file::AssetFile;\n");
    mod_content.push_str("\tpub use super::sections::BuildTarget;\n");

    mod_content.push('\n');
    for x in &files {
        mod_content.push_str(&format!("\tpub use super::{}::{};\n", x.to_lowercase(), x));
    }

    mod_content.push_str("}\n");

    write_file(&mod_content, "mod");
}
