mod plugin_functions;

use std::time::Duration;

use rustyscript::{Error, Module, Runtime, RuntimeOptions};
use serde::de::DeserializeOwned;
use serde_json::Value;
use tracing_subscriber::{fmt, layer::SubscriberExt};

pub fn run_code<T>(
    plugin_name: &String,
    code: &String,
    function: &String,
    args: Value,
) -> Result<T, Error>
where
    T: DeserializeOwned + Clone,
{
    let file_appender = tracing_appender::rolling::never(
        dotenvy::var("LOG_PATH").unwrap_or("./logs".into()),
        plugin_name.clone() + ".logs",
    );
    let (non_blocking_log_writer, _guard) = tracing_appender::non_blocking(file_appender);
    let subscriber =
        tracing_subscriber::registry().with(fmt::layer().with_writer(non_blocking_log_writer));

    let _logguard = tracing::subscriber::set_default(subscriber);

    let logging = plugin_functions::logging::logging::init_ops_and_esm();
    let module = Module::new("script.js", code);
    let mut runtime = Runtime::new(RuntimeOptions {
        timeout: Duration::from_millis(50),
        extensions: vec![logging],
        ..Default::default()
    })?;
    let handle = runtime.tokio_runtime();
    let module_handle = runtime.load_module(&module)?;
    handle.block_on(async {
        runtime
            .call_function_async::<T>(Some(&module_handle), &function, &args)
            .await
    })
}
