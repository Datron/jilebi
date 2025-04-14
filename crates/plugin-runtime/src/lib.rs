use std::time::Duration;

use rustyscript::{Error, Module, Runtime, RuntimeOptions, json_args};

pub fn run_code(code: &String, function: &String) -> Result<serde_json::Value, Error> {
    let module = Module::new("script.js", code);
    let mut runtime = Runtime::new(RuntimeOptions {
        timeout: Duration::from_millis(50),
        ..Default::default()
    })?;
    let module_handle = runtime.load_module(&module)?;
    let fn_output = runtime.call_function::<serde_json::Value>(
        Some(&module_handle),
        &function,
        json_args!("kartik", "asa"),
    )?;
    Ok(fn_output)
}
