use std::{fs, path::Path};

use dosa::run_code;
use jilebi_types::plugin::Manifest;

fn main() {
    let toml_file_path = Path::new("examples/ts-simple-computer-use/manifest.toml");
    let manifest = fs::read_to_string(toml_file_path).expect("Manifest file not found");
    let manifest = toml::from_str::<Manifest>(&manifest).expect("Could not parse toml file");

    println!("{:#?}", manifest);
    let code =
        fs::read_to_string("examples/ts-simple-computer-use/main.js").expect("File not found");
    let function = manifest.resources.get("ls").unwrap();
    match run_code(&code, &function.function) {
        Ok(result) => println!("Function output = {}", result),
        Err(err) => println!("Function failed with error {}", err),
    }
}
