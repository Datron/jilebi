use std::{fs, path::Path};

use dosa::run_code;
use jilebi_types::plugin::Manifest;

fn main() {
    let code = r#"
	export function main(name) {
		console.log("main says = ", name);
		return "Hello dosa!";
	}
	"#;

    let toml_file_path = Path::new("examples/ts-simple-computer-use/manifest.toml");
    let manifest = fs::read_to_string(toml_file_path).expect("Manifest file not found");
    let manifest = toml::from_str::<Manifest>(&manifest).expect("Could not parse toml file");

    println!("{:#?}", manifest);

    match run_code(String::from(code)) {
        Ok(result) => println!("Function output = {}", result),
        Err(err) => println!("Function failed with error {}", err),
    }
}
