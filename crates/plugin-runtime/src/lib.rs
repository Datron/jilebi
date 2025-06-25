use std::time::Duration;

use rustyscript::{Error, Module, Runtime, RuntimeOptions};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub fn run_code<T>(code: &String, function: &String, args: Value) -> Result<T, Error>
where
    T: DeserializeOwned + Clone,
{
    let module = Module::new("script.js", code);
    let mut runtime = Runtime::new(RuntimeOptions {
        timeout: Duration::from_millis(50),
        ..Default::default()
    })?;
    let module_handle = runtime.load_module(&module)?;
    let fn_output = runtime.call_function::<T>(Some(&module_handle), &function, &args)?;
    Ok(fn_output)
}
