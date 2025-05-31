use std::{fs, path::Path};
mod server;
// use dosa::run_code;
use jilebi_types::plugin::Manifest;
use rmcp::{ServiceExt, transport::stdio};
use server::JilebiMcpServer;

#[tokio::main]
async fn main() -> Result<(), String> {
    let toml_file_path = Path::new("/home/kartik/jilebi/examples/ts-simple-computer-use/manifest.toml");
    let manifest = fs::read_to_string(toml_file_path).expect("Manifest file not found");
    let manifest = toml::from_str::<Manifest>(&manifest).expect("Could not parse toml file");

    // println!("{:#?}", manifest);
    let mut server = JilebiMcpServer::new();
    server.add(manifest).await;

    let service = server.serve(stdio()).await.map_err(|e| {
        tracing::error!("Serving error: {:?}", e);
        e.to_string()
    })?;
    service.waiting().await.map_err(|e| e.to_string())?;
    Ok(())
    // let code =
    //     fs::read_to_string("examples/ts-simple-computer-use/main.js").expect("File not found");
    // let function = manifest.resources.get("ls").unwrap();
    // match run_code(&code, &function.function) {
    //     Ok(result) => println!("Function output = {}", result),
    //     Err(err) => println!("Function failed with error {}", err),
    // }
}
