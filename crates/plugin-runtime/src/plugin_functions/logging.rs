use deno_core::{extension, op2};

#[op2(fast)]
fn op_trace(#[string] msg: &str) {
    tracing::trace!(msg);
}

#[op2(fast)]
fn op_info(#[string] msg: &str) {
    tracing::info!(msg);
}

#[op2(fast)]
fn op_debug(#[string] msg: &str) {
    tracing::debug!(msg);
}

#[op2(fast)]
fn op_warn(#[string] msg: &str) {
    tracing::warn!(msg);
}

#[op2(fast)]
fn op_error(#[string] msg: &str) {
    tracing::error!(msg);
}

extension!(
    logging,
    ops = [op_trace, op_debug, op_error, op_info, op_warn],
    esm_entry_point = "ext:logging/logging.js",
    esm = [ dir "extensions", "logging.js" ],
);
