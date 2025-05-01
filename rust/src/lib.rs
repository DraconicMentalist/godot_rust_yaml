
use godot::classes::Json;
use godot::prelude::*;
use godot::obj::cap::GodotDefault;
use godot::builtin::GString;
struct GodotRustYAML;
#[gdextension]
unsafe impl ExtensionLibrary for GodotRustYAML {}

#[derive(GodotClass)]
struct YAML;
impl GodotDefault for YAML {}

#[godot_api]
impl YAML {
    #[func]
    fn parse(str: GString) -> Variant {
        let input: String = str.to_string();
        let yaml_parse_result: Result<serde_json::Value, serde_yaml::Error> = serde_yaml::from_str(&input.replace("	", "   "));
        let json_value = match yaml_parse_result {
            Ok(parsed) => parsed,
            Err(error) => return Variant::from(format!("Problem parsing the YAML: {:?}", error)),
        };
        let json_string = serde_json::to_string(&json_value).unwrap();
        return Json::parse_string(&GString::from(json_string));
    }

    #[func]
    fn to_string(input: Dictionary) -> GString {
        let input_string = &Variant::from(input);
        let json_string = Json::stringify(input_string);
        let input: String = json_string.to_string();
        let json_parse_result: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&input);
        let yaml_value = match json_parse_result {
            Ok(parsed) => parsed,
            Err(error) => return GString::from(format!("Problem parsing the dictionary: {:?}", error)),
        };
        let yaml_string = serde_yaml::to_string(&yaml_value).unwrap();
        return GString::from(yaml_string);
    }
}
