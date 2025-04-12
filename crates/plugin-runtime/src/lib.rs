use std::time::Duration;

use rustyscript::{Error, Module, Runtime, RuntimeOptions, json_args};

pub fn run_code(code: String) -> Result<String, Error> {
    let module = Module::new("script.js", code);
    let mut runtime = Runtime::new(RuntimeOptions {
        timeout: Duration::from_millis(50),
        ..Default::default()
    })?;
    let module_handle = runtime.load_module(&module)?;
    let fn_output = runtime.call_function::<String>(
        Some(&module_handle),
        "main",
        json_args!("kartik"),
    )?;
    Ok(fn_output)
}
