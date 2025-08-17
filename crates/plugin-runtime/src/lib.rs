mod plugin_functions;

use std::{path::PathBuf, sync::Arc, time::Duration};

use jilebi_types::permissions::JilebiPermissions;
use rustyscript::{Error, Module, Runtime, RuntimeOptions, json_args};
use serde::de::DeserializeOwned;
use serde_json::Value;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt};

pub fn run_code<T>(
    plugin_name: &String,
    code: &String,
    function: &String,
    args: Value,
    env: Value,
    permissions: &Option<JilebiPermissions>,
    log_path: &PathBuf,
) -> Result<T, Error>
where
    T: DeserializeOwned + Clone,
{
    let file_appender = tracing_appender::rolling::never(log_path, plugin_name.clone() + ".logs");
    let (non_blocking_log_writer, _guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = tracing_subscriber::registry()
        .with(fmt::layer().pretty().with_writer(non_blocking_log_writer))
        .with(EnvFilter::new("dosa::plugin_functions::logging=trace"));

    let _logguard = tracing::subscriber::set_default(subscriber);

    let logging = plugin_functions::logging::logging::init_ops_and_esm();
    let state_management = plugin_functions::state::state::init_ops_and_esm();
    let permissions = permissions.clone().map(Arc::new);
    let module = Module::new("script.js", code);
    let mut runtime_options = RuntimeOptions {
        timeout: Duration::from_millis(50),
        extensions: vec![logging, state_management],
        ..Default::default()
    };
    if let Some(permissions) = permissions {
        runtime_options.extension_options.web.permissions = permissions.clone();
    }
    let mut runtime = Runtime::new(runtime_options)?;
    let handle = runtime.tokio_runtime();
    let module_handle = runtime.load_module(&module)?;
    handle.block_on(async {
        runtime
            .call_function_async::<T>(Some(&module_handle), &function, json_args!(args, env))
            .await
    })
}
